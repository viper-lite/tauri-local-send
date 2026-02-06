mod network;
mod qrcode;
mod server;

use std::sync::Mutex;
use tauri::State;
use warp::{Filter, Buf};
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
struct CustomError(#[allow(dead_code)] String);

impl warp::reject::Reject for CustomError {}

struct ServerState(Mutex<Option<tokio::task::JoinHandle<()>>>);

#[tauri::command]
async fn get_local_ip() -> Result<String, String> {
    network::get_local_ip().ok_or("无法获取本机IP地址".to_string())
}

#[tauri::command]
async fn start_server(state: State<'_, ServerState>) -> Result<serde_json::Value, String> {
    let port = network::get_free_port();
    let upload_dir = server::get_upload_dir();
    
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| format!("Failed to create upload directory: {}", e))?;

    let ip = network::get_local_ip().unwrap_or_else(|| "0.0.0.0".to_string());
    let access_url = format!("http://{}:{}", ip, port);

    let qr_data_url = qrcode::qr_to_data_url(&access_url)
        .map_err(|e| format!("生成二维码失败: {}", e))?;

    let routes = warp::path::end()
        .map(|| warp::reply::html(server::get_html_template()));

    let upload_dir_clone = upload_dir.clone();
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(500_000_000))
        .and(warp::any().map(move || upload_dir_clone.clone()))
        .and_then(handle_upload);

    let routes = routes.or(upload_route);

    let (_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], port), async move {
            tokio::signal::ctrl_c().await.ok();
        });

    let join_handle = tokio::spawn(server);

    {
        let mut guard = state.0.lock().unwrap();
        *guard = Some(join_handle);
    }

    Ok(serde_json::json!({
        "success": true,
        "url": access_url,
        "qrCode": qr_data_url,
        "ip": ip,
        "port": port
    }))
}

async fn handle_upload(
    mut form: warp::multipart::FormData,
    upload_dir: std::path::PathBuf,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut parts = Vec::new();
    while let Some(part) = form.next().await {
        parts.push(part.map_err(|e| warp::reject::custom(CustomError(e.to_string())))?);
    }

    let mut uploaded_files = Vec::new();

    for part in parts {
        let filename = part.filename().unwrap_or("unknown").to_string();
        let sanitized: String = filename
            .chars()
            .filter(|&c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
            .collect();
        
        let mut filepath = upload_dir.join(&sanitized);
        if filepath.exists() {
            let ext = std::path::Path::new(&sanitized).extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();
            let stem = std::path::Path::new(&sanitized).file_stem()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or("file".to_string());
            let mut counter = 1;
            loop {
                let new_name = if ext.is_empty() {
                    format!("{}_{}", stem, counter)
                } else {
                    format!("{}_{}.{}", stem, counter, ext)
                };
                filepath = upload_dir.join(&new_name);
                if !filepath.exists() { break; }
                counter += 1;
            }
        }

        let mut file = tokio::fs::File::create(&filepath)
            .await
            .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
        let mut stream = part.stream();
        let mut bytes_written = 0usize;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
            let chunk_bytes = chunk.chunk();
            file.write_all(chunk_bytes)
                .await
                .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
            bytes_written += chunk_bytes.len();
        }

        file.flush()
            .await
            .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
        uploaded_files.push(sanitized);
        server::RECEIVED_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        server::TOTAL_SIZE.fetch_add(bytes_written, std::sync::atomic::Ordering::SeqCst);
    }

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "message": format!("成功上传 {} 个文件", uploaded_files.len()),
        "files": uploaded_files
    })))
}

#[tauri::command]
async fn stop_server(state: State<'_, ServerState>) -> Result<(), String> {
    let mut guard = state.0.lock().unwrap();
    if let Some(handle) = guard.take() {
        handle.abort();
    }
    Ok(())
}

#[tauri::command]
async fn get_server_status() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "received_count": server::get_received_count(),
        "total_size": server::get_total_size(),
        "upload_dir": server::get_upload_dir().to_string_lossy().to_string()
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_local_ip,
            start_server,
            stop_server,
            get_server_status
        ])
        .manage(ServerState(Mutex::new(None)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
