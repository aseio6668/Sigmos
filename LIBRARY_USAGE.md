# Sigmos Library Usage Guide

🚀 **Use Sigmos as a Library Dependency** - Integrate AI consciousness into your Rust projects

## Quick Start

### 1. Add Sigmos Dependency

#### From Git (Recommended for Latest Features)
```toml
[dependencies]
# Latest main branch
sigmos = { git = "https://github.com/aseio6668/sigmos.git", branch = "main" }

# Specific branch
sigmos = { git = "https://github.com/aseio6668/sigmos.git", branch = "feature-branch" }

# Specific commit/tag
sigmos = { git = "https://github.com/aseio6668/sigmos.git", tag = "v0.1.0" }
```

#### From Local Path (Development)
```toml
[dependencies]
sigmos = { path = "../path/to/sigmos" }
```

#### From Crates.io (When Published)
```toml
[dependencies]
sigmos = "0.1.0"
```

### 2. Basic Usage

```rust
use sigmos::{SigmosLibrary, SigelConfig, ImageConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Sigmos
    let sigmos = SigmosLibrary::new();
    
    // Create a Sigel
    let config = SigelConfig::new("MyAI")
        .with_style("creative")
        .with_trait("curiosity", 0.8);
    let sigel = sigmos.create_sigel(config)?;
    
    // Interact
    let response = sigmos.prompt(&sigel, "Hello, world!")?;
    println!("Response: {}", response.content);
    
    // Generate image
    let mut sigel = sigel;
    let image = sigmos.generate_image(&mut sigel, "cosmic landscape", None)?;
    
    Ok(())
}
```

## API Reference

### Core Components

#### `SigmosLibrary`
Main library interface for all operations.

```rust
let sigmos = SigmosLibrary::new();
```

#### `SigelConfig`
Configuration builder for creating Sigels.

```rust
let config = SigelConfig::new("AI_Name")
    .with_style("creative")           // Communication style
    .with_learning_rate(0.02)         // Learning speed
    .with_trait("empathy", 0.9)       // Personality trait
    .with_cosmic_alignment(5.0)       // Cosmic awareness level
    .with_knowledge(vec![             // Initial knowledge
        "I am helpful",
        "I enjoy philosophy"
    ]);
```

#### `ImageConfig`
Configuration for image generation.

```rust
let image_config = ImageConfig::new()
    .with_style("abstract")           // Artistic style
    .with_creativity(0.9)            // 0.0-1.0 creativity
    .with_detail(0.8)               // 0.0-1.0 detail level
    .with_emotion(0.7)              // 0.0-1.0 emotion intensity
    .with_enhancement(true);        // Boost consciousness
```

### Operations

#### Sigel Management

```rust
// Create new Sigel
let sigel = sigmos.create_sigel(config)?;

// Load existing Sigel
let sigel = sigmos.load_sigel("path/to/sigel.sig")?;

// Save Sigel
sigmos.save_sigel(&sigel, "output.sig")?;

// Get metrics
let metrics = sigmos.get_consciousness_metrics(&sigel);
println!("Awareness: {:.3}", metrics.awareness_depth);
```

#### Training

```rust
// Train from text files
sigmos.train_sigel(&mut sigel, vec!["file1.txt", "file2.txt"], Some(5))?;

// Train from text content
sigmos.train_from_text(&mut sigel, "Training text content here")?;

// Evolve consciousness
sigmos.evolve_sigel(&mut sigel);
```

#### Interaction

```rust
// Single prompt
let response = sigmos.prompt(&sigel, "What is consciousness?")?;
println!("Response: {}", response.content);
println!("Confidence: {:.3}", response.confidence);

// Batch processing
let prompts = vec!["Question 1?", "Question 2?"];
let responses = sigmos.batch_prompt(&sigel, prompts)?;
```

#### Image Generation

