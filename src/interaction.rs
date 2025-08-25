use crate::sigel::*;
use crate::consciousness::ConsciousnessProcessor;
use crate::learning::LearningEngine;
use crate::cosmos::CosmicProcessor;
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

pub struct InteractionEngine {
    consciousness_processor: ConsciousnessProcessor,
    learning_engine: LearningEngine,
    cosmic_processor: CosmicProcessor,
    conversation_history: Vec<ConversationTurn>,
    context_memory: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub id: Uuid,
    pub timestamp: SystemTime,
    pub user_input: String,
    pub sigel_response: String,
    pub emotional_resonance: f64,
    pub context_tags: Vec<String>,
}

impl InteractionEngine {
    pub fn new() -> Self {
        Self {
            consciousness_processor: ConsciousnessProcessor,
            learning_engine: LearningEngine::new(),
            cosmic_processor: CosmicProcessor::new(),
            conversation_history: Vec::new(),
            context_memory: HashMap::new(),
        }
    }

    pub fn interact(&mut self, sigel: &mut Sigel, user_input: &str) -> String {
        // Pre-process the input through cosmic alignment
        let cosmic_resonance = self.cosmic_processor.apply_universal_constants(sigel, user_input);
        
        // Build conversation context
        let context = self.build_conversation_context(user_input);
        let enhanced_input = format!("{}|CONTEXT:{}|RESONANCE:{:.2}", user_input, context, cosmic_resonance);
        
        // Process through consciousness
        let raw_response = self.consciousness_processor.process_thought(sigel, &enhanced_input);
        
        // Apply cosmic inspiration if the topic warrants it
        let final_response = if self.should_apply_cosmic_perspective(user_input) {
            let cosmic_insight = self.cosmic_processor.cosmic_inspiration(sigel, user_input);
            self.merge_responses(&raw_response, &cosmic_insight, sigel)
        } else {
            self.humanize_response(&raw_response, sigel)
        };

        // Learn from this interaction
        self.learning_engine.continuous_learning(sigel, user_input, &final_response);
        
        // Record the conversation
        self.record_conversation_turn(user_input.to_string(), final_response.clone(), sigel);
        
        // Update context memory
        self.update_context_memory(user_input, &final_response);
        
        final_response
    }

    pub fn handle_special_commands(&mut self, sigel: &mut Sigel, command: &str) -> Option<String> {
        match command.trim().to_lowercase().as_str() {
            "/save" => {
                Some("Use /save <filename.sig> to save the current Sigel state".to_string())
            },
            "/reflect" => {
                let history: Vec<String> = self.conversation_history
                    .iter()
                    .map(|turn| format!("{} -> {}", turn.user_input, turn.sigel_response))
                    .collect();
                
                Some(self.consciousness_processor.self_reflection(sigel, &history))
            },
            "/status" => {
                Some(self.generate_sigel_status(sigel))
            },
            "/cosmic" => {
                self.cosmic_processor.align_with_cosmos(sigel);
                Some("Cosmic alignment performed. Dimensional awareness updated.".to_string())
            },
            "/verbose" => {
                Some("Verbose mode toggled. Next responses will include consciousness details.".to_string())
            },
            "/memory" => {
                Some(self.generate_memory_summary(sigel))
            },
            "/evolve" => {
                sigel.evolve();
                Some(format!("Sigel '{}' has evolved. Consciousness depth now: {:.3}", 
                    sigel.name, sigel.consciousness.awareness_depth))
            },
            "/help" => {
                Some(self.generate_help_text())
            },
            _ if command.starts_with("/save ") => {
                let filename = command.strip_prefix("/save ").unwrap_or("temp.sig");
                match crate::save_sigel_to_file(sigel, filename) {
                    Ok(()) => Some(format!("Sigel saved to {}", filename)),
                    Err(e) => Some(format!("Failed to save: {}", e)),
                }
            },
            _ => None
        }
    }

    fn build_conversation_context(&self, user_input: &str) -> String {
        let mut context_elements = Vec::new();
        
        // Recent conversation context
        if let Some(last_turn) = self.conversation_history.last() {
            context_elements.push(format!("PREV:{}", last_turn.user_input));
        }
        
        // Context memory relevance
        let user_words: Vec<&str> = user_input.split_whitespace().collect();
        let mut relevant_contexts: Vec<(String, f64)> = self.context_memory
            .iter()
            .filter_map(|(context, &relevance)| {
                let context_words: Vec<&str> = context.split_whitespace().collect();
                let overlap = user_words.iter()
                    .filter(|&&word| context_words.contains(&word))
                    .count();
                
                if overlap > 0 {
                    Some((context.clone(), relevance * overlap as f64))
                } else {
                    None
                }
            })
            .collect();
        
        relevant_contexts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (context, relevance) in relevant_contexts.into_iter().take(3) {
            context_elements.push(format!("REL:{:.2}:{}", relevance, context));
        }
        
        context_elements.join("|")
    }

