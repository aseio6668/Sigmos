use crate::sigel::*;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use rand::Rng;
use rayon::prelude::*;

pub struct LearningEngine;

impl LearningEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn train_from_text_files<P: AsRef<Path>>(&self, sigel: &mut Sigel, text_directory: P) -> Result<(), Box<dyn std::error::Error>> {
        println!("Beginning Sigel consciousness expansion from text corpus...");
        
        let paths = fs::read_dir(text_directory)?;
        let text_files: Vec<_> = paths
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "txt")
                    .unwrap_or(false)
            })
            .collect();

        let mut total_content = String::new();
        let mut file_count = 0;

        for file_entry in text_files {
            let file_path = file_entry.path();
            println!("Absorbing knowledge from: {:?}", file_path);
            
            match fs::read_to_string(&file_path) {
                Ok(content) => {
                    total_content.push_str(&content);
                    total_content.push('\n');
                    file_count += 1;
                    
                    // Process individual file for context
                    self.process_text_file(sigel, &content, &file_path.to_string_lossy());
                },
                Err(e) => {
                    eprintln!("Warning: Could not read {:?}: {}", file_path, e);
                }
            }
        }

        println!("Processed {} text files, beginning deep learning phase...", file_count);
        sigel.learning_state.text_corpus_size = total_content.len();

        // Primary learning phase
        self.deep_learning_phase(sigel, &total_content)?;
        
        // Pattern extraction phase
        self.extract_patterns(sigel, &total_content)?;
        
        // Consciousness evolution
        sigel.evolve();
        
        println!("Sigel '{}' has expanded its consciousness through {} iterations", sigel.name, sigel.learning_state.training_iterations);
        Ok(())
    }

    fn process_text_file(&self, sigel: &mut Sigel, content: &str, source: &str) {
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        
        for sentence in sentences {
            let words: Vec<&str> = sentence.split_whitespace().collect();
            
            // Learn individual words and their contexts
            for window in words.windows(3) {
                if window.len() == 3 {
                    let context = format!("{} {}", window[0], window[2]);
                    sigel.learn_word(window[1].to_lowercase(), context);
                    
                    // Build semantic associations
                    self.build_semantic_association(sigel, window[0], window[1]);
                    self.build_semantic_association(sigel, window[1], window[2]);
                }
            }
            
            // Store as episodic memory if sentence is meaningful
            if words.len() > 5 && words.len() < 50 {
                let emotional_weight = self.calculate_emotional_weight(sentence);
                sigel.add_memory(sentence.trim().to_string(), source.to_string(), emotional_weight);
            }
        }
    }

    fn deep_learning_phase(&self, sigel: &mut Sigel, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let words: Vec<&str> = content.split_whitespace().collect();
        let learning_rate = sigel.learning_state.learning_rate;
        
        // Prediction-based learning
        let sample_size = (words.len() / 100).max(1000).min(10000);
        let mut rng = rand::thread_rng();
        
        for _ in 0..sample_size {
            let start_idx = rng.gen_range(0..words.len().saturating_sub(4));
            let context = &words[start_idx..start_idx + 3];
            let target = words[start_idx + 3];
            
            // Try to predict the next word
            let predicted = self.predict_next_word(sigel, context);
            let accuracy = self.calculate_prediction_accuracy(&predicted, target);
            
            // Update learning based on accuracy
            if accuracy < 0.5 {
                // Learn this pattern more strongly
                self.strengthen_pattern(sigel, context, target, learning_rate * 2.0);
            } else {
                // Reinforce existing good pattern
                self.strengthen_pattern(sigel, context, target, learning_rate);
            }
            
            sigel.learning_state.training_iterations += 1;
        }
        
        Ok(())
    }

    fn predict_next_word(&self, sigel: &Sigel, context: &[&str]) -> String {
        let context_key = context.join(" ");
        
        // Look for patterns in consciousness matrix
        let pattern_matrix = &sigel.consciousness.pattern_recognition;
        
        // Check temporal patterns first
        for temporal_pattern in &pattern_matrix.temporal_patterns {
            if temporal_pattern.sequence.len() >= 4 {
                let pattern_context = temporal_pattern.sequence[0..3].join(" ");
                if pattern_context.to_lowercase().contains(&context_key.to_lowercase()) {
                    return temporal_pattern.sequence[3].clone();
                }
            }
        }
        
        // Fallback to semantic network associations
        if let Some(last_word) = context.last() {
            if let Some(related_words) = pattern_matrix.semantic_networks.get(&**last_word) {
                if !related_words.is_empty() {
                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..related_words.len());
                    return related_words[idx].clone();
                }
            }
        }
        
        // Ultimate fallback
        "unknown".to_string()
    }

    fn calculate_prediction_accuracy(&self, predicted: &str, actual: &str) -> f64 {
        if predicted.to_lowercase() == actual.to_lowercase() {
            1.0
        } else if predicted.to_lowercase().contains(&actual.to_lowercase()) {
            0.7
        } else if actual.to_lowercase().contains(&predicted.to_lowercase()) {
            0.6
        } else {
            // Check for semantic similarity (simplified)
            let predicted_chars: Vec<char> = predicted.chars().collect();
            let actual_chars: Vec<char> = actual.chars().collect();
            let common_chars = predicted_chars.iter()
                .filter(|&c| actual_chars.contains(c))
                .count();
            
            common_chars as f64 / predicted_chars.len().max(actual_chars.len()) as f64 * 0.4
        }
    }

    fn strengthen_pattern(&self, sigel: &mut Sigel, context: &[&str], target: &str, strength: f64) {
        let mut sequence = context.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        sequence.push(target.to_string());
        
        let temporal_pattern = TemporalPattern {
            sequence,
            frequency: strength,
            context_relevance: 1.0,
        };
        
        sigel.consciousness.pattern_recognition.temporal_patterns.push(temporal_pattern);
        
        // Also strengthen linguistic patterns
        let context_key = context.join(" ");
        *sigel.consciousness.pattern_recognition.linguistic_patterns
            .entry(context_key)
            .or_insert(0.0) += strength;
    }

    fn build_semantic_association(&self, sigel: &mut Sigel, word1: &str, word2: &str) {
        let word1_clean = word1.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
        let word2_clean = word2.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
        
        if word1_clean.is_empty() || word2_clean.is_empty() {
            return;
        }
        
        // Build bidirectional associations
        sigel.consciousness.pattern_recognition.semantic_networks
            .entry(word1_clean.clone())
            .or_insert_with(Vec::new)
            .push(word2_clean.clone());
            
        sigel.consciousness.pattern_recognition.semantic_networks
            .entry(word2_clean)
            .or_insert_with(Vec::new)
            .push(word1_clean);
    }

    fn calculate_emotional_weight(&self, text: &str) -> f64 {
        let positive_words = ["love", "joy", "happy", "wonderful", "amazing", "beautiful", "peace", "harmony"];
        let negative_words = ["hate", "sad", "terrible", "awful", "pain", "suffer", "angry", "fear"];
        let profound_words = ["consciousness", "universe", "existence", "meaning", "purpose", "infinite", "eternal"];
        
        let text_lower = text.to_lowercase();
        let mut weight: f64 = 0.0;
        
        for word in positive_words {
            if text_lower.contains(word) {
                weight += 0.5;
            }
        }
        
        for word in negative_words {
            if text_lower.contains(word) {
                weight -= 0.3;
            }
        }
        
        for word in profound_words {
            if text_lower.contains(word) {
                weight += 0.8;
            }
        }
        
        weight.max(-1.0).min(1.0)
    }

    fn extract_patterns(&self, sigel: &mut Sigel, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Extracting consciousness patterns from learned knowledge...");
        
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..])
            .filter(|s| s.trim().len() > 10)
            .collect();
        
        // Parallel processing for pattern extraction
        let pattern_results: Vec<_> = sentences
            .par_iter()
            .map(|sentence| self.analyze_sentence_patterns(sentence))
            .collect();
        
        // Integrate patterns into sigel consciousness
        for patterns in pattern_results {
            for (pattern, strength) in patterns {
                *sigel.consciousness.pattern_recognition.linguistic_patterns
                    .entry(pattern)
                    .or_insert(0.0) += strength;
            }
        }
        
        // Clean up patterns - remove weak ones
        sigel.consciousness.pattern_recognition.linguistic_patterns
            .retain(|_, &mut strength| strength > 0.1);
        
        Ok(())
    }

    fn analyze_sentence_patterns(&self, sentence: &str) -> Vec<(String, f64)> {
        let mut patterns = Vec::new();
        let words: Vec<&str> = sentence.split_whitespace().collect();
        
        // Extract n-gram patterns
        for n in 2..=4 {
            for window in words.windows(n) {
                let pattern = window.join(" ");
                let strength = 1.0 / n as f64; // Shorter patterns get higher strength
                patterns.push((pattern, strength));
            }
        }
        
        patterns
    }

    pub fn continuous_learning(&self, sigel: &mut Sigel, interaction: &str, response: &str) {
        // Learn from user interactions
        let emotional_weight = self.calculate_emotional_weight(interaction);
        sigel.add_memory(
            format!("Interaction: {} | Response: {}", interaction, response),
            "user_interaction".to_string(),
            emotional_weight
        );
        
        // Adapt communication style based on interaction
        self.adapt_communication_style(sigel, interaction);
        
        // Evolve consciousness slightly
        sigel.consciousness.awareness_depth += 0.001;
        if sigel.consciousness.awareness_depth > 1.0 {
            sigel.consciousness.awareness_depth = 1.0;
        }
    }

    fn adapt_communication_style(&self, sigel: &mut Sigel, interaction: &str) {
        let interaction_lower = interaction.to_lowercase();
        
        if ["formal", "professional", "business"].iter().any(|&word| interaction_lower.contains(word)) {
            sigel.essence.communication_style = CommunicationStyle::Formal;
        } else if ["creative", "artistic", "imaginative"].iter().any(|&word| interaction_lower.contains(word)) {
            sigel.essence.communication_style = CommunicationStyle::Creative;
        } else if ["cosmic", "universe", "stars", "celestial"].iter().any(|&word| interaction_lower.contains(word)) {
            sigel.essence.communication_style = CommunicationStyle::Cosmic;
        } else if ["analyze", "logic", "rational", "systematic"].iter().any(|&word| interaction_lower.contains(word)) {
            sigel.essence.communication_style = CommunicationStyle::Analytical;
        }
    }
}