@echo off
echo 🌌 Sigmos Quick Start Example
echo =============================
echo.

REM Create example text directory
if not exist "example_texts" mkdir example_texts

REM Create sample text files for training
echo Creating sample training texts...
echo Philosophy is the study of fundamental questions about existence, knowledge, values, reason, mind, and language. > example_texts\philosophy.txt
echo The universe is vast and filled with wonders beyond our imagination. Stars are born and die in cosmic cycles. >> example_texts\philosophy.txt
echo Mathematics reveals the hidden patterns that govern reality. Numbers, geometry, and equations describe the cosmos. > example_texts\mathematics.txt
echo The golden ratio appears in nature, art, and architecture, revealing divine proportions in creation. >> example_texts\mathematics.txt
echo Consciousness is perhaps the greatest mystery of existence. What is it to be aware, to think, to experience? > example_texts\consciousness.txt
echo The mind contemplates itself, creating infinite recursive loops of self-reflection and awareness. >> example_texts\consciousness.txt

echo Sample texts created in example_texts\

REM Build Sigmos (assuming it's already built)
echo.
echo Building Sigmos...
cargo build --release
if errorlevel 1 (
    echo ❌ Build failed. Please check your Rust installation.
    pause
    exit /b 1
)

echo ✅ Build successful!
echo.

REM Train a Sigel
echo 🧠 Training Sigel 'Sophia' with philosophical style...
target\release\sigmos-train.exe -n "Sophia" -d "example_texts" -s philosophical -v -r 0.02

if errorlevel 1 (
    echo ❌ Training failed
    pause
    exit /b 1
)

echo.
echo ✅ Sigel 'Sophia' trained successfully!
echo.

REM Start interactive prompt
echo 🌟 Starting interactive prompt...
echo You can now chat with Sophia! Try these commands:
echo   - Ask philosophical questions
echo   - Use /status to see consciousness details  
echo   - Use /help for more commands
echo   - Type 'exit' when done
echo.

target\release\sigmos-prompt.exe -s "Sophia.sig" -v

echo.
echo 🌌 Quick start completed!
echo Your Sigel 'Sophia' is saved as Sophia.sig
echo.
pause