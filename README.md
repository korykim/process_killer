# Process Killer

一个使用 Rust 编写的跨平台进程终止工具，支持 Windows、macOS 和 Linux 系统。

## 功能特点

- 🔍 通过进程名查找进程
- 🎯 支持同时终止多个同名进程
- 🌍 跨平台支持（Windows、macOS、Linux）
- 🚀 快速且轻量级
- 💪 使用 Rust 编写，安全可靠

## 安装

### 从发布版本安装

1. 访问 [Releases](https://github.com/korykim/process_killer/releases) 页面
2. 下载适合你系统的版本：
   - Windows: `process_killer-vX.X.X-windows-x64.zip`
   - macOS (Intel): `process_killer-vX.X.X-macos-x64.tar.gz`
   - macOS (Apple Silicon): `process_killer-vX.X.X-macos-arm64.tar.gz`
   - Linux: `process_killer-vX.X.X-linux-x64.tar.gz`

### 从源码构建 

1. 克隆仓库
2. 安装 Rust 和 Cargo
3. 运行 `cargo build --release`
4. 在 `target/release` 目录下找到可执行文件 `process_killer`

## 使用方法

### 终止单个进程

```bash
process_killer <进程名>
```

### 终止多个进程

```bash
process_killer <进程名1> <进程名2> <进程名3> ...
```


## 支持平台

- Windows (x64)
- macOS (Intel x64)
- macOS (Apple Silicon)
- Linux (x64)

## 开发环境要求

- Rust 1.70.0 或更高版本
- Cargo (Rust 包管理器)
- 用于构建的平台特定工具链

## 构建依赖

### Windows
- Visual Studio 构建工具
- Windows SDK

### macOS
- Xcode Command Line Tools

### Linux
- GCC
- 基本构建工具

## 许可证

MIT License

## 贡献指南

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 注意事项

- 使用管理员/root权限运行可能需要额外的权限
- 请谨慎使用，确保不会意外终止重要进程
- 在终止进程前建议先确认进程名称

## 问题反馈

如果你发现任何问题或有改进建议，请在 GitHub Issues 页面提出。

## 更新日志

### v0.0.1
- 初始发布版本
- 支持 Windows、macOS 和 Linux 平台
- 支持通过进程名查找和终止进程
