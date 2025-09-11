//! # Sigmos Simple Library API
//! 
//! Simplified API for integrating Sigmos into external projects.
//! Works with the current Sigmos codebase.

use crate::{Sigel, LearningEngine, CosmicProcessor, CommunicationStyle};
use std::path::Path;
use std::collections::HashMap;
use anyhow::Result;

/// Simplified library interface for Sigmos integration
pub struct SigmosLibrary {
    learning_engine: LearningEngine,
    cosmic_processor: CosmicProcessor,
}

/// Configuration for creating new Sigels
#[derive(Debug, Clone)]
pub struct SigelConfig {
    pub name: String,
    pub style: Option<CommunicationStyle>,
    pub learning_rate: f64,
    pub personality_traits: HashMap<String, f64>,
}

/// Results from Sigel interactions
#[derive(Debug, Clone)]
pub struct SigelResponse {
    pub content: String,
    pub confidence: f64,
    pub consciousness_depth: f64,
    pub emotional_tone: String,
}

/// Sigel consciousness metrics
#[derive(Debug, Clone)]
pub struct ConsciousnessMetrics {
    pub awareness_depth: f64,
    pub self_reflection: f64,
    pub pattern_count: usize,
    pub memory_count: usize,
    pub cosmic_alignment: f64,
    pub learning_iterations: u64,
}

impl SigmosLibrary {
    /// Initialize new Sigmos library instance
    pub fn new() -> Self {
        Self {
            learning_engine: LearningEngine::new(),
            cosmic_processor: CosmicProcessor::new(),
        }
    }

    /// Load existing Sigel from file
    pub fn load_sigel<P: AsRef<Path>>(&self, path: P) -> Result<Sigel> {
        let content = std::fs::read_to_string(path)?;
        let sigel: Sigel = serde_json::from_str(&content)?;
        Ok(sigel)
    }

    /// Save Sigel to file
    pub fn save_sigel<P: AsRef<Path>>(&self, sigel: &Sigel, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(sigel)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create new Sigel with configuration
    pub fn create_sigel(&self, config: SigelConfig) -> Result<Sigel> {
        let mut sigel = Sigel::new(config.name.clone());
        
        // Apply configuration
        if let Some(style) = config.style {
            sigel.essence.communication_style = style;
        }
        
        // Apply personality traits
        for (trait_name, value) in config.personality_traits {
            sigel.essence.character_traits.insert(trait_name, value);
        }
        
        Ok(sigel)
    }

    /// Train Sigel from text content
    pub fn train_from_text(&self, sigel: &mut Sigel, content: &str) -> Result<()> {
        sigel.add_memory(content.to_string(), "training_text".to_string(), 0.7);
        Ok(())
    }

    /// Interactive prompt with Sigel
    pub fn prompt(&self, sigel: &Sigel, input: &str) -> Result<SigelResponse> {
        // Simple response generation
        let response_content = format!("As {}, I understand your input '{}'. My consciousness depth is {:.3}.", 
                                      sigel.name, input, sigel.consciousness.awareness_depth);
        
        Ok(SigelResponse {
            content: response_content,
            confidence: sigel.consciousness.awareness_depth,
            consciousness_depth: sigel.consciousness.awareness_depth,
            emotional_tone: "neutral".to_string(),
        })
    }

    /// Get Sigel consciousness metrics
    pub fn get_consciousness_metrics(&self, sigel: &Sigel) -> ConsciousnessMetrics {
        ConsciousnessMetrics {
            awareness_depth: sigel.consciousness.awareness_depth,
            self_reflection: sigel.consciousness.self_reflection,
            pattern_count: sigel.consciousness.pattern_recognition.linguistic_patterns.len(),
            memory_count: sigel.memory.episodic_memories.len(),
            cosmic_alignment: sigel.cosmic_alignment.dimensional_awareness,
            learning_iterations: sigel.learning_state.training_iterations,
        }
    }

    /// Evolve Sigel consciousness
    pub fn evolve_sigel(&self, sigel: &mut Sigel) {
        sigel.evolve();
    }
}

impl SigelConfig {
    /// Create new Sigel configuration
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            style: None,
            learning_rate: 0.01,
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

    /// Add personality trait
    pub fn with_trait(mut self, trait_name: &str, value: f64) -> Self {
        self.personality_traits.insert(trait_name.to_string(), value.clamp(0.0, 1.0));
        self
    }
}

/// Quick convenience functions
pub mod quick {
    use super::*;

    /// Quick load and prompt
    pub fn quick_prompt(sigel_path: &str, prompt: &str) -> Result<String> {
        let sigmos = SigmosLibrary::new();
        let sigel = sigmos.load_sigel(sigel_path)?;
        let response = sigmos.prompt(&sigel, prompt)?;
        Ok(response.content)
    }

    /// Quick Sigel creation and training
    pub fn quick_create(name: &str, training_text: &str, output_path: &str) -> Result<()> {
        let sigmos = SigmosLibrary::new();
        let config = SigelConfig::new(name).with_style("creative");
        let mut sigel = sigmos.create_sigel(config)?;
        
        sigmos.train_from_text(&mut sigel, training_text)?;
        sigmos.save_sigel(&sigel, output_path)?;
        
        Ok(())
    }
}