@echo off
chcp 65001 >nul
echo Starting Viya PSP DouChong Bang Application (Backend + Frontend on port 2992)...

REM Build frontend first
echo Building frontend...
cd /d "%~dp0frontend"
call npm install
call npm run build
if ERRORLEVEL 1 (
    echo Error: Failed to build frontend
    pause
    exit /b 1
)

REM Return to main directory
cd /d "%~dp0"

REM Build backend first to ensure it compiles
echo Building backend...
cd /d "%~dp0rust_backend"
cargo build
if ERRORLEVEL 1 (
    echo Error: Failed to build backend
    pause
    exit /b 1
)

REM Start the combined backend+frontend server
echo Starting combined server on port 2992...
echo Note: Server is running. Press Ctrl+C to stop the server.
cargo run