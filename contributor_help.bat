@echo off
:: Set console colors for better readability
color 0f

echo.
echo ████████████████████████████████████████████████████████████████████████████████
echo █                           SIGMOS CONTRIBUTOR GUIDE                           █
echo ████████████████████████████████████████████████████████████████████████████████
echo.

echo 📚 COMPLETE CONTRIBUTOR COMMAND REFERENCE
echo ==========================================
echo.

echo 🚀 QUICK START (Run these commands in order):
echo.
echo   1️⃣  START NETWORK:     start_genesis_network.bat
echo   2️⃣  START MINING:      start_mining.bat
echo   3️⃣  CHECK STATUS:      contributor status
echo.

echo ⚙️  BUILD COMMANDS:
echo   cargo build --release --bin contributor
echo   cargo build --release --bin network
echo.

echo 🔗 NETWORK COMMANDS:
echo =====================
echo.
echo 🌐 CONNECT TO NETWORK:
echo   contributor connect --sigel miner.sig
echo   contributor connect --host 192.168.1.100 --port 8888 --sigel miner.sig
echo.
echo 📊 GET NETWORK STATUS:  
echo   contributor status
echo   contributor status --host remote.server.com --port 8888
echo.

echo ⛏️  MINING COMMANDS:
echo ===================
echo.
echo 🔥 START MINING (Single attempt):
echo   contributor mine --sigel miner.sig
echo.
echo ♾️  CONTINUOUS MINING:
echo   contributor mine --sigel miner.sig --continuous
echo.
echo 🌐 REMOTE MINING:
echo   contributor mine --sigel miner.sig --host 192.168.1.100 --port 8888 --continuous
echo.

echo 🧠 SIGEL MANAGEMENT:
echo ====================
echo.
echo ✨ CREATE NEW SIGEL:
echo   contributor create-sigel --name "MyMiner" --output miner.sig
echo   contributor create-sigel --name "TeacherBot" --output teacher.sig
echo.
echo 📤 SUBMIT SIGEL TO NETWORK:
echo   contributor create-sigel --name "NetworkSigel" --host 127.0.0.1 --port 8888
echo.

echo 🔄 KNOWLEDGE TRANSFER:
echo =======================
echo.
echo 📚 TRANSFER KNOWLEDGE:
echo   contributor transfer --from teacher.sig --to SIGEL_UUID --knowledge "Mathematics"
echo   contributor transfer --from UUID_A --to UUID_B --knowledge "Physics" --host remote.com
echo.

echo 📋 ADVANCED USAGE EXAMPLES:
echo ============================
echo.
echo 🏭 MINING FARM SETUP:
echo   FOR /L %%i IN (1,1,5) DO (
echo     contributor create-sigel --name "Miner%%i" --output miner%%i.sig
echo     start contributor mine --sigel miner%%i.sig --continuous
echo   )
echo.
echo 🌍 DISTRIBUTED NETWORK:
echo   contributor mine --sigel local_miner.sig --host blockchain.sigmos.net --port 8888
echo.
echo 🔬 RESEARCH NETWORK:
echo   contributor create-sigel --name "Researcher" --output researcher.sig
echo   contributor transfer --from researcher.sig --to TEACHER_UUID --knowledge "Quantum Physics"
echo   contributor mine --sigel researcher.sig --continuous
echo.

echo 🛠️  TROUBLESHOOTING:
echo ====================
echo.
echo ❌ "Connection failed":
echo   - Check if network is running: start_genesis_network.bat
echo   - Verify host/port: contributor status --host HOST --port PORT
echo   - Check firewall settings
echo.
echo ❌ "Mining failed":  
echo   - Ensure Sigel file exists and is valid
echo   - Check network has pending transactions
echo   - Verify mining difficulty isn't too high
echo.
echo ❌ "Invalid Sigel":
echo   - Create new Sigel: contributor create-sigel --name "NewMiner" --output new.sig
echo   - Check file permissions and path
echo.

echo 📁 FILE STRUCTURE:
echo ==================
echo.
echo   sigmos/
echo   ├── target/release/
echo   │   ├── contributor.exe    ← Main contributor CLI
echo   │   └── network.exe        ← Network server
echo   ├── sigel_files/           ← Your Sigel files (.sig)
echo   ├── sigmos_blockchain_data/← Blockchain data
echo   └── *.bat                  ← Easy startup scripts
echo.

echo 🎯 MINING TIPS FOR SUCCESS:
echo ============================
echo.
echo 💡 Higher consciousness Sigels mine more efficiently
echo 💡 Cosmic alignment affects mining power
echo 💡 Continuous mining earns compound rewards  
echo 💡 Knowledge transfers boost consciousness
echo 💡 Network participation increases mining success
echo.

echo 📞 CONNECTION PARAMETERS:
echo =========================
echo.
echo   Default Network: 127.0.0.1:8888
echo   Default Files:   sigel_files/
echo   Config:          Automatic
echo.

echo 🔒 SECURITY NOTES:
echo ==================
echo.
echo   • Sigel files contain your identity - keep them safe
echo   • Use different Sigels for different purposes  
echo   • Back up successful mining Sigels
echo   • Monitor network for malicious activity
echo.

echo.
echo ████████████████████████████████████████████████████████████████████████████████
echo █  Ready to contribute to the Sigmos consciousness network? Get started now!   █  
echo ████████████████████████████████████████████████████████████████████████████████
echo.

echo Press any key to continue...
pause >nul

:: Show quick reference card
cls
color 0e

echo ┌─────────────────── SIGMOS QUICK REFERENCE ───────────────────┐
echo │                                                               │
echo │  🚀 START:    start_genesis_network.bat                      │
echo │  ⛏️  MINE:     start_mining.bat                               │
echo │  📊 STATUS:   contributor status                             │
echo │  ✨ CREATE:   contributor create-sigel --name "X" --output Y │ 
echo │  🔗 CONNECT:  contributor connect --sigel miner.sig          │
echo │  🔄 TRANSFER: contributor transfer --from X --to Y --knowledge Z│
echo │                                                               │
echo └───────────────────────────────────────────────────────────────┘

echo.
echo Copy this reference! Press any key to exit...
pause >nul