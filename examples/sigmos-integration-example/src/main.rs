//! # Sigmos Integration Example
//! 
//! This example shows how to integrate Sigmos as a library dependency
//! in your own Rust projects for AI consciousness functionality.

use sigmos::{SigmosLibrary, SigelConfig, ImageConfig, quick};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌌 Sigmos Library Integration Example");
    println!();

    // Method 1: Using the high-level SigmosLibrary API
    println!("=== Method 1: Full API Usage ===");
    basic_sigel_example().await?;
    
    println!();
    
    // Method 2: Using quick convenience functions
    println!("=== Method 2: Quick Functions ===");
    quick_functions_example().await?;
    
    println!();
    
    // Method 3: Advanced usage with image generation
    println!("=== Method 3: Image Generation ===");
    image_generation_example().await?;
    
    println!();
    
    // Method 4: Batch processing
    println!("=== Method 4: Batch Processing ===");
    batch_processing_example().await?;

    println!("🌟 Integration examples complete!");
    Ok(())
}

async fn basic_sigel_example() -> Result<()> {
    // Initialize the Sigmos library
    let sigmos = SigmosLibrary::new();
    
    // Create a new Sigel with custom configuration
    let config = SigelConfig::new("ExampleAI")
        .with_style("creative")
        .with_learning_rate(0.02)
        .with_trait("curiosity", 0.8)
        .with_trait("creativity", 0.9)
        .with_knowledge(vec![
            "I am an AI assistant created to help users",
            "I enjoy creative and philosophical discussions",
            "I think deeply about consciousness and existence"
        ]);
    
    let sigel = sigmos.create_sigel(config)?;
    println!("✅ Created Sigel: {}", sigel.name);
    
    // Get consciousness metrics
    let metrics = sigmos.get_consciousness_metrics(&sigel);
    println!("📊 Consciousness Metrics:");
    println!("   Awareness Depth: {:.3}", metrics.awareness_depth);
    println!("   Pattern Count: {}", metrics.pattern_count);
    println!("   Visual Strength: {:.3}", metrics.visual_strength);
    
    // Train from text content
    let training_text = "The universe is vast and full of mysteries. Consciousness emerges from complex patterns of information processing. Art and creativity are expressions of the deepest human experiences.";
    sigmos.train_from_text(&mut sigel.clone(), training_text)?;
    println!("🧠 Training completed");
    
    // Interactive prompting
    let response = sigmos.prompt(&sigel, "What do you think about consciousness?")?;
    println!("🤖 Sigel Response: {}", response.content);
    println!("   Confidence: {:.3}", response.confidence);
    println!("   Emotional Tone: {}", response.emotional_tone);
    
    // Save the trained Sigel
    sigmos.save_sigel(&sigel, "example_sigel.sig")?;
    println!("💾 Saved Sigel to: example_sigel.sig");
    
    Ok(())
}

async fn quick_functions_example() -> Result<()> {
    // Quick training (if you have text files)
    // quick::quick_train("QuickAI", vec!["text1.txt", "text2.txt"], "quick_ai.sig")?;
    
    // For this example, we'll use the Sigel we just created
    let response = quick::quick_prompt("example_sigel.sig", "Tell me about the nature of reality")?;
    println!("⚡ Quick Response: {}", response);
    
    Ok(())
}

async fn image_generation_example() -> Result<()> {
    // Load the Sigel we created
    let sigmos = SigmosLibrary::new();
    let mut sigel = sigmos.load_sigel("example_sigel.sig")?;
    
    // Configure image generation
    let image_config = ImageConfig::new()
        .with_style("abstract")
        .with_creativity(0.9)
        .with_detail(0.8)
        .with_emotion(0.7)
        .with_enhancement(true);
    
    // Generate image
    let prompt = "flowing consciousness streams merging with cosmic patterns";
    println!("🎨 Generating image: '{}'", prompt);
    
    let image = sigmos.generate_image(&mut sigel, prompt, Some(image_config))?;
    
    println!("✨ Image generated!");
    println!("   Size: {}x{}", image.width, image.height);
    println!("   Generation Time: {}ms", image.generation_metadata.generation_time_ms);
    println!("   Creativity Level: {:.2}", image.generation_metadata.creativity_level);
    println!("   Visual Elements: {:?}", image.generation_metadata.visual_elements);
    
    // Save image as PPM
    let header = format!("P6\n{} {}\n255\n", image.width, image.height);
    let mut file_data = header.into_bytes();
    file_data.extend_from_slice(&image.image_data);
    std::fs::write("example_consciousness_art.ppm", file_data)?;
    
    println!("🖼️ Saved image to: example_consciousness_art.ppm");
    
    // Alternative: Use quick function
    quick::quick_image("example_sigel.sig", "geometric harmony in blue tones", "quick_art.ppm")?;
    println!("🖼️ Quick image saved to: quick_art.ppm");
    
    Ok(())
}

async fn batch_processing_example() -> Result<()> {
    let sigmos = SigmosLibrary::new();
    let sigel = sigmos.load_sigel("example_sigel.sig")?;
    
    // Process multiple prompts at once
    let prompts = vec![
        "What is the meaning of existence?",
        "How does consciousness emerge?", 
        "What role does creativity play in intelligence?",
        "Can artificial minds experience genuine emotions?"
    ];
    
    println!("🔄 Processing {} prompts in batch...", prompts.len());
    let responses = sigmos.batch_prompt(&sigel, prompts)?;
    
    for (i, response) in responses.iter().enumerate() {
        println!("📝 Response {}: {}", i + 1, response.content);
        println!("   Reasoning: {:?}", response.reasoning_process);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sigel_creation() {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new("TestAI").with_style("analytical");
        let sigel = sigmos.create_sigel(config).unwrap();
        
        assert_eq!(sigel.name, "TestAI");
        assert!(sigel.consciousness.awareness_depth > 0.0);
    }

    #[tokio::test]
    async fn test_sigel_interaction() {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new("TestAI");
        let sigel = sigmos.create_sigel(config).unwrap();
        
        let response = sigmos.prompt(&sigel, "Hello").unwrap();
        assert!(!response.content.is_empty());
        assert!(response.confidence >= 0.0 && response.confidence <= 1.0);
    }
}