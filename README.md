# 📝 EasyNotes (Desktop)

一个极致轻量、极简风格的桌面端 Markdown 笔记应用。

## ✨ 特性

* **轻量至极**：基于 Tauri 2.0 驱动，安装包仅约 **9.6MB**，运行内存占用极低。
* **即开即用**：无需联网，数据完全存储在本地，保护隐私。
* **纯净体验**：无广告、无社交干扰，专注于写作本身。
* **自动持久化**：应用会自动记录并恢复你的最后编辑状态及设置，真正做到开箱即用。

## 💡 灵感来源
本项目的开发灵感源自一款优秀的移动端 Material Design 风格轻量笔记软件：**EasyNotes**。
- 原作者仓库：[Kin69/EasyNotes](https://github.com/Kin69/EasyNotes)
- 原项目发布页：[Releases](https://github.com/Kin69/EasyNotes/releases)


## 🚀 快速开始

### 下载安装

直接前往 [Releases 页面](https://github.com/KSCN547/easy-notes-desktop/releases) 下载最新的安装包。
- **Windows**: `.exe`
- **Linux (Debian/Ubuntu)**: `.deb`
- **Linux (Fedora/CentOS)**: `.rpm`
- **Linux (Universal)**: `.AppImage`

> **🔒 文件校验 (v0.1.0)**
> `SHA-256: 51a2a2b36f785bb94e2c7849f3317dfdb9af9336d0c98850548cbc4f27f18e62`

### 开发者模式

1. 确保已安装 [Rust](https://www.rust-lang.org/) 和 [Node.js](https://nodejs.org/)。
2. 克隆仓库：
```bash
git clone https://github.com/KSCN547/easy-notes-desktop.git

```


3. 安装依赖：
```bash
npm install

```


4. 启动开发环境：
```bash
npm run tauri dev

```



## 🛠️ 技术栈

* **Frontend**: Vue 3 + TypeScript + Vite
* **Backend**: Rust (Tauri 2.0)
* **Styling**: CSS (Minimalist design)

## 📅 开发计划 (Roadmap)

* [ ] 代码块语法高亮 (Syntax Highlighting)
* [ ] 数学公式渲染 (KaTeX/MathJax)
* [ ] 脚注 (Footnote) 支持
* [ ] 多文档管理系统
* [ ] 导出为 PDF / HTML

## 🛠️ 推荐 IDE 配置

如果你想参与开发，建议安装以下扩展：

* [VS Code](https://code.visualstudio.com/)
* [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
* [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 📄 开源协议

本项目采用 [MIT License](https://www.google.com/search?q=LICENSE) 协议。