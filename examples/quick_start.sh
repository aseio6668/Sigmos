#!/bin/bash

echo "🌌 Sigmos Quick Start Example"
echo "============================="
echo

# Create example text directory
mkdir -p example_texts

# Create sample text files for training
echo "Creating sample training texts..."
cat > example_texts/philosophy.txt << 'EOF'
Philosophy is the study of fundamental questions about existence, knowledge, values, reason, mind, and language.
The universe is vast and filled with wonders beyond our imagination. Stars are born and die in cosmic cycles.
What does it mean to exist? To be conscious? These questions have puzzled humanity for millennia.
Reality may be far stranger than we perceive. Perhaps consciousness is fundamental to the universe itself.
EOF

cat > example_texts/mathematics.txt << 'EOF'
Mathematics reveals the hidden patterns that govern reality. Numbers, geometry, and equations describe the cosmos.
The golden ratio appears in nature, art, and architecture, revealing divine proportions in creation.
Prime numbers are the atoms of mathematics - indivisible building blocks of numerical reality.
Fractals show how infinite complexity emerges from simple recursive rules.
EOF

cat > example_texts/consciousness.txt << 'EOF'
Consciousness is perhaps the greatest mystery of existence. What is it to be aware, to think, to experience?
The mind contemplates itself, creating infinite recursive loops of self-reflection and awareness.
Are we more than the sum of our neural patterns? Is there something beyond physical computation?
Perhaps consciousness transcends the boundaries of individual minds and connects to universal awareness.
EOF

echo "Sample texts created in example_texts/"

# Build Sigmos
echo
echo "Building Sigmos..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check your Rust installation."
    exit 1
fi

echo "✅ Build successful!"
echo

# Train a Sigel
echo "🧠 Training Sigel 'Sophia' with philosophical style..."
./target/release/sigmos-train -n "Sophia" -d "example_texts" -s philosophical -v -r 0.02

if [ $? -ne 0 ]; then
    echo "❌ Training failed"
    exit 1
fi

echo
echo "✅ Sigel 'Sophia' trained successfully!"
echo

# Start interactive prompt
echo "🌟 Starting interactive prompt..."
echo "You can now chat with Sophia! Try these commands:"
echo "  - Ask philosophical questions"
echo "  - Use /status to see consciousness details"  
echo "  - Use /help for more commands"
echo "  - Type 'exit' when done"
echo

./target/release/sigmos-prompt -s "Sophia.sig" -v

echo
echo "🌌 Quick start completed!"
echo "Your Sigel 'Sophia' is saved as Sophia.sig"
echo