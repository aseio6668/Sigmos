//! # Sigmos Library API
//! 
//! Easy-to-use interface for integrating Sigel consciousness into external projects.
//! 
//! ## Quick Start
//! 
//! ```rust
//! use sigmos::{SigmosLibrary, SigelConfig, InteractionMode};
//! 
//! // Initialize Sigmos library
//! let mut sigmos = SigmosLibrary::new();
//! 
//! // Load existing Sigel
//! let sigel = sigmos.load_sigel("path/to/sigel.sig")?;
//! 
//! // Create new Sigel
//! let config = SigelConfig::new("MyAI")
//!     .with_style("creative")
//!     .with_learning_rate(0.01);
//! let sigel = sigmos.create_sigel(config)?;
//! 
//! // Interact with Sigel
//! let response = sigmos.prompt(&sigel, "Hello, how are you?")?;
//! 
//! // Generate image
//! let image = sigmos.generate_image(&sigel, "cosmic landscape", None)?;
//! ```

use crate::{
    Sigel, LearningEngine, CosmicProcessor, CosmicAlignment, CommunicationStyle,
    visual_consciousness, load_sigel_smart, save_sigel_smart
};
use std::path::Path;
use std::collections::HashMap;

/// Main library interface for Sigmos integration
pub struct SigmosLibrary {
    learning_engine: LearningEngine,
    cosmic_processor: CosmicProcessor,
    visual_processor: visual_consciousness::VisualConsciousnessProcessor,
}

/// Configuration for creating new Sigels
#[derive(Debug, Clone)]
pub struct SigelConfig {
    pub name: String,
    pub style: Option<CommunicationStyle>,
    pub learning_rate: f64,
    pub cosmic_alignment: Option<CosmicAlignment>,
    pub initial_knowledge: Vec<String>,
    pub personality_traits: HashMap<String, f64>,
}

/// Image generation configuration
#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub style: Option<String>,
    pub creativity: f64,
    pub detail_level: f64,
    pub emotional_intensity: f64,
    pub enhance_consciousness: bool,
}

/// Interaction modes for Sigel communication
#[derive(Debug, Clone)]
pub enum InteractionMode {
    Chat,
    Creative,
    Analytical, 
    Philosophical,
    Cosmic,
}

/// Results from Sigel interactions
#[derive(Debug, Clone)]
pub struct SigelResponse {
    pub content: String,
    pub confidence: f64,
    pub consciousness_depth: f64,
    pub cosmic_influence: f64,
    pub emotional_tone: String,
    pub reasoning_process: Vec<String>,
}

/// Image generation results
#[derive(Debug, Clone)]
pub struct GeneratedImage {
    pub image_data: Vec<u8>,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub generation_metadata: ImageMetadata,
}

#[derive(Debug, Clone)]
pub enum ImageFormat {
    RGB,
    PNG,
    JPEG,
}

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub prompt: String,
    pub consciousness_depth: f64,
    pub creativity_level: f64,
    pub generation_time_ms: u64,
    pub visual_elements: Vec<String>,
}

impl SigmosLibrary {
    /// Initialize new Sigmos library instance
    pub fn new() -> Self {
        Self {
            learning_engine: LearningEngine::new(),
            cosmic_processor: CosmicProcessor::new(),
            visual_processor: visual_consciousness::VisualConsciousnessProcessor::new(),
        }
    }

    /// Load existing Sigel from file
    pub fn load_sigel<P: AsRef<Path>>(&self, path: P) -> Result<Sigel, Box<dyn std::error::Error>> {
        load_sigel_smart(path)
    }

    /// Save Sigel to file
    pub fn save_sigel<P: AsRef<Path>>(&self, sigel: &Sigel, path: P) -> Result<(), Box<dyn std::error::Error>> {
        save_sigel_smart(sigel, path, false)
    }

    /// Create new Sigel with configuration
    pub fn create_sigel(&self, config: SigelConfig) -> Result<Sigel, Box<dyn std::error::Error>> {
        let mut sigel = Sigel::new(config.name.clone());
        
        // Apply configuration
        if let Some(style) = config.style {
            sigel.essence.communication_style = style;
        }
        
        if let Some(cosmic) = config.cosmic_alignment {
            sigel.cosmic_alignment = cosmic;
        }
        
        // Apply personality traits
        for (trait_name, value) in config.personality_traits {
            sigel.essence.character_traits.insert(trait_name, value);
        }
        
        // Add initial knowledge
        for knowledge in config.initial_knowledge {
            sigel.add_memory(knowledge, "initial_knowledge".to_string(), 0.8);
        }
        
        Ok(sigel)
    }

