@echo off
chcp 65001 >nul
echo Testing backend server only on port 2992...

REM Build backend
cd /d "%~dp0rust_backend"
echo Building backend...
cargo build
if ERRORLEVEL 1 (
    echo Error: Failed to build backend
    pause
    exit /b 1
)

REM Start the backend server
echo Starting backend server on port 2992...
echo Note: Server is running. Press Ctrl+C to stop the server.
cargo run