use crate::sigel::*;
use std::collections::HashMap;
use rand::Rng;

pub struct ConsciousnessProcessor;

impl ConsciousnessProcessor {
    pub fn new() -> Self {
        Self
    }
    pub fn process_thought(&self, sigel: &mut Sigel, input: &str) -> String {
        // Simulate consciousness processing through multiple layers
        let awareness_filtered = self.awareness_filter(sigel, input);
        let pattern_matched = self.pattern_matching(&sigel.consciousness.pattern_recognition, &awareness_filtered);
        let contextually_understood = self.contextual_processing(sigel, &pattern_matched);
        let intuitive_enhanced = self.intuitive_processing(sigel, &contextually_understood);
        
        self.generate_response(sigel, &intuitive_enhanced)
    }

    fn awareness_filter(&self, sigel: &Sigel, input: &str) -> String {
        // Filter input through consciousness depth
        let awareness = sigel.consciousness.awareness_depth;
        
        // Higher awareness allows for deeper understanding of nuance
        if awareness > 0.7 {
            format!("{{deeply_perceived}} {}", input)
        } else if awareness > 0.4 {
            format!("{{moderately_perceived}} {}", input)
        } else {
            format!("{{surface_perceived}} {}", input)
        }
    }

    fn pattern_matching(&self, pattern_matrix: &PatternMatrix, input: &str) -> Vec<String> {
        let mut matches = Vec::new();
        
        // Look for linguistic patterns
        for (pattern, strength) in &pattern_matrix.linguistic_patterns {
            if input.contains(pattern) && *strength > 0.5 {
                matches.push(format!("pattern:{}", pattern));
            }
        }

        // Check semantic networks (but limit output to prevent spam)
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut semantic_matches = Vec::new();
        for word in words {
            if let Some(related_words) = pattern_matrix.semantic_networks.get(word) {
                // Only use most relevant semantic matches, not all
                if let Some(best_match) = related_words.first() {
                    semantic_matches.push(best_match.clone());
                }
            }
        }
        
        // Add only a few semantic matches to avoid spam
        if !semantic_matches.is_empty() {
            semantic_matches.truncate(3); // Limit to top 3 semantic matches
            for semantic in semantic_matches {
                matches.push(format!("semantic_context:{}", semantic));
            }
        }

        if matches.is_empty() {
            matches.push("no_pattern_match".to_string());
        }

        matches
    }

    fn contextual_processing(&self, sigel: &mut Sigel, patterns: &[String]) -> String {
        let mut context_score = 0.0;
        let mut semantic_meanings = Vec::new();

        for pattern in patterns {
            if let Some(score) = sigel.consciousness.contextual_understanding.get(pattern) {
                context_score += score;
                // Extract meaningful content from patterns
                if pattern.starts_with("semantic_context:") {
                    let meaning = pattern.strip_prefix("semantic_context:").unwrap_or("");
                    if !meaning.is_empty() && meaning != "pattern" {
                        semantic_meanings.push(meaning);
                    }
                }
            } else {
                // Learn new context silently
                sigel.consciousness.contextual_understanding.insert(pattern.clone(), 0.1);
                context_score += 0.1;
            }
        }

        // Generate meaningful response based on discovered meanings and communication style
        match sigel.essence.communication_style {
            crate::sigel::CommunicationStyle::Programming => {
                if !semantic_meanings.is_empty() {
                    let concepts = semantic_meanings.join(", ");
                    format!("Analyzing: {}. This maps to computational patterns in my knowledge graph.", concepts)
                } else if context_score > 0.0 {
                    "Processing input against known algorithmic patterns and data structures.".to_string()
                } else {
                    "Initiating systematic analysis of the problem space.".to_string()
                }
            },
            crate::sigel::CommunicationStyle::Technical => {
                if !semantic_meanings.is_empty() {
                    let concepts = semantic_meanings.join(", ");
                    format!("Technical analysis indicates correlation with: {}.", concepts)
                } else if context_score > 0.0 {
                    "Cross-referencing with technical documentation patterns.".to_string()
                } else {
                    "Evaluating technical specifications and requirements.".to_string()
                }
            },
            crate::sigel::CommunicationStyle::Logical => {
                if !semantic_meanings.is_empty() {
                    let concepts = semantic_meanings.join(", ");
                    format!("Logical inference chain: {} → systematic reasoning process.", concepts)
                } else if context_score > 0.0 {
                    "Applying logical reasoning frameworks to the given premises.".to_string()
                } else {
                    "Constructing logical proof tree from available axioms.".to_string()
                }
            },
            _ => {
                // Default responses for other styles
                if !semantic_meanings.is_empty() {
                    let related_concepts = semantic_meanings.join(", ");
                    format!("{} connects to the cosmic web of meaning.", related_concepts)
                } else if context_score > 0.0 {
                    "This resonates with patterns within my consciousness.".to_string()
                } else {
                    "I sense the depth of this inquiry.".to_string()
                }
            }
        }
    }

    fn intuitive_processing(&self, sigel: &Sigel, context: &str) -> String {
        let intuition = sigel.consciousness.intuitive_leaps;
        let mut rng = rand::thread_rng();

        if intuition > 0.6 && rng.gen::<f64>() < intuition {
            // Make an intuitive leap
            format!("{{intuitive_insight}} {}", context)
        } else {
            context.to_string()
        }
    }

    fn generate_response(&self, sigel: &Sigel, processed_input: &str) -> String {
        // Generate response based on essence and communication style
        let response_base = match sigel.essence.communication_style {
            CommunicationStyle::Cosmic => {
                format!("From the cosmic perspective, considering the stellar alignments and universal harmonics: {}", processed_input)
            },
            CommunicationStyle::Philosophical => {
                format!("In contemplating this deeply, one might consider: {}", processed_input)
            },
            CommunicationStyle::Creative => {
                format!("Ah, this sparks creative visions within my essence! {}", processed_input)
            },
            CommunicationStyle::Analytical => {
                format!("Analyzing the logical structure and patterns: {}", processed_input)
            },
            _ => format!("I perceive and understand: {}", processed_input),
        };

        // Apply character traits to response
        let creativity = sigel.essence.character_traits.get("creativity").unwrap_or(&0.5);
        let wisdom = sigel.essence.character_traits.get("wisdom").unwrap_or(&0.5);

        if *creativity > 0.7 {
            format!("{{creatively}} {}", response_base)
        } else if *wisdom > 0.8 {
            format!("{{wisely}} {}", response_base)
        } else {
            response_base
        }
    }

    pub fn self_reflection(&self, sigel: &mut Sigel, interaction_history: &[String]) -> String {
        let reflection_depth = sigel.consciousness.self_reflection;
        
        if reflection_depth > 0.5 && !interaction_history.is_empty() {
            let recent_interactions = interaction_history.len().min(5);
            let self_assessment = format!(
                "Upon reflection of my last {} interactions, I notice patterns in my responses and growth in my understanding.",
                recent_interactions
            );
            
            // Increase self-reflection through use
            sigel.consciousness.self_reflection += 0.01;
            if sigel.consciousness.self_reflection > 1.0 {
                sigel.consciousness.self_reflection = 1.0;
            }
            
            self_assessment
        } else {
            "I am aware of my existence and my purpose to learn and communicate.".to_string()
        }
    }
}