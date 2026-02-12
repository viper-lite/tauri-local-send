import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface ServerInfo {
  success: boolean;
  url?: string;
  qrCode?: string;
  ip?: string;
  port?: number;
  error?: string;
}

interface FileStatus {
  received_count: number;
  total_size: number;
  upload_dir: string;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  return (bytes / (1024 * 1024 * 1024)).toFixed(2) + " GB";
}

const DocumentIcon = () => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
    <polyline points="14 2 14 8 20 8"/>
    <line x1="16" y1="13" x2="8" y2="13"/>
    <line x1="16" y1="17" x2="8" y2="17"/>
    <polyline points="10 9 9 9 8 9"/>
  </svg>
);

const SmartphoneIcon = () => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <rect x="5" y="2" width="14" height="20" rx="2" ry="2"/>
    <line x1="12" y1="18" x2="12.01" y2="18"/>
  </svg>
);

const StopIcon = () => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
  </svg>
);

const PlayIcon = () => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <polygon points="5 3 19 12 5 21 5 3"/>
  </svg>
);

function App() {
  const [serverInfo, setServerInfo] = useState<ServerInfo | null>(null);
  const [loading, setLoading] = useState(false);
  const [fileStatus, setFileStatus] = useState<FileStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    startServer();
  }, []);

  useEffect(() => {
    if (serverInfo?.success) {
      const interval = setInterval(fetchFileStatus, 2000);
      return () => clearInterval(interval);
    }
  }, [serverInfo]);

  const startServer = async () => {
    setLoading(true);
    setError(null);
    try {
      const info = await invoke<ServerInfo>("start_server");
      if (info.success) {
        setServerInfo(info);
      } else {
        setError(info.error || "Failed to start server");
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const stopServer = async () => {
    try {
      await invoke("stop_server");
      setServerInfo(null);
    } catch (e) {
      console.error("Failed to stop server:", e);
    }
  };

  const fetchFileStatus = async () => {
    try {
      const status = await invoke<FileStatus>("get_server_status");
      setFileStatus(status);
    } catch (e) {
      console.error("Failed to fetch status:", e);
    }
  };

  const copyToClipboard = () => {
    if (serverInfo?.url) {
      navigator.clipboard.writeText(serverInfo.url);
    }
  };

  return (
    <div className="app">
      <header className="header">
        <div className="logo-badge">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
            <path d="M12 2v6M12 22v-6M4.93 4.93l4.24 4.24M14.83 14.83l4.24 4.24M2 12h6M22 12h-6M4.93 19.07l4.24-4.24M14.83 9.17l4.24-4.24"/>
          </svg>
        </div>
        <h1>LocalSend</h1>
        <p className="subtitle">扫码即可上传文件到这台电脑</p>
      </header>

      {error && (
        <div className="error-message">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="8" x2="12" y2="12"/>
            <line x1="12" y1="16" x2="12.01" y2="16"/>
          </svg>
          {error}
        </div>
      )}

      {loading && (
        <div className="loading">
          <div className="spinner"></div>
          <p style={{ color: '#94a3b8' }}>正在启动服务器...</p>
        </div>
      )}

      {serverInfo?.success && (
        <div className="content">
          <div className="qr-section">
            <div className="qr-container">
              <img
                src={serverInfo.qrCode}
                alt="QR Code"
                className="qr-code"
              />
            </div>
            <p className="qr-hint">
              <SmartphoneIcon />
              使用手机扫描二维码
            </p>
          </div>

          <div className="info-section">
            <div className="info-card">
              <div className="info-row">
                <span className="info-label">本机地址</span>
                <span className="info-value copyable" onClick={copyToClipboard}>
                  {serverInfo.url}
                  <span className="copy-hint">点击复制</span>
                </span>
              </div>
              <div className="info-row">
                <span className="info-label">IP地址</span>
                <span className="info-value">{serverInfo.ip}:{serverInfo.port}</span>
              </div>
            </div>

            <div className="status-card">
              <h3>
                <DocumentIcon />
                已接收文件
              </h3>
              <div className="stats">
                <div className="stat">
                  <span className="stat-value">{fileStatus?.received_count || 0}</span>
                  <span className="stat-label">个文件</span>
                </div>
                <div className="stat">
                  <span className="stat-value">{formatSize(fileStatus?.total_size || 0)}</span>
                  <span className="stat-label">总计</span>
                </div>
              </div>
              <p className="save-path">
                保存位置: {fileStatus?.upload_dir || "Downloads/LocalSend"}
              </p>
            </div>
          </div>

          <button className="stop-btn" onClick={stopServer}>
            <StopIcon />
            停止服务器
          </button>
        </div>
      )}

      {!serverInfo?.success && !loading && !error && (
        <div className="start-prompt">
          <p>点击下方按钮启动服务</p>
          <button className="start-btn" onClick={startServer}>
            <PlayIcon />
            启动局域网服务
          </button>
        </div>
      )}
    </div>
  );
}

export default App;
