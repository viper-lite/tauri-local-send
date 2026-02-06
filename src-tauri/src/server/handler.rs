use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::fs::{File, create_dir_all};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use warp::multipart::FormData;
use std::fs;

static RECEIVED_COUNT: AtomicUsize = AtomicUsize::new(0);
static TOTAL_SIZE: AtomicUsize = AtomicUsize::new(0);

pub fn get_received_count() -> usize {
    RECEIVED_COUNT.load(Ordering::SeqCst)
}

pub fn get_total_size() -> usize {
    TOTAL_SIZE.load(Ordering::SeqCst)
}

pub fn get_upload_dir() -> PathBuf {
    dirs::download_dir()
        .unwrap_or_else(|| PathBuf::from(std::env::temp_dir()))
        .join("LocalSend")
}

pub async fn handle_upload(form: FormData, upload_dir: PathBuf) -> Result<impl warp::Reply, warp::Rejection> {
    create_dir_all(&upload_dir).await.map_err(|e| {
        warp::reject::custom(Error(format!("Failed to create upload directory: {}", e)))
    })?;

    let parts = form
        .filter_map(|part| async move {
            match part {
                warp::multipart::Part::File(file) => Some(file),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
        .await;

    let mut uploaded_files = Vec::new();
    let mut total_bytes = 0usize;

    for part in parts {
        let filename = get_filename(&part).await?;
        let (filepath, bytes_written) = save_file(part, &upload_dir, &filename).await?;
        uploaded_files.push(filename);
        total_bytes += bytes_written;

        RECEIVED_COUNT.fetch_add(1, Ordering::SeqCst);
        TOTAL_SIZE.fetch_add(bytes_written, Ordering::SeqCst);
    }

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "message": format!("成功上传 {} 个文件", uploaded_files.len()),
        "files": uploaded_files
    })))
}

async fn get_filename(part: &warp::multipart::PartData) -> String {
    let name = part.name().to_string();
    let filename = part.filename()
        .map(|s| sanitize_filename(s))
        .unwrap_or_else(|| format!("file_{}", Uuid::new_v4()));

    format!("{}_{}", name, filename)
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
        .collect()
}

async fn save_file(
    mut part: warp::multipart::Part,
    upload_dir: &PathBuf,
    original_name: &str,
) -> Result<(PathBuf, usize), warp::Rejection> {
    let upload_dir = upload_dir.clone();
    let mut filepath = upload_dir.join(original_name);

    if filepath.exists() {
        let stem = original_name.rsplitn(2, '.').last().unwrap_or(original_name);
        let ext = original_name.split('.').last().map(|s| format!(".{}", s)).unwrap_or_default();
        let mut counter = 1;
        loop {
            let new_name = format!("{}_{}{}", stem, counter, ext);
            filepath = upload_dir.join(&new_name);
            if !filepath.exists() {
                break;
            }
            counter += 1;
        }
    }

    let mut file = File::create(&filepath).await.map_err(|e| {
        warp::reject::custom(Error(format!("Failed to create file: {}", e)))
    })?;

    let mut bytes_written = 0usize;
    while let Some(chunk) = part.data().await {
        let chunk = chunk.map_err(|e| {
            warp::reject::custom(Error(format!("Read chunk error: {}", e)))
        })?;
        file.write_all(&chunk).await.map_err(|e| {
            warp::reject::custom(Error(format!("Write chunk error: {}", e)))
        })?;
        bytes_written += chunk.len();
    }

    file.flush().await.map_err(|e| {
        warp::reject::custom(Error(format!("Flush error: {}", e)))
    })?;

    Ok((filepath, bytes_written))
}

#[derive(Debug)]
struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl warp::reject::Reject for Error {}

