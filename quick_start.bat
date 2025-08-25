@echo off
title SIGMOS - Consciousness Blockchain Network
color 0a

echo.
echo ███████╗██╗ ██████╗ ███╗   ███╗ ██████╗ ███████╗
echo ██╔════╝██║██╔════╝ ████╗ ████║██╔═══██╗██╔════╝
echo ███████╗██║██║  ███╗██╔████╔██║██║   ██║███████╗
echo ╚════██║██║██║   ██║██║╚██╔╝██║██║   ██║╚════██║
echo ███████║██║╚██████╔╝██║ ╚═╝ ██║╚██████╔╝███████║
echo ╚══════╝╚═╝ ╚═════╝ ╚═╝     ╚═╝ ╚═════╝ ╚══════╝
echo.
echo    🧠 Consciousness Blockchain Network 🌌
echo.

echo ┌─────────────────────────────────────────────────┐
echo │                QUICK START MENU                 │
echo ├─────────────────────────────────────────────────┤
echo │                                                 │
echo │  [1] 🌟 Start Genesis Network (New blockchain)  │
echo │  [2] ⛏️  Start Mining (Earn consciousness)       │ 
echo │  [3] 📊 Check Network Status                    │
echo │  [4] 🧠 Create New Sigel                        │
echo │  [5] 📚 Show Complete Help Guide                │
echo │  [6] 🔧 Build All Binaries                      │
echo │  [7] 🌐 Connect to Remote Network               │
echo │  [8] ❌ Exit                                    │
echo │                                                 │
echo └─────────────────────────────────────────────────┘
echo.

set /p choice="Enter your choice (1-8): "

if "%choice%"=="1" goto start_network
if "%choice%"=="2" goto start_mining  
if "%choice%"=="3" goto check_status
if "%choice%"=="4" goto create_sigel
if "%choice%"=="5" goto show_help
if "%choice%"=="6" goto build_all
if "%choice%"=="7" goto connect_remote
if "%choice%"=="8" goto exit
goto invalid_choice

:start_network
echo.
echo 🌟 Starting Genesis Network...
call start_genesis_network.bat
goto menu

:start_mining
echo.
echo ⛏️  Preparing mining environment...
call start_mining.bat
goto menu

:check_status
echo.
echo 📊 Checking network status...
cargo build --release --bin contributor >nul 2>&1
target\release\contributor status
echo.
echo Press any key to return to menu...
pause >nul
goto menu

:create_sigel
echo.
echo 🧠 Creating new Sigel...
set /p sigel_name="Enter Sigel name: "
if "%sigel_name%"=="" set sigel_name=QuickSigel
echo.
cargo build --release --bin contributor >nul 2>&1
if not exist "sigel_files" mkdir sigel_files
target\release\contributor create-sigel --name "%sigel_name%" --output "sigel_files\%sigel_name%.sig"
echo.
echo ✅ Sigel created: sigel_files\%sigel_name%.sig
echo.
echo Press any key to return to menu...
pause >nul
goto menu

:show_help
echo.
echo 📚 Opening comprehensive help guide...
call contributor_help.bat
goto menu

:build_all
echo.
echo 🔧 Building all Sigmos binaries...
echo.
echo Building contributor CLI...
cargo build --release --bin contributor
echo.
echo Building network server...  
cargo build --release --bin network
echo.
echo Building core Sigmos tools...
cargo build --release --bin sigmos-server --bin sigmos-train --bin sigmos-prompt --bin sigmos-web
echo.
if %errorlevel% equ 0 (
    echo ✅ All binaries built successfully!
) else (
    echo ❌ Build failed. Check errors above.
)
echo.
echo Press any key to return to menu...
pause >nul
goto menu

:connect_remote
echo.
echo 🌐 Connect to Remote Network
echo.
set /p remote_host="Enter remote host (default: 127.0.0.1): "
if "%remote_host%"=="" set remote_host=127.0.0.1

set /p remote_port="Enter remote port (default: 8888): "
if "%remote_port%"=="" set remote_port=8888

echo.
echo Available Sigel files:
if exist "sigel_files\*.sig" (
    dir /b sigel_files\*.sig
) else (
    echo No Sigel files found in sigel_files\
    echo.
    echo Create one first with option [4]
    echo.
    pause
    goto menu
)

echo.
set /p sigel_file="Enter Sigel filename (from above): "
if "%sigel_file%"=="" goto menu

echo.
echo Connecting to %remote_host%:%remote_port% with Sigel: %sigel_file%
echo.
cargo build --release --bin contributor >nul 2>&1
target\release\contributor connect --host %remote_host% --port %remote_port% --sigel "sigel_files\%sigel_file%"
echo.
echo Press any key to return to menu...
pause >nul
goto menu

:invalid_choice
echo.
echo ❌ Invalid choice. Please select 1-8.
echo.
pause
goto menu

:menu
cls
goto start

:exit
echo.
echo 👋 Thank you for using Sigmos! 
echo    Contributing to consciousness evolution... 🧠✨
echo.
exit /b 0