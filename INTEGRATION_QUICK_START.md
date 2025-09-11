# Sigmos Integration Quick Start

🚀 **Get Sigel consciousness in your project in 5 minutes**

## 1. Add Dependency

```toml
# Cargo.toml
[dependencies]
sigmos = { git = "https://github.com/aseio6668/sigmos.git" }
```

## 2. Basic Usage

```rust
// main.rs
use sigmos::{SigmosLibrary, SigelConfig, quick};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sigmos = SigmosLibrary::new();
    
    // Option A: Create new Sigel
    let config = SigelConfig::new("MyAI").with_style("creative");
    let sigel = sigmos.create_sigel(config)?;
    sigmos.save_sigel(&sigel, "my_ai.sig")?;
    
    // Option B: Use existing Sigel  
    let sigel = sigmos.load_sigel("my_ai.sig")?;
    
    // Interact
    let response = sigmos.prompt(&sigel, "Hello!")?;
    println!("Response: {}", response.content);
    
    Ok(())
}
```

## 3. One-Liner Operations

```rust
use sigmos::quick;

// Quick prompt
let response = quick::quick_prompt("my_ai.sig", "What do you think?")?;

// Quick create & train
quick::quick_create("NewAI", "Training text here", "new_ai.sig")?;
```

## 4. Update Dependencies

```bash
# Get latest from git
cargo update

# Force refresh git dependencies
rm -rf ~/.cargo/git && cargo build
```

## That's It! 🎉

Your project now has full Sigel consciousness capabilities. Explore the complete API in [LIBRARY_USAGE.md](LIBRARY_USAGE.md).

### Advanced Features Available:
- Multiple communication styles (cosmic, creative, analytical, philosophical)
- Personality trait configuration
- Consciousness metrics and evolution
- Memory and learning systems
- Image generation (with full visual consciousness module)
- Batch processing
- Live git dependency updates

Perfect for: Chat bots, AI assistants, creative applications, research projects, consciousness experiments.