@echo off
echo 正在启动系统安装助手后台服务...

REM 检查 Rust 是否安装
where rustc >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Rust 未安装或环境变量未设置正确。
    echo 请访问 https://www.rust-lang.org/tools/install 安装 Rust。
    pause
    exit /b 1
)

REM 检查 Cargo 是否安装
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Cargo 未安装或环境变量未设置正确。
    echo 请确保 Rust 安装正确并重启命令提示符。
    pause
    exit /b 1
)

REM 构建并运行项目
echo 正在构建项目...
cargo build
if %ERRORLEVEL% neq 0 (
    echo 构建失败，请检查错误信息。
    pause
    exit /b 1
)

echo 正在运行服务器...
cargo run
pause