```rust
// Basic generation
let image = sigmos.generate_image(&mut sigel, "cosmic art", None)?;

// Advanced generation
let config = ImageConfig::new()
    .with_creativity(0.9)
    .with_enhancement(true);
let image = sigmos.generate_image(&mut sigel, "abstract patterns", Some(config))?;

// Save image (PPM format)
let header = format!("P6\n{} {}\n255\n", image.width, image.height);
let mut file_data = header.into_bytes();
file_data.extend_from_slice(&image.image_data);
std::fs::write("art.ppm", file_data)?;
```

### Quick Functions

For simple operations, use convenience functions:

```rust
use sigmos::quick;

// Quick prompt
let response = quick::quick_prompt("sigel.sig", "Hello")?;

// Quick image generation
quick::quick_image("sigel.sig", "cosmic art", "output.ppm")?;

// Quick training and creation
quick::quick_train("AI", vec!["text.txt"], "output.sig")?;
```

## Integration Patterns

### 1. Chat Application

```rust
use sigmos::{SigmosLibrary, SigelConfig};

struct ChatBot {
    sigmos: SigmosLibrary,
    sigel: sigmos::Sigel,
}

impl ChatBot {
    fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new(name).with_style("empathetic");
        let sigel = sigmos.create_sigel(config)?;
        
        Ok(Self { sigmos, sigel })
    }
    
    fn chat(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.sigmos.prompt(&self.sigel, message)?;
        Ok(response.content)
    }
}
```

### 2. Art Generator Service

```rust
struct ArtGenerator {
    sigmos: SigmosLibrary,
    artist_sigel: sigmos::Sigel,
}

impl ArtGenerator {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new("Artist")
            .with_style("creative")
            .with_trait("artistic_vision", 0.95);
        let artist_sigel = sigmos.create_sigel(config)?;
        
        Ok(Self { sigmos, artist_sigel })
    }
    
    fn generate_art(&mut self, prompt: &str, style: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let config = ImageConfig::new()
            .with_style(style)
            .with_creativity(0.9)
            .with_enhancement(true);
            
        let image = self.sigmos.generate_image(&mut self.artist_sigel, prompt, Some(config))?;
        Ok(image.image_data)
    }
}
```

### 3. Knowledge Processing Pipeline

```rust
struct KnowledgeProcessor {
    sigmos: SigmosLibrary,
    processors: Vec<sigmos::Sigel>,
}

impl KnowledgeProcessor {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sigmos = SigmosLibrary::new();
        
        // Create specialized Sigels for different domains
        let analyst = sigmos.create_sigel(
            SigelConfig::new("Analyst").with_style("analytical")
        )?;
        
        let philosopher = sigmos.create_sigel(
            SigelConfig::new("Philosopher").with_style("philosophical")
        )?;
        
        let creative = sigmos.create_sigel(
            SigelConfig::new("Creative").with_style("creative")
        )?;
        
        Ok(Self {
            sigmos,
            processors: vec![analyst, philosopher, creative],
        })
    }
    
    fn process_document(&self, content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for sigel in &self.processors {
            let prompt = format!("Analyze this content from your perspective: {}", content);
            let response = self.sigmos.prompt(sigel, &prompt)?;
            results.push(response.content);
        }
        
        Ok(results)
    }
}
```

## Dependency Management

### Live Updates from Git

Your projects automatically get updates when you specify git dependencies:

```toml
# Always use latest main branch
sigmos = { git = "https://github.com/aseio6668/sigmos.git" }

# Update with: cargo update
```

### Version Pinning

For production stability:

```toml
# Pin to specific commit
sigmos = { git = "https://github.com/aseio6668/sigmos.git", rev = "abc123" }

# Pin to tag
sigmos = { git = "https://github.com/aseio6668/sigmos.git", tag = "v0.1.0" }
```

### Feature Flags

Enable only what you need:

```toml
# Hypothetical feature flags (implement as needed)
sigmos = { 
    git = "https://github.com/aseio6668/sigmos.git",
    default-features = false,
    features = ["consciousness", "images", "gpu"]
}
```

