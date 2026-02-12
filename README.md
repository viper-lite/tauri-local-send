# LocalSend 📡

> 零配置、跨平台的局域网文件传输神器

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri&logoColor=white" alt="Tauri">
  <img src="https://img.shields.io/badge/React-19-61DAFB?logo=react&logoColor=white" alt="React">
  <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License">
</p>

<p align="center">
  <b>无需注册、无需网络、扫码即传</b>
</p>

<p align="center">
  <a href="#-下载">⬇️ 立即下载</a> •
  <a href="#-快速开始">🚀 快速开始</a> •
  <a href="#-功能特性">✨ 功能特性</a>
</p>

---

## 📸 预览

<p align="center">
  <img src="./screenshots/app-preview.png" alt="LocalSend Preview" width="600">
</p>

*现代化的界面设计，简洁直观的操作体验*

---

## ✨ 功能特性

### 🚀 零配置启动
- 一键启动本地服务器
- 自动生成访问地址和二维码
- 手机扫码即可上传文件

### 📱 全平台支持
- **Windows** - 原生安装包 (.msi)
- **macOS** - 支持 Intel 和 Apple Silicon (.dmg)
- **Linux** - AppImage 和 deb 包

### 🎨 精美设计
- 清新淡雅的青色渐变主题
- 扁平化设计风格
- 流畅的交互动效

### 🔒 安全私密
- 纯局域网传输，数据不经过云端
- 无需注册账号
- 自动保存到指定目录

### 📊 实时状态
- 查看已接收文件数量
- 实时显示传输总大小
- 一键复制访问地址

---

## 📥 下载

| 平台 | 下载链接 | 说明 |
|------|---------|------|
| Windows | [LocalSend-1.0.0.msi](https://github.com/viper-lite/tauri-local-send/releases/latest) | Windows 10/11 |
| macOS (Intel) | [LocalSend-1.0.0-x64.dmg](https://github.com/viper-lite/tauri-local-send/releases/latest) | Intel Mac |
| macOS (Apple Silicon) | [LocalSend-1.0.0-aarch64.dmg](https://github.com/viper-lite/tauri-local-send/releases/latest) | M1/M2/M3 Mac |
| Linux | [LocalSend-1.0.0.AppImage](https://github.com/viper-lite/tauri-local-send/releases/latest) | 通用Linux |
| Linux | [LocalSend-1.0.0.deb](https://github.com/viper-lite/tauri-local-send/releases/latest) | Debian/Ubuntu |

📦 [查看所有版本](https://github.com/viper-lite/tauri-local-send/releases)

---

## 🚀 快速开始

### 1. 下载并安装
从上方链接下载适合你系统的安装包，双击安装。

### 2. 启动应用
打开 LocalSend，点击「启动局域网服务」按钮。

### 3. 扫码传输
- 手机扫描屏幕上显示的二维码
- 或在手机浏览器输入显示的 IP 地址
- 选择文件，一键上传！

### 4. 查看文件
上传的文件会自动保存到电脑的 `Downloads/LocalSend` 目录。

---

## 🛠️ 技术栈

- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Tauri 2.0 + Rust
- **UI Design**: 现代化扁平设计，青色渐变主题
- **Build**: GitHub Actions 自动构建发布

---

## 🏗️ 从源码构建

```bash
# 克隆仓库
git clone https://github.com/viper-lite/tauri-local-send.git
cd tauri-local-send

# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 系统要求

**Windows:**
- Windows 10 或更高版本
- WebView2 运行时

**macOS:**
- macOS 10.13 或更高版本
- Xcode Command Line Tools (构建时需要)

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev

# Fedora
sudo dnf install gtk3-devel webkit2gtk4.1-devel javascriptcoregtk4.1-devel

# Arch
sudo pacman -S gtk3 webkit2gtk-4.1
```

---

## 📋 使用场景

- 💼 **办公室** - 快速传输文件给同事
- 🏠 **家庭网络** - 手机照片备份到电脑
- 🎓 **教室/会议室** - 无需U盘，一键分享
- 🔒 **隐私敏感** - 拒绝云服务，保持数据本地

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

---

## 📜 许可证

本项目采用 [MIT](LICENSE) 许可证开源。

---

## 💖 支持

如果这个项目对你有帮助，请给个 ⭐️ Star！

<p align="center">
  <a href="https://github.com/viper-lite/tauri-local-send">
    <img src="https://img.shields.io/github/stars/viper-lite/tauri-local-send?style=social" alt="GitHub Stars">
  </a>
</p>

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/viper-lite">@viper-lite</a>
</p>