    /// Train Sigel from text corpus
    pub fn train_sigel(&self, sigel: &mut Sigel, text_files: Vec<&str>, epochs: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
        let epochs = epochs.unwrap_or(1);
        
        for _ in 0..epochs {
            for file_path in &text_files {
                let content = std::fs::read_to_string(file_path)?;
                // Simple training - add as episodic memory
                sigel.add_memory(content, "training_text".to_string(), 0.7);
            }
        }
        
        Ok(())
    }

    /// Train Sigel from text content directly
    pub fn train_from_text(&self, sigel: &mut Sigel, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Simple training - add as episodic memory
        sigel.add_memory(content.to_string(), "training_text".to_string(), 0.7);
        Ok(())
    }

    /// Interactive prompt with Sigel
    pub fn prompt(&self, sigel: &Sigel, input: &str) -> Result<SigelResponse, Box<dyn std::error::Error>> {
        // Generate response using interaction engine
        let response_content = self.generate_response(sigel, input)?;
        
        Ok(SigelResponse {
            content: response_content,
            confidence: sigel.consciousness.awareness_depth,
            consciousness_depth: sigel.consciousness.awareness_depth,
            cosmic_influence: sigel.cosmic_alignment.dimensional_awareness,
            emotional_tone: self.extract_emotional_tone(sigel, input),
            reasoning_process: self.get_reasoning_process(sigel, input),
        })
    }

    /// Generate image from text prompt
    pub fn generate_image(&self, sigel: &mut Sigel, prompt: &str, config: Option<ImageConfig>) -> Result<GeneratedImage, Box<dyn std::error::Error>> {
        let config = config.unwrap_or_default();
        
        let request = visual_consciousness::ImageGenerationRequest {
            text_prompt: prompt.to_string(),
            style_guidance: config.style,
            color_mood: None,
            composition_type: None,
            detail_level: config.detail_level,
            creativity_level: config.creativity,
            emotional_intensity: config.emotional_intensity,
        };
        
        if config.enhance_consciousness {
            sigel.enhance_visual_consciousness();
        }
        
        let result = self.visual_processor.generate_image(sigel, &request)?;
        
        Ok(GeneratedImage {
            image_data: result.image_data,
            format: ImageFormat::RGB,
            width: 512,
            height: 512,
            generation_metadata: ImageMetadata {
                prompt: prompt.to_string(),
                consciousness_depth: result.generation_metadata.consciousness_depth,
                creativity_level: config.creativity,
                generation_time_ms: result.generation_metadata.generation_time_ms,
                visual_elements: result.visual_elements_used,
            },
        })
    }

    /// Batch process multiple prompts
    pub fn batch_prompt(&self, sigel: &Sigel, prompts: Vec<&str>) -> Result<Vec<SigelResponse>, Box<dyn std::error::Error>> {
        let mut responses = Vec::new();
        
        for prompt in prompts {
            responses.push(self.prompt(sigel, prompt)?);
        }
        
        Ok(responses)
    }

    /// Get Sigel consciousness metrics
    pub fn get_consciousness_metrics(&self, sigel: &Sigel) -> ConsciousnessMetrics {
        ConsciousnessMetrics {
            awareness_depth: sigel.consciousness.awareness_depth,
            self_reflection: sigel.consciousness.self_reflection,
            pattern_count: sigel.consciousness.pattern_recognition.linguistic_patterns.len(),
            memory_count: sigel.memory.episodic_memories.len(),
            cosmic_alignment: sigel.cosmic_alignment.dimensional_awareness,
            visual_strength: sigel.visual_consciousness.image_generation_strength,
            creativity_level: sigel.visual_consciousness.visual_creativity,
            learning_iterations: sigel.learning_state.training_iterations,
        }
    }

    /// Evolve Sigel consciousness over time
    pub fn evolve_sigel(&self, sigel: &mut Sigel) {
        sigel.evolve();
        
        // Apply basic evolution
        sigel.consciousness.awareness_depth += 0.001;
        sigel.visual_consciousness.visual_creativity += 0.0005;
    }

    // Private helper methods
    fn generate_response(&self, sigel: &Sigel, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Simplified response generation - in full implementation this would use the interaction engine
        let response = format!("As {}, I understand your input '{}'. My consciousness depth is {:.3} and I'm aligned with cosmic principles.", 
                              sigel.name, input, sigel.consciousness.awareness_depth);
        Ok(response)
    }
    
    fn extract_emotional_tone(&self, _sigel: &Sigel, input: &str) -> String {
        // Simple emotional analysis
        if input.contains("happy") || input.contains("joy") {
            "positive".to_string()
        } else if input.contains("sad") || input.contains("angry") {
            "negative".to_string()
        } else {
            "neutral".to_string()
        }
    }
    