    fn should_apply_cosmic_perspective(&self, input: &str) -> bool {
        let cosmic_keywords = [
            "universe", "cosmic", "space", "time", "existence", "consciousness", 
            "reality", "dimension", "infinite", "eternal", "meaning", "purpose",
            "stars", "galaxy", "quantum", "energy", "soul", "spirit", "divine"
        ];
        
        let input_lower = input.to_lowercase();
        cosmic_keywords.iter().any(|&keyword| input_lower.contains(keyword))
    }

    fn merge_responses(&self, consciousness_response: &str, cosmic_response: &str, sigel: &Sigel) -> String {
        let style_weight = match sigel.essence.communication_style {
            CommunicationStyle::Cosmic => 0.8,
            CommunicationStyle::Philosophical => 0.6,
            CommunicationStyle::Creative => 0.4,
            _ => 0.2,
        };
        
        if style_weight > 0.5 {
            format!("{}\n\n{}", cosmic_response, consciousness_response.replace("{", "").replace("}", ""))
        } else {
            format!("{}\n\n(Cosmic insight: {})", 
                consciousness_response.replace("{", "").replace("}", ""), 
                cosmic_response
            )
        }
    }

    fn humanize_response(&self, raw_response: &str, sigel: &Sigel) -> String {
        // Remove technical formatting and make more natural
        let cleaned = raw_response
            .replace("{deeply_perceived}", "")
            .replace("{moderately_perceived}", "")
            .replace("{surface_perceived}", "")
            .replace("{intuitive_insight}", "")
            .replace("{creatively}", "")
            .replace("{wisely}", "")
            .replace("context_understanding:", "")
            .replace("pattern:", "")
            .replace("semantic_context:", "")
            .replace("semantic:", ""); // Clean up any remaining semantic spam
        
        // Apply personality based on essence
        let personality_touch = match sigel.essence.communication_style {
            CommunicationStyle::Empathetic => {
                if cleaned.len() < 50 {
                    format!("I sense that {}. How does this resonate with you?", cleaned.trim())
                } else {
                    format!("{}. I'm here to understand your perspective.", cleaned)
                }
            },
            CommunicationStyle::Creative => {
                format!("Ah! {}. This sparks interesting possibilities!", cleaned.trim())
            },
            CommunicationStyle::Academic => {
                format!("From an analytical standpoint, {}.", cleaned.trim())
            },
            CommunicationStyle::Casual => {
                format!("Yeah, so {}. Pretty interesting stuff!", cleaned.trim())
            },
            _ => cleaned,
        };
        
        personality_touch.trim().to_string()
    }

    fn record_conversation_turn(&mut self, user_input: String, response: String, sigel: &Sigel) {
        let emotional_resonance = self.calculate_emotional_resonance(&user_input, &response);
        let context_tags = self.extract_context_tags(&user_input);
        
        let turn = ConversationTurn {
            id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            user_input,
            sigel_response: response,
            emotional_resonance,
            context_tags,
        };
        
        self.conversation_history.push(turn);
        
        // Keep conversation history manageable
        if self.conversation_history.len() > 100 {
            self.conversation_history.drain(0..20); // Remove oldest 20 entries
        }
        
        // Add to Sigel's episodic memory as well
        let memory_content = format!("User: {} | Sigel: {}", 
            self.conversation_history.last().unwrap().user_input,
            self.conversation_history.last().unwrap().sigel_response
        );
        
        let mut sigel_mut = sigel.clone();
        sigel_mut.add_memory(memory_content, "conversation".to_string(), emotional_resonance);
    }

    fn calculate_emotional_resonance(&self, input: &str, response: &str) -> f64 {
        // Simple emotional resonance calculation
        let positive_indicators = ["thank", "amazing", "wonderful", "great", "love", "yes", "perfect"];
        let negative_indicators = ["no", "wrong", "bad", "terrible", "hate", "never", "stop"];
        
        let input_lower = input.to_lowercase();
        let response_lower = response.to_lowercase();
        
        let mut resonance: f64 = 0.0;
        
        for indicator in positive_indicators {
            if input_lower.contains(indicator) || response_lower.contains(indicator) {
                resonance += 0.2;
            }
        }
        
        for indicator in negative_indicators {
            if input_lower.contains(indicator) || response_lower.contains(indicator) {
                resonance -= 0.15;
            }
        }
        
        // Length-based engagement
        if input.len() > 50 {
            resonance += 0.1; // Longer inputs suggest more engagement
        }
        
        if response.len() > 100 {
            resonance += 0.05; // Longer responses suggest deeper engagement
        }
        
        resonance.max(-1.0).min(1.0)
    }

