use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

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

pub fn get_html_template() -> Result<String, String> {
    // 使用 include_str! 宏在编译时嵌入模板文件
    // 路径相对于 crate 根目录 (src-tauri)
    const TEMPLATE_CONTENT: &str = include_str!("../../templates/upload.html");
    Ok(TEMPLATE_CONTENT.to_string())
}
