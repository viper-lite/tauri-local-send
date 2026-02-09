use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::fs::read_to_string;

pub static RECEIVED_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static TOTAL_SIZE: AtomicUsize = AtomicUsize::new(0);

pub fn get_upload_dir() -> PathBuf {
    dirs::download_dir()
        .unwrap_or_else(|| PathBuf::from(std::env::temp_dir()))
        .join("LocalSend")
}

pub fn get_received_count() -> usize {
    RECEIVED_COUNT.load(Ordering::SeqCst)
}

pub fn get_total_size() -> usize {
    TOTAL_SIZE.load(Ordering::SeqCst)
}

pub async fn get_html_template() -> Result<String, String> {
    let template_path = PathBuf::from("templates/upload.html");
    
    // 尝试多个可能的路径
    let possible_paths = [
        template_path.clone(),
        PathBuf::from("src-tauri/templates/upload.html"),
        std::env::current_dir()
            .map(|d| d.join("templates/upload.html"))
            .unwrap_or(template_path.clone()),
        PathBuf::from("../Resources/templates/upload.html"),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("../Resources/templates/upload.html")))
            .unwrap_or_else(|| PathBuf::from("templates/upload.html")),
    ];
    
    for path in &possible_paths {
        if let Ok(content) = read_to_string(path).await {
            return Ok(content);
        }
    }
    
    // 如果文件读取失败，返回错误
    Err("无法读取模板文件 templates/upload.html".to_string())
}
