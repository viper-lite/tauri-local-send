use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use warp::Filter;

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
        h1 { color: #333; font-size: 24px; text-align: center; margin-bottom: 10px; }
        .subtitle { color: #666; font-size: 14px; text-align: center; margin-bottom: 24px; }
        .upload-area {
            border: 3px dashed #667eea;
            border-radius: 12px;
            padding: 30px 20px;
            text-align: center;
            margin-bottom: 20px;
            background: #f8f9ff;
            cursor: pointer;
        }
        .upload-area:hover { border-color: #764ba2; background: #f0f2ff; }
        .icon { font-size: 48px; margin-bottom: 10px; }
        .upload-text { color: #667eea; font-size: 16px; font-weight: 500; }
        .file-info { color: #666; font-size: 14px; margin-top: 10px; }
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
        }
        .btn:disabled { opacity: 0.6; cursor: not-allowed; }
        .progress-container { margin: 20px 0; display: none; }
        .progress-container.show { display: block; }
        .progress-bar { height: 8px; background: #e5e5e5; border-radius: 4px; overflow: hidden; }
        .progress-fill { height: 100%; background: linear-gradient(90deg, #667eea, #764ba2); transition: width 0.3s; }
        .progress-text { text-align: center; margin-top: 8px; color: #666; font-size: 14px; }
        .status { text-align: center; margin-top: 16px; padding: 12px; border-radius: 8px; font-size: 14px; display: none; }
        .status.success { background: #d4edda; color: #155724; }
        .status.error { background: #f8d7da; color: #721c24; }
        input[type="file"] { display: none; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📁 上传文件</h1>
        <p class="subtitle">将文件发送到这台电脑</p>
        <div class="upload-area" onclick="document.getElementById('fileInput').click()">
            <div class="icon">📤</div>
            <div class="upload-text">点击选择文件</div>
            <div class="file-info" id="fileInfo">支持多个文件</div>
        </div>
        <input type="file" id="fileInput" multiple>
        <button class="btn" id="uploadBtn" onclick="uploadFiles()" disabled>开始上传</button>
        <div class="progress-container" id="progressContainer">
            <div class="progress-bar"><div class="progress-fill" id="progressFill" style="width: 0%"></div></div>
            <div class="progress-text" id="progressText">0%</div>
        </div>
        <div class="status" id="status"></div>
    </div>
    <script>
        let selectedFiles = [];
        document.getElementById('fileInput').addEventListener('change', function(e) {
            selectedFiles = Array.from(e.target.files);
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
            const sizeStr = totalSize < 1024*1024 ? (totalSize/1024).toFixed(1)+' KB' : (totalSize/(1024*1024)).toFixed(1)+' MB';
            fileInfo.textContent = selectedFiles.length + ' 个文件 (' + sizeStr + ')';
            uploadBtn.disabled = false;
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
            selectedFiles.forEach(file => formData.append('files', file));
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
                        status.textContent = '✓ 上传成功';
                        status.className = 'status success';
                        status.style.display = 'block';
                        selectedFiles = [];
                        updateFileInfo();
                        setTimeout(() => progressContainer.classList.remove('show'), 2000);
                    } else { throw new Error('上传失败'); }
                });
                xhr.addEventListener('error', function() { throw new Error('网络错误'); });
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

pub fn with_state(
    upload_dir: PathBuf,
) -> impl Filter<Extract = (PathBuf,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || upload_dir.clone())
}
