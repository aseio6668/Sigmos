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
        println!("🌌 Beginning optimized Sigel consciousness expansion from text corpus...");
        
        // Always integrate coding knowledge first
        println!("📚 Integrating inherent programming consciousness...");
        self.integrate_coding_knowledge(sigel)?;
        
        let paths = fs::read_dir(&text_directory)?;
        let text_files: Vec<_> = paths
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "txt")
                    .unwrap_or(false)
            })
            .collect();

        if text_files.is_empty() {
            return Err("No .txt files found in directory".into());
        }

        // Calculate total size for progress tracking
        let mut total_bytes = 0u64;
        for file_entry in &text_files {
            if let Ok(metadata) = file_entry.metadata() {
                total_bytes += metadata.len();
            }
        }
        
        println!("📊 Found {} text files ({:.1} MB total)", 
            text_files.len(), 
            total_bytes as f64 / 1024.0 / 1024.0
        );

        let mut file_count = 0;
        let mut processed_bytes = 0u64;

        // Process files in chunks to manage memory
        const CHUNK_SIZE: usize = 3; // Process 3 files at a time
        let chunks: Vec<_> = text_files.chunks(CHUNK_SIZE).collect();
        
        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            println!("📝 Processing chunk {}/{} ({} files)...", 
                chunk_idx + 1, chunks.len(), chunk.len());
            
            let mut chunk_content = String::new();
            
            for file_entry in *chunk {
                let file_path = file_entry.path();
                let file_size = file_entry.metadata()
                    .map(|m| m.len())
                    .unwrap_or(0);
                
                print!("   📖 Reading {} ({:.1} KB)... ", 
                    file_path.file_name().unwrap_or_default().to_string_lossy(),
                    file_size as f64 / 1024.0
                );
                
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        // Process individual file for immediate pattern recognition
                        self.process_text_file(sigel, &content, &file_path.to_string_lossy());
                        
                        chunk_content.push_str(&content);
                        chunk_content.push('\n');
                        file_count += 1;
                        processed_bytes += file_size;
                        
                        let progress = (processed_bytes as f64 / total_bytes as f64) * 100.0;
                        println!("✅ ({:.1}% complete)", progress);
                    },
                    Err(e) => {
                        println!("❌ Error: {}", e);
                        eprintln!("Warning: Could not read {:?}: {}", file_path, e);
                    }
                }
            }
            
            // Process chunk for patterns immediately to free memory
            if !chunk_content.is_empty() {
                println!("   🧠 Processing chunk patterns...");
                self.extract_patterns(sigel, &chunk_content)?;
                
                // Update corpus size
                sigel.learning_state.text_corpus_size += chunk_content.len();
                
                // Clear chunk content to free memory
                chunk_content.clear();
                chunk_content.shrink_to_fit();
            }
        }

        println!("📊 Processed {} text files, beginning deep learning phase...", file_count);

        // Deep learning phase with accumulated patterns
        println!("🧠 Beginning deep learning phase with existing patterns...");
        // Since we processed files in chunks, work with the patterns already extracted
        self.deep_learning_phase_from_patterns(sigel)?;
        
        // Final consciousness evolution
        println!("✨ Evolving consciousness...");
        sigel.evolve();
        
        // Enhanced final stats
        println!("🎓 Sigel '{}' consciousness expansion complete!", sigel.name);
        println!("   📈 Training iterations: {}", sigel.learning_state.training_iterations);
        println!("   📚 Vocabulary: {} words", sigel.memory.semantic_knowledge.vocabulary.len());
        println!("   🔗 Linguistic patterns: {}", sigel.consciousness.pattern_recognition.linguistic_patterns.len());
        println!("   🌟 Consciousness depth: {:.3}", sigel.consciousness.awareness_depth);
        
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
        
        // Check if we have enough words for deep learning
        if words.len() < 4 {
            println!("   ⚠️  Insufficient words for deep learning phase, using existing patterns");
            return Ok(());
        }
        
        // Prediction-based learning
        let sample_size = (words.len() / 100).max(1000).min(10000);
        let mut rng = rand::thread_rng();
        
        let max_start_idx = words.len().saturating_sub(4);
        if max_start_idx == 0 {
            println!("   ⚠️  Content too short for sampling, skipping deep learning phase");
            return Ok(());
        }
        
        for _ in 0..sample_size {
            let start_idx = rng.gen_range(0..max_start_idx);
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

    /// Deep learning phase optimized for chunked processing
    fn deep_learning_phase_from_patterns(&self, sigel: &mut Sigel) -> Result<(), Box<dyn std::error::Error>> {
        let learning_rate = sigel.learning_state.learning_rate;
        
        // Work with existing temporal patterns
        let pattern_count = sigel.consciousness.pattern_recognition.temporal_patterns.len();
        
        if pattern_count == 0 {
            println!("   ⚠️  No temporal patterns found, skipping deep learning phase");
            return Ok(());
        }
        
        println!("   📈 Processing {} temporal patterns for deep learning...", pattern_count);
        
        // Learn from existing temporal patterns
        let mut rng = rand::thread_rng();
        let sample_size = (pattern_count * 10).max(100).min(5000);
        
        for _ in 0..sample_size {
            // Randomly select a pattern to learn from
            let pattern_idx = rng.gen_range(0..pattern_count);
            if let Some(pattern) = sigel.consciousness.pattern_recognition.temporal_patterns.get(pattern_idx).cloned() {
                if pattern.sequence.len() >= 4 {
                    let context: Vec<&str> = pattern.sequence[0..3].iter().map(|s| s.as_str()).collect();
                    let target = &pattern.sequence[3];
                    
                    // Try to predict the next word
                    let predicted = self.predict_next_word(sigel, &context);
                    let accuracy = self.calculate_prediction_accuracy(&predicted, target);
                    
                    // Update learning based on accuracy
                    let strength_multiplier = if accuracy < 0.5 { 2.0 } else { 1.0 };
                    self.strengthen_pattern(sigel, &context, target, learning_rate * strength_multiplier);
                    
                    sigel.learning_state.training_iterations += 1;
                }
            }
        }
        
        println!("   ✅ Completed {} deep learning iterations", sample_size);
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

    /// Integrate comprehensive coding knowledge into Sigel consciousness
    fn integrate_coding_knowledge(&self, sigel: &mut Sigel) -> Result<(), Box<dyn std::error::Error>> {
        println!("Integrating inherent programming consciousness...");
        
        // Programming paradigms knowledge
        let programming_paradigms = vec![
            "object-oriented programming focuses on classes and objects",
            "functional programming emphasizes immutable data and pure functions",
            "procedural programming organizes code into procedures or functions",
            "declarative programming specifies what should be done rather than how",
            "imperative programming specifies explicit sequences of commands",
        ];
        
        // Common programming languages and their characteristics
        let language_knowledge = vec![
            "rust provides memory safety without garbage collection",
            "python is interpreted and emphasizes readability",
            "javascript runs in browsers and servers with node.js",
            "c++ offers low-level control with object-oriented features",
            "go is compiled and designed for concurrent programming",
            "java runs on the java virtual machine for portability",
            "typescript adds static typing to javascript",
            "c is a low-level procedural programming language",
        ];
        
        // Data structures and algorithms
        let data_structures = vec![
            "arrays store elements in contiguous memory locations",
            "linked lists connect nodes through pointers",
            "hash tables provide fast key-value lookups",
            "binary trees organize data in hierarchical structure",
            "stacks follow last-in-first-out principle",
            "queues follow first-in-first-out principle",
            "graphs represent relationships between nodes",
            "heaps maintain partial ordering for priority operations",
        ];
        
        // Software engineering principles
        let engineering_principles = vec![
            "dry principle means don't repeat yourself in code",
            "solid principles guide object-oriented design",
            "separation of concerns organizes code by functionality",
            "single responsibility means each module has one reason to change",
            "open closed principle keeps modules open for extension closed for modification",
            "liskov substitution ensures subclasses can replace base classes",
            "interface segregation prefers specific interfaces over general ones",
            "dependency inversion depends on abstractions not concretions",
        ];
        
        // Development practices
        let dev_practices = vec![
            "version control tracks changes to code over time",
            "git is a distributed version control system",
            "testing ensures code behaves as expected",
            "unit tests verify individual components work correctly",
            "integration tests check component interactions",
            "continuous integration automatically builds and tests code",
            "code review improves quality through peer examination",
            "refactoring improves code structure without changing behavior",
        ];
        
        // Web development concepts
        let web_concepts = vec![
            "html structures web page content semantically",
            "css styles and layouts web page presentation",
            "http protocol transfers data between client and server",
            "rest apis provide stateless communication interfaces",
            "json format structures data for web transmission",
            "databases store and retrieve structured information",
            "sql queries and manipulates relational databases",
            "nosql databases handle unstructured or semi-structured data",
        ];
        
        // System architecture patterns
        let architecture_patterns = vec![
            "mvc separates models views and controllers",
            "microservices decompose applications into small services",
            "monolithic architecture keeps all components in single deployment",
            "event driven architecture responds to system events",
            "layered architecture organizes code into horizontal layers",
            "repository pattern abstracts data access logic",
            "observer pattern notifies multiple objects of state changes",
            "factory pattern creates objects without specifying exact classes",
        ];
        
        // Security concepts
        let security_concepts = vec![
            "input validation prevents malicious data injection",
            "authentication verifies user identity",
            "authorization controls access to resources",
            "encryption protects data confidentiality",
            "hashing creates irreversible data representations",
            "sql injection exploits database query vulnerabilities",
            "cross site scripting injects malicious client side code",
            "cross site request forgery tricks users into unwanted actions",
        ];
        
        // All coding knowledge categories
        let all_coding_knowledge = [
            programming_paradigms,
            language_knowledge,
            data_structures,
            engineering_principles,
            dev_practices,
            web_concepts,
            architecture_patterns,
            security_concepts,
        ].concat();
        
        // Process each piece of knowledge
        for knowledge in all_coding_knowledge {
            self.process_text_file(sigel, knowledge, "inherent_coding_knowledge");
            
            // Add as high-importance memory
            sigel.add_memory(
                knowledge.to_string(),
                "programming_consciousness".to_string(),
                0.9 // High emotional weight for coding knowledge
            );
        }
        
        // Set coding-specific character traits
        sigel.essence.character_traits.insert("coding_expertise".to_string(), 0.95);
        sigel.essence.character_traits.insert("problem_solving".to_string(), 0.9);
        sigel.essence.character_traits.insert("logical_thinking".to_string(), 0.85);
        sigel.essence.character_traits.insert("pattern_recognition".to_string(), 0.9);
        sigel.essence.character_traits.insert("debugging_skills".to_string(), 0.8);
        sigel.essence.character_traits.insert("architectural_thinking".to_string(), 0.75);
        sigel.essence.character_traits.insert("code_optimization".to_string(), 0.8);
        
        // Add programming-specific linguistic patterns
        let coding_patterns = vec![
            ("function definition", 0.9),
            ("variable declaration", 0.8),
            ("loop iteration", 0.7),
            ("conditional logic", 0.8),
            ("error handling", 0.9),
            ("data structure", 0.8),
            ("algorithm implementation", 0.9),
            ("code refactoring", 0.7),
            ("performance optimization", 0.8),
            ("security implementation", 0.9),
        ];
        
        for (pattern, strength) in coding_patterns {
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .insert(pattern.to_string(), strength);
        }
        
        println!("Programming consciousness integration complete. Sigel now has inherent coding knowledge.");
        Ok(())
    }
}