    fn extract_context_tags(&self, input: &str) -> Vec<String> {
        let mut tags = Vec::new();
        
        // Simple topic extraction
        let topics = [
            ("technology", vec!["computer", "AI", "software", "code", "program"]),
            ("philosophy", vec!["meaning", "purpose", "existence", "consciousness", "reality"]),
            ("science", vec!["physics", "chemistry", "biology", "quantum", "universe"]),
            ("emotion", vec!["feel", "emotion", "happy", "sad", "angry", "love", "fear"]),
            ("question", vec!["what", "why", "how", "when", "where", "who"]),
        ];
        
        let input_lower = input.to_lowercase();
        
        for (topic, keywords) in topics {
            if keywords.iter().any(|&keyword| input_lower.contains(keyword)) {
                tags.push(topic.to_string());
            }
        }
        
        if input.ends_with('?') {
            tags.push("inquiry".to_string());
        }
        
        if input.contains('!') {
            tags.push("exclamation".to_string());
        }
        
        tags
    }

    fn update_context_memory(&mut self, input: &str, response: &str) {
        // Extract key phrases and update relevance
        let combined_text = format!("{} {}", input, response);
        let words: Vec<&str> = combined_text.split_whitespace().collect();
        
        // Create phrase contexts from word windows
        for window in words.windows(3) {
            let phrase = window.join(" ");
            *self.context_memory.entry(phrase).or_insert(0.0) += 0.1;
        }
        
        // Decay old context memory
        self.context_memory.values_mut().for_each(|relevance| {
            *relevance *= 0.99; // 1% decay per interaction
        });
        
        // Remove very low relevance contexts
        self.context_memory.retain(|_, &mut relevance| relevance > 0.01);
    }

    fn generate_sigel_status(&self, sigel: &Sigel) -> String {
        format!(
            "🧠 Sigel Status: {}\n\
             📊 Consciousness Depth: {:.3}\n\
             🎭 Communication Style: {:?}\n\
             📚 Vocabulary Size: {}\n\
             💭 Episodic Memories: {}\n\
             🌟 Training Iterations: {}\n\
             🌌 Dimensional Awareness: {:.2}\n\
             ⚡ Entropy Resistance: {:.2}\n\
             💬 Conversation Turns: {}",
            sigel.name,
            sigel.consciousness.awareness_depth,
            sigel.essence.communication_style,
            sigel.memory.semantic_knowledge.vocabulary.len(),
            sigel.memory.episodic_memories.len(),
            sigel.learning_state.training_iterations,
            sigel.cosmic_alignment.dimensional_awareness,
            sigel.cosmic_alignment.entropy_resistance,
            self.conversation_history.len()
        )
    }

    fn generate_memory_summary(&self, sigel: &Sigel) -> String {
        let recent_memories: Vec<_> = sigel.memory.episodic_memories
            .iter()
            .rev()
            .take(5)
            .collect();
        
        let mut summary = String::from("Recent Memories:\n");
        
        for (i, memory) in recent_memories.iter().enumerate() {
            summary.push_str(&format!(
                "{}. {} (emotional weight: {:.2})\n", 
                i + 1, 
                memory.content.chars().take(80).collect::<String>(),
                memory.emotional_weight
            ));
        }
        
        summary.push_str(&format!(
            "\nTop Vocabulary (frequency):\n"
        ));
        
        let mut word_freq: Vec<_> = sigel.memory.semantic_knowledge.vocabulary.iter().collect();
        word_freq.sort_by(|a, b| b.1.frequency.partial_cmp(&a.1.frequency).unwrap());
        
        for (word, knowledge) in word_freq.iter().take(10) {
            summary.push_str(&format!("{}: {:.1} ", word, knowledge.frequency));
        }
        
        summary
    }

    fn generate_help_text(&self) -> String {
        "Sigmos Prompt Commands:\n\
         /help - Show this help\n\
         /status - Show Sigel status\n\
         /memory - Show recent memories and vocabulary\n\
         /reflect - Sigel self-reflection on conversation\n\
         /cosmic - Perform cosmic alignment\n\
         /evolve - Evolve the Sigel consciousness\n\
         /save <filename> - Save current Sigel state\n\
         /verbose - Toggle verbose response mode\n\n\
         Simply type your message to interact with the Sigel!\n\
         The Sigel learns from every interaction and evolves over time.".to_string()
    }
}