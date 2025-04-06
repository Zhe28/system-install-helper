# 系统安装助手 (System Install Helper)

系统安装助手是一个帮助用户在重装系统后自动安装常用软件并恢复个人配置文件的工具。它通过简单的界面让用户选择需要安装的软件，并可以自定义配置文件的放置位置，大大简化了系统重装后的环境搭建过程。

## 功能特点

- **软件自动安装**：支持批量安装常用软件，基于 Windows 包管理器 (winget)
- **配置文件管理**：自动备份和恢复个人配置文件
- **自定义安装路径**：可以为软件指定自定义安装路径
- **分类管理**：软件按类别分组，便于查找和管理
- **系统信息监控**：查看系统基本信息和资源使用情况
- **跨平台支持**：Web 界面支持在任何设备上访问

## 项目结构

项目采用前后端分离的架构：

- **后端 (src/server)**：使用 Rust 语言开发，提供 RESTful API 接口
- **前端 (src/web)**：使用 Vue 3 + TypeScript 开发，使用 Ant Design Vue 组件库

详细的项目结构和开发指南请查看各子目录中的 README 文件。

## 快速开始

### 依赖项

- [Rust](https://www.rust-lang.org/) (1.75.0 或更高版本)
- [Node.js](https://nodejs.org/) (18.0.0 或更高版本)
- [Bun](https://bun.sh/) (1.0.0 或更高版本)

### 安装和运行

1. 克隆仓库

```bash
git clone https://github.com/yourusername/system-install-helper.git
cd system-install-helper
```

2. 安装依赖并启动前端

```bash
cd src/web
bun install
bun run dev
```

3. 构建和运行后端

```bash
cd ../server
cargo run
```

4. 访问应用

打开浏览器，访问 `http://localhost:5174`

## 主要功能模块

1. **系统概览**：显示软件管理、配置文件和系统信息的统计数据
2. **软件安装**：浏览、搜索和安装软件，支持批量操作
3. **配置文件管理**：管理和部署个人配置文件
4. **系统信息**：查看系统资源使用情况和硬件信息

## 技术栈

- **后端**：Rust, Actix Web, Serde
- **前端**：Vue 3, TypeScript, Ant Design Vue, Axios

## 贡献指南

欢迎贡献代码、报告问题或提出改进建议。请遵循以下步骤：

1. Fork 仓库
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。