pub fn get_html_template() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>上传文件 - LocalSend</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 20px;
        }
        .container {
            background: white;
            border-radius: 16px;
            padding: 30px;
            width: 100%;
            max-width: 400px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
        }
        h1 {
            color: #333;
            font-size: 24px;
            text-align: center;
            margin-bottom: 10px;
        }
        .subtitle {
            color: #666;
            font-size: 14px;
            text-align: center;
            margin-bottom: 24px;
        }
        .upload-area {
            border: 3px dashed #667eea;
            border-radius: 12px;
            padding: 30px 20px;
            text-align: center;
            margin-bottom: 20px;
            background: #f8f9ff;
            transition: all 0.3s;
            cursor: pointer;
        }
        .upload-area:hover {
            border-color: #764ba2;
            background: #f0f2ff;
        }
        .upload-area.dragover {
            border-color: #764ba2;
            background: #e8ebff;
        }
        .icon {
            font-size: 48px;
            margin-bottom: 10px;
        }
        .upload-text {
            color: #667eea;
            font-size: 16px;
            font-weight: 500;
        }
        .file-info {
            color: #666;
            font-size: 14px;
            margin-top: 10px;
            word-break: break-all;
        }
        .btn {
            width: 100%;
            padding: 16px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
        }
        .btn:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
        }
        .progress-container {
            margin: 20px 0;
            display: none;
        }
        .progress-container.show {
            display: block;
        }
        .progress-bar {
            height: 8px;
            background: #e5e5e5;
            border-radius: 4px;
            overflow: hidden;
        }
        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, #667eea, #764ba2);
            border-radius: 4px;
            transition: width 0.3s ease;
        }
        .progress-text {
            text-align: center;
            margin-top: 8px;
            color: #666;
            font-size: 14px;
        }
        .status {
            text-align: center;
            margin-top: 16px;
            padding: 12px;
            border-radius: 8px;
            font-size: 14px;
        }
        .status.success {
            background: #d4edda;
            color: #155724;
        }
        .status.error {
            background: #f8d7da;
            color: #721c24;
        }
        .status.uploading {
            background: #fff3cd;
            color: #856404;
        }
        input[type="file"] {
            display: none;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📁 上传文件</h1>
        <p class="subtitle">将文件发送到这台电脑</p>
        
        <div class="upload-area" id="uploadArea" onclick="document.getElementById('fileInput').click()">
            <div class="icon">📤</div>
            <div class="upload-text">点击选择文件</div>
            <div class="file-info" id="fileInfo">支持多个文件</div>
        </div>
        
        <input type="file" id="fileInput" multiple>
        
        <button class="btn" id="uploadBtn" onclick="uploadFiles()" disabled>
            开始上传
        </button>
        
        <div class="progress-container" id="progressContainer">
            <div class="progress-bar">
                <div class="progress-fill" id="progressFill" style="width: 0%"></div>
            </div>
            <div class="progress-text" id="progressText">0%</div>
        </div>
        
        <div class="status" id="status" style="display: none"></div>
    </div>

    <script>
        let selectedFiles = [];
        
        document.getElementById('fileInput').addEventListener('change', function(e) {
            selectedFiles = Array.from(e.target.files);
            updateFileInfo();
        });
        
        const uploadArea = document.getElementById('uploadArea');
        
        ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
            uploadArea.addEventListener(eventName, preventDefaults, false);
        });
        
        function preventDefaults(e) {
            e.preventDefault();
            e.stopPropagation();
        }
        
        ['dragenter', 'dragover'].forEach(eventName => {
            uploadArea.addEventListener(eventName, () => {
                uploadArea.classList.add('dragover');
            });
        });
        
        ['dragleave', 'drop'].forEach(eventName => {
            uploadArea.addEventListener(eventName => {
                uploadArea.classList.remove('dragover');
            });
        });
        
        uploadArea.addEventListener('drop', function(e) {
            const files = e.dataTransfer.files;
            selectedFiles = Array.from(files);
            updateFileInfo();
        });
        
        function updateFileInfo() {
            const fileInfo = document.getElementById('fileInfo');
            const uploadBtn = document.getElementById('uploadBtn');
            
            if (selectedFiles.length === 0) {
                fileInfo.textContent = '支持多个文件';
                uploadBtn.disabled = true;
                return;
            }
            
            const totalSize = selectedFiles.reduce((sum, f) => sum + f.size, 0);
            const sizeStr = formatSize(totalSize);
            fileInfo.textContent = `${selectedFiles.length} 个文件 (${sizeStr})`;
            uploadBtn.disabled = false;
        }
        
        function formatSize(bytes) {
            if (bytes < 1024) return bytes + ' B';
            if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
            if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
            return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
        }
        
        async function uploadFiles() {
            if (selectedFiles.length === 0) return;
            
            const uploadBtn = document.getElementById('uploadBtn');
            const progressContainer = document.getElementById('progressContainer');
            const progressFill = document.getElementById('progressFill');
            const progressText = document.getElementById('progressText');
            const status = document.getElementById('status');
            
            uploadBtn.disabled = true;
            progressContainer.classList.add('show');
            status.style.display = 'none';
            
            const formData = new FormData();
            selectedFiles.forEach(file => {
                formData.append('files', file);
            });
            
            try {
                const xhr = new XMLHttpRequest();
                
                xhr.upload.addEventListener('progress', function(e) {
                    if (e.lengthComputable) {
                        const percent = Math.round((e.loaded / e.total) * 100);
                        progressFill.style.width = percent + '%';
                        progressText.textContent = percent + '%';
                    }
                });
                
                xhr.addEventListener('load', function() {
                    if (xhr.status === 200) {
                        const response = JSON.parse(xhr.responseText);
                        status.textContent = '✓ ' + response.message;
                        status.className = 'status success';
                        status.style.display = 'block';
                        selectedFiles = [];
                        updateFileInfo();
                        setTimeout(() => {
                            progressContainer.classList.remove('show');
                        }, 2000);
                    } else {
                        throw new Error('上传失败');
                    }
                });
                
                xhr.addEventListener('error', function() {
                    throw new Error('网络错误');
                });
                
                xhr.open('POST', '/upload');
                xhr.send(formData);
                
            } catch (error) {
                status.textContent = '✗ ' + error.message;
                status.className = 'status error';
                status.style.display = 'block';
                uploadBtn.disabled = false;
            }
        }
    </script>
</body>
</html>"#
}
