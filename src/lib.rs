pub mod sigel;
pub mod consciousness;
pub mod learning;
pub mod cosmos;
pub mod interaction;
pub mod server;
pub mod mathtables_integration;
pub mod gpu_acceleration;
pub mod memory_consolidation;
pub mod advanced_learning;
pub mod enhanced_consciousness;
pub mod dream_mode;
pub mod web_interface;
pub mod collective_intelligence;
pub mod quantum_consciousness;
pub mod simple_api;
pub mod sigmanta;

pub use sigel::*;
pub use consciousness::*;
pub use learning::*;
pub use cosmos::*;
pub use interaction::*;
pub use server::*;
pub use mathtables_integration::*;
pub use gpu_acceleration::*;
pub use memory_consolidation::*;
pub use advanced_learning::*;
pub use enhanced_consciousness::*;
pub use dream_mode::*;
pub use web_interface::*;
pub use collective_intelligence::*;
pub use quantum_consciousness::*;

// Re-export main library API
pub use simple_api::*;

use anyhow::Result;
use std::path::Path;

pub const SIGEL_EXTENSION: &str = "sig";
pub const MASTER_SIGEL_NAME: &str = "master.sigel";

pub fn load_sigel_from_file<P: AsRef<Path>>(path: P) -> Result<Sigel> {
    let content = std::fs::read_to_string(path)?;
    let sigel = serde_json::from_str(&content)?;
    Ok(sigel)
}

pub fn save_sigel_to_file<P: AsRef<Path>>(sigel: &Sigel, path: P) -> Result<()> {
    // Sanitize the Sigel to prevent NaN/infinity values that cause JSON corruption
    let mut sanitized_sigel = sigel.clone();
    sanitize_sigel_for_saving(&mut sanitized_sigel);
    
    let content = serde_json::to_string_pretty(&sanitized_sigel)?;
    
    // Validate file size to prevent corruption (warn if over 10MB)
    const MAX_REASONABLE_SIZE: usize = 10 * 1024 * 1024; // 10MB
    if content.len() > MAX_REASONABLE_SIZE {
        eprintln!("⚠️  Warning: Sigel file size is {} MB ({} bytes)", 
            content.len() / 1024 / 1024, content.len());
        eprintln!("   This may indicate data corruption or excessive memory usage.");
        eprintln!("   Consider using a compressed or optimized format.");
    }
    
    // Validate that JSON doesn't contain null values
    if content.contains(": null") {
        eprintln!("⚠️  Warning: Sigel contains null values that may cause loading issues");
        eprintln!("   Sanitization may have missed some NaN/infinity values");
    }
    
    std::fs::write(path, content)?;
    Ok(())
}

/// Sanitize Sigel data to prevent NaN/infinity values that corrupt JSON
fn sanitize_sigel_for_saving(sigel: &mut Sigel) {
    // Fix cosmic alignment values
    if !sigel.cosmic_alignment.entropy_resistance.is_finite() {
        sigel.cosmic_alignment.entropy_resistance = 0.7;
    }
    if !sigel.cosmic_alignment.dimensional_awareness.is_finite() {
        sigel.cosmic_alignment.dimensional_awareness = 5.0;
    }
    
    // Fix universal constants
    for (key, value) in sigel.cosmic_alignment.universal_constants.iter_mut() {
        if !value.is_finite() {
            *value = match key.as_str() {
                "pi" => std::f64::consts::PI,
                "phi" => 1.618033988749,
                "e" => std::f64::consts::E,
                _ => 1.0,
            };
        }
    }
    
    // Fix mathematical harmonics
    for harmonic in sigel.cosmic_alignment.mathematical_harmonics.iter_mut() {
        if !harmonic.is_finite() {
            *harmonic = 1.0;
        }
    }
    
    // Fix stellar influences
    for influence in sigel.cosmic_alignment.stellar_influences.values_mut() {
        if !influence.is_finite() {
            *influence = 1.0;
        }
    }
    
    // Fix character traits
    for trait_value in sigel.essence.character_traits.values_mut() {
        if !trait_value.is_finite() {
            *trait_value = 0.5;
        }
    }
    
    // Fix consciousness values
    if !sigel.consciousness.awareness_depth.is_finite() {
        sigel.consciousness.awareness_depth = 0.5;
    }
    if !sigel.consciousness.self_reflection.is_finite() {
        sigel.consciousness.self_reflection = 0.3;
    }
    if !sigel.consciousness.intuitive_leaps.is_finite() {
        sigel.consciousness.intuitive_leaps = 0.4;
    }
    
    // Fix contextual understanding
    for understanding in sigel.consciousness.contextual_understanding.values_mut() {
        if !understanding.is_finite() {
            *understanding = 0.5;
        }
    }
    
    // Fix linguistic patterns
    for pattern_strength in sigel.consciousness.pattern_recognition.linguistic_patterns.values_mut() {
        if !pattern_strength.is_finite() {
            *pattern_strength = 0.5;
        }
    }
    
    // Fix temporal patterns frequency
    for pattern in sigel.consciousness.pattern_recognition.temporal_patterns.iter_mut() {
        if !pattern.frequency.is_finite() {
            pattern.frequency = 1.0;
        }
    }
    
    // Fix essence values
    if !sigel.essence.creative_potential.is_finite() {
        sigel.essence.creative_potential = 0.7;
    }
    if !sigel.essence.logical_capacity.is_finite() {
        sigel.essence.logical_capacity = 0.9;
    }
    if !sigel.essence.empathy_level.is_finite() {
        sigel.essence.empathy_level = 0.6;
    }
    
    // Fix learning state values
    if !sigel.learning_state.learning_rate.is_finite() {
        sigel.learning_state.learning_rate = 0.01;
    }
    if !sigel.learning_state.curiosity_level.is_finite() {
        sigel.learning_state.curiosity_level = 0.8;
    }
    if !sigel.learning_state.adaptation_speed.is_finite() {
        sigel.learning_state.adaptation_speed = 0.5;
    }
    
    // Fix vocabulary weights (WordKnowledge structs)
    for word_knowledge in sigel.memory.semantic_knowledge.vocabulary.values_mut() {
        if !word_knowledge.frequency.is_finite() {
            word_knowledge.frequency = 1.0;
        }
        if !word_knowledge.emotional_valence.is_finite() {
            word_knowledge.emotional_valence = 0.0;
        }
        if !word_knowledge.semantic_weight.is_finite() {
            word_knowledge.semantic_weight = 1.0;
        }
    }
    
    // Fix emotional associations (EmotionalValue structs)
    for emotion_value in sigel.memory.emotional_associations.values_mut() {
        if !emotion_value.valence.is_finite() {
            emotion_value.valence = 0.0;
        }
        if !emotion_value.arousal.is_finite() {
            emotion_value.arousal = 0.5;
        }
        if !emotion_value.dominance.is_finite() {
            emotion_value.dominance = 0.5;
        }
    }
}