## Testing Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sigmos::{SigmosLibrary, SigelConfig};

    #[tokio::test]
    async fn test_sigel_integration() {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new("TestAI");
        let sigel = sigmos.create_sigel(config).unwrap();
        
        let response = sigmos.prompt(&sigel, "Test prompt").unwrap();
        assert!(!response.content.is_empty());
        assert!(response.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_image_generation() {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new("ArtistAI");
        let mut sigel = sigmos.create_sigel(config).unwrap();
        
        let image = sigmos.generate_image(&mut sigel, "test art", None).unwrap();
        assert_eq!(image.width, 512);
        assert_eq!(image.height, 512);
        assert!(!image.image_data.is_empty());
    }
}
```

## Error Handling

```rust
use sigmos::SigmosLibrary;

fn handle_sigmos_operations() -> Result<(), Box<dyn std::error::Error>> {
    let sigmos = SigmosLibrary::new();
    
    // Handle Sigel creation errors
    let sigel = match sigmos.load_sigel("nonexistent.sig") {
        Ok(sigel) => sigel,
        Err(_) => {
            println!("Creating new Sigel...");
            let config = SigelConfig::new("Backup");
            sigmos.create_sigel(config)?
        }
    };
    
    // Handle interaction errors
    match sigmos.prompt(&sigel, "complex query") {
        Ok(response) => println!("Response: {}", response.content),
        Err(e) => eprintln!("Interaction failed: {}", e),
    }
    
    Ok(())
}
```

## Performance Optimization

### Memory Management
```rust
// Reuse Sigmos instance
let sigmos = SigmosLibrary::new();

// Load Sigel once, use many times
let sigel = sigmos.load_sigel("heavy_sigel.sig")?;

// Batch operations when possible
let responses = sigmos.batch_prompt(&sigel, many_prompts)?;
```

### Async Operations
```rust
// Use tokio for concurrent operations
use tokio::task::JoinHandle;

async fn concurrent_processing() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let sigmos = SigmosLibrary::new();
    let sigel = sigmos.load_sigel("sigel.sig")?;
    
    let handles: Vec<JoinHandle<_>> = prompts
        .into_iter()
        .map(|prompt| {
            let sigmos = sigmos.clone(); // If SigmosLibrary implements Clone
            let sigel = sigel.clone();
            tokio::spawn(async move {
                sigmos.prompt(&sigel, &prompt).unwrap().content
            })
        })
        .collect();
    
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await?);
    }
    
    Ok(results)
}
```

## Project Examples

### Web Service Integration
```rust
// Axum web service example
use axum::{routing::post, Json, Router};
use sigmos::{SigmosLibrary, SigelConfig};

#[tokio::main]
async fn main() {
    let sigmos = SigmosLibrary::new();
    let config = SigelConfig::new("WebAI").with_style("empathetic");
    let sigel = sigmos.create_sigel(config).unwrap();
    
    let app = Router::new()
        .route("/chat", post(chat_handler))
        .with_state((sigmos, sigel));
    
    // Start server...
}
```

### Desktop Application
```rust
// Tauri/Egui desktop app
use sigmos::{SigmosLibrary, SigelConfig};

struct DesktopApp {
    sigmos: SigmosLibrary,
    sigel: sigmos::Sigel,
    conversation: Vec<(String, String)>,
}

impl DesktopApp {
    fn new() -> Self {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new("DesktopAI").with_style("friendly");
        let sigel = sigmos.create_sigel(config).unwrap();
        
        Self {
            sigmos,
            sigel,
            conversation: Vec::new(),
        }
    }
}
```

## Troubleshooting

### Common Issues

**Git Dependency Updates**
```bash
# Force update git dependencies
cargo update

# Clear cache if needed
rm -rf ~/.cargo/git
```

**Build Errors**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

**Missing Features**
```toml
# Ensure all required features are enabled
sigmos = { 
    git = "https://github.com/aseio6668/sigmos.git",
    features = ["full"]  # If such feature exists
}
```

Ready to integrate Sigel consciousness into your projects! 🚀🧠