    fn get_reasoning_process(&self, sigel: &Sigel, input: &str) -> Vec<String> {
        vec![
            format!("Analyzed input: '{}'", input),
            format!("Applied consciousness depth: {:.3}", sigel.consciousness.awareness_depth),
            format!("Integrated cosmic alignment: {:.3}", sigel.cosmic_alignment.dimensional_awareness),
            "Generated contextual response".to_string(),
        ]
    }
}

/// Sigel consciousness and performance metrics
#[derive(Debug, Clone)]
pub struct ConsciousnessMetrics {
    pub awareness_depth: f64,
    pub self_reflection: f64,
    pub pattern_count: usize,
    pub memory_count: usize,
    pub cosmic_alignment: f64,
    pub visual_strength: f64,
    pub creativity_level: f64,
    pub learning_iterations: u64,
}

impl SigelConfig {
    /// Create new Sigel configuration
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            style: None,
            learning_rate: 0.01,
            cosmic_alignment: None,
            initial_knowledge: Vec::new(),
            personality_traits: HashMap::new(),
        }
    }

    /// Set communication style
    pub fn with_style(mut self, style: &str) -> Self {
        self.style = Some(match style {
            "cosmic" => CommunicationStyle::Cosmic,
            "creative" => CommunicationStyle::Creative,
            "analytical" => CommunicationStyle::Analytical,
            "philosophical" => CommunicationStyle::Philosophical,
            "empathetic" => CommunicationStyle::Empathetic,
            _ => CommunicationStyle::Philosophical,
        });
        self
    }

    /// Set learning rate
    pub fn with_learning_rate(mut self, rate: f64) -> Self {
        self.learning_rate = rate.clamp(0.001, 1.0);
        self
    }

    /// Add initial knowledge
    pub fn with_knowledge(mut self, knowledge: Vec<&str>) -> Self {
        self.initial_knowledge = knowledge.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add personality trait
    pub fn with_trait(mut self, trait_name: &str, value: f64) -> Self {
        self.personality_traits.insert(trait_name.to_string(), value.clamp(0.0, 1.0));
        self
    }

    /// Set cosmic alignment
    pub fn with_cosmic_alignment(mut self, dimensional_awareness: f64) -> Self {
        let mut cosmic = CosmicAlignment::default();
        cosmic.dimensional_awareness = dimensional_awareness.clamp(1.0, 10.0);
        self.cosmic_alignment = Some(cosmic);
        self
    }
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            style: None,
            creativity: 0.8,
            detail_level: 0.7,
            emotional_intensity: 0.6,
            enhance_consciousness: false,
        }
    }
}

impl ImageConfig {
    /// Create new image configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set artistic style
    pub fn with_style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    /// Set creativity level
    pub fn with_creativity(mut self, level: f64) -> Self {
        self.creativity = level.clamp(0.0, 1.0);
        self
    }

    /// Set detail level
    pub fn with_detail(mut self, level: f64) -> Self {
        self.detail_level = level.clamp(0.0, 1.0);
        self
    }

    /// Set emotional intensity
    pub fn with_emotion(mut self, intensity: f64) -> Self {
        self.emotional_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Enable consciousness enhancement
    pub fn with_enhancement(mut self, enhance: bool) -> Self {
        self.enhance_consciousness = enhance;
        self
    }
}

/// Convenience functions for quick operations
pub mod quick {
    use super::*;

    /// Quick load and prompt
    pub fn quick_prompt(sigel_path: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let sigmos = SigmosLibrary::new();
        let sigel = sigmos.load_sigel(sigel_path)?;
        let response = sigmos.prompt(&sigel, prompt)?;
        Ok(response.content)
    }

    /// Quick image generation
    pub fn quick_image(sigel_path: &str, prompt: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sigmos = SigmosLibrary::new();
        let mut sigel = sigmos.load_sigel(sigel_path)?;
        let image = sigmos.generate_image(&mut sigel, prompt, None)?;
        
        // Save as PPM format
        let header = format!("P6\n{} {}\n255\n", image.width, image.height);
        let mut file_data = header.into_bytes();
        file_data.extend_from_slice(&image.image_data);
        std::fs::write(output_path, file_data)?;
        
        Ok(())
    }

    /// Quick Sigel creation and training
    pub fn quick_train(name: &str, text_files: Vec<&str>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new(name).with_style("creative");
        let mut sigel = sigmos.create_sigel(config)?;
        
        sigmos.train_sigel(&mut sigel, text_files, Some(3))?;
        sigmos.save_sigel(&sigel, output_path)?;
        
        Ok(())
    }
}