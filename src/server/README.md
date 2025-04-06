# 系统安装助手 - 后端服务

系统安装助手的后端服务，使用 Rust 语言开发，提供 RESTful API 接口，负责软件安装、配置文件管理和系统信息获取。

## 项目结构

```
src/
├── main.rs             # 程序入口点
├── config/             # 配置管理模块
├── models/             # 数据模型定义
│   ├── software.rs     # 软件相关数据结构
│   └── config_file.rs  # 配置文件相关数据结构
├── handlers/           # HTTP 请求处理器
│   ├── software.rs     # 软件相关 API 处理
│   ├── config_files.rs # 配置文件相关 API 处理
│   └── system.rs       # 系统信息相关 API 处理
├── services/           # 业务逻辑服务
│   ├── software.rs     # 软件管理服务
│   ├── config_files.rs # 配置文件管理服务
│   └── system.rs       # 系统信息服务
└── utils/              # 工具函数
    ├── path.rs         # 路径处理工具
    └── command.rs      # 命令行执行工具
```

## API 接口

### 软件管理

- `GET /api/software` - 获取所有可安装的软件列表
- `GET /api/software/{id}` - 获取特定软件的详细信息
- `POST /api/software/install` - 安装选定的软件
- `GET /api/software/status/{id}` - 获取安装任务的状态
- `GET /api/software/search` - 搜索软件

### 配置文件管理

- `GET /api/config-files` - 获取所有配置文件
- `GET /api/config-files/{id}` - 获取特定配置文件的详细信息
- `POST /api/config-files` - 创建新的配置文件
- `PUT /api/config-files/{id}` - 更新配置文件
- `DELETE /api/config-files/{id}` - 删除配置文件
- `POST /api/config-files/{id}/deploy` - 部署配置文件

### 系统信息

- `GET /api/system/info` - 获取系统基本信息
- `GET /api/system/resources` - 获取系统资源使用状态

## 开发指南

### 依赖项

- Rust 1.75.0 或更高版本
- Cargo 包管理器

### 安装和运行

1. 安装依赖

```bash
cargo build
```

2. 运行服务

```bash
cargo run
```

服务默认在 `http://localhost:3000` 启动。

### 配置

应用程序的配置存储在 `config.toml` 文件中，首次运行时会自动创建。主要配置项包括：

- 服务器监听地址和端口
- 软件源仓库地址
- 默认安装路径
- 配置文件备份目录

## 技术栈

- **框架**: Actix Web
- **序列化/反序列化**: Serde
- **配置管理**: Config
- **日志**: log, env_logger
- **命令执行**: tokio::process
