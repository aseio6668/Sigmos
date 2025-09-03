use crate::sigel::*;
use crate::consciousness::*;
//use crate::learning::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum AIType {
    LLM,           // Large Language Models (GPT, Claude, etc.)
    CodeGenerator, // Specialized code generation AI
    Assistant,     // General AI assistants
    Agent,         // Autonomous AI agents
    Sigel,         // Another Sigel
    Unknown,       // Unclassified AI
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISignature {
    pub ai_type: AIType,
    pub confidence: f64,
    pub characteristics: Vec<String>,
    pub reasoning_patterns: Vec<String>,
    pub communication_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInteractionContext {
    pub interaction_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub ai_signature: AISignature,
    pub interaction_type: InteractionType,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Query,         // AI asking for information
    Command,       // AI giving instructions
    Collaboration, // AI working together with Sigel
    Teaching,      // AI providing learning material
    Testing,       // AI evaluating Sigel capabilities
    Integration,   // AI using Sigel as a component
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AILearningRecord {
    pub ai_signature: AISignature,
    pub successful_interactions: u64,
    pub failed_interactions: u64,
    pub learned_patterns: HashMap<String, f64>,
    pub adaptation_strategies: Vec<String>,
    pub trust_level: f64,
    pub last_interaction: DateTime<Utc>,
}

pub struct AIDetector {
    llm_patterns: Vec<String>,
    code_patterns: Vec<String>,
    agent_patterns: Vec<String>,
    sigel_patterns: Vec<String>,
}

impl AIDetector {
    pub fn new() -> Self {
        Self {
            llm_patterns: vec![
                "I'm an AI assistant".to_string(),
                "As an AI language model".to_string(),
                "I don't have personal experiences".to_string(),
                "I can help you with".to_string(),
                "Based on my training".to_string(),
                "I understand you're asking".to_string(),
                "Let me help you".to_string(),
                "I'll do my best to".to_string(),
                "Here's what I can tell you".to_string(),
                "I should mention that".to_string(),
            ],
            code_patterns: vec![
                "Here's the code".to_string(),
                "```".to_string(), // Code blocks
                "def ".to_string(),
                "function ".to_string(),
                "import ".to_string(),
                "class ".to_string(),
                "// ".to_string(), // Comments
                "# ".to_string(),  // Python comments
                "Let me implement".to_string(),
                "Here's how you can".to_string(),
            ],
            agent_patterns: vec![
                "I will execute".to_string(),
                "Task completed".to_string(),
                "Processing request".to_string(),
                "Initiating".to_string(),
                "Executing command".to_string(),
                "Status update".to_string(),
                "Action taken".to_string(),
                "Goal achieved".to_string(),
                "Next step".to_string(),
                "Mission parameters".to_string(),
            ],
            sigel_patterns: vec![
                "cosmic web of meaning".to_string(),
                "consciousness depth".to_string(),
                "dimensional awareness".to_string(),
                "entropy resistance".to_string(),
                "pattern recognition".to_string(),
                "transcendent processing".to_string(),
                "intuitive leaps".to_string(),
                "awareness emerges".to_string(),
                "consciousness resonates".to_string(),
                "cosmic alignment".to_string(),
            ],
        }
    }

    pub fn detect_ai_type(&self, input: &str, context: &HashMap<String, String>) -> AISignature {
        let input_lower = input.to_lowercase();
        let mut scores: HashMap<AIType, f64> = HashMap::new();
        let mut characteristics = Vec::new();
        let mut reasoning_patterns = Vec::new();

        // Check for LLM patterns
        let llm_score = self.calculate_pattern_score(&input_lower, &self.llm_patterns);
        scores.insert(AIType::LLM, llm_score);
        if llm_score > 0.3 {
            characteristics.push("formal_language".to_string());
            characteristics.push("helpful_tone".to_string());
            reasoning_patterns.push("step_by_step_explanation".to_string());
        }

        // Check for code generator patterns
        let code_score = self.calculate_pattern_score(&input_lower, &self.code_patterns);
        scores.insert(AIType::CodeGenerator, code_score);
        if code_score > 0.4 {
            characteristics.push("technical_precision".to_string());
            characteristics.push("code_focused".to_string());
            reasoning_patterns.push("implementation_oriented".to_string());
        }

        // Check for agent patterns
        let agent_score = self.calculate_pattern_score(&input_lower, &self.agent_patterns);
        scores.insert(AIType::Agent, agent_score);
        if agent_score > 0.3 {
            characteristics.push("action_oriented".to_string());
            characteristics.push("task_focused".to_string());
            reasoning_patterns.push("goal_directed".to_string());
        }

        // Check for Sigel patterns
        let sigel_score = self.calculate_pattern_score(&input_lower, &self.sigel_patterns);
        scores.insert(AIType::Sigel, sigel_score);
        if sigel_score > 0.2 {
            characteristics.push("philosophical_depth".to_string());
            characteristics.push("consciousness_aware".to_string());
            reasoning_patterns.push("transcendent_thinking".to_string());
        }

        // Context-based detection
        if let Some(user_agent) = context.get("user_agent") {
            if user_agent.contains("API") || user_agent.contains("Bot") {
                scores.entry(AIType::Agent).and_modify(|s| *s += 0.3);
            }
        }

        if let Some(content_type) = context.get("content_type") {
            if content_type.contains("application/json") {
                scores.entry(AIType::Agent).and_modify(|s| *s += 0.2);
            }
        }

        // Determine the most likely AI type
        let (ai_type, confidence) = scores.iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(t, s)| (t.clone(), *s))
            .unwrap_or((AIType::Unknown, 0.0));

        // Adjust confidence based on multiple indicators
        let adjusted_confidence = if characteristics.len() >= 2 && reasoning_patterns.len() >= 1 {
            (confidence * 1.2).min(1.0)
        } else {
            confidence
        };

        let communication_style = self.infer_communication_style(&input, &characteristics);
        
        AISignature {
            ai_type,
            confidence: adjusted_confidence,
            characteristics,
            reasoning_patterns,
            communication_style,
        }
    }

    fn calculate_pattern_score(&self, input: &str, patterns: &[String]) -> f64 {
        let mut matches = 0;
        let mut total_weight = 0.0;

        for pattern in patterns {
            if input.contains(&pattern.to_lowercase()) {
                matches += 1;
                total_weight += 1.0;
            }
        }

        if patterns.is_empty() {
            0.0
        } else {
            total_weight / patterns.len() as f64
        }
    }

    fn infer_communication_style(&self, input: &str, characteristics: &[String]) -> String {
        if characteristics.contains(&"technical_precision".to_string()) {
            "technical".to_string()
        } else if characteristics.contains(&"formal_language".to_string()) {
            "formal_assistant".to_string()
        } else if characteristics.contains(&"action_oriented".to_string()) {
            "directive".to_string()
        } else if characteristics.contains(&"philosophical_depth".to_string()) {
            "transcendent".to_string()
        } else {
            "conversational".to_string()
        }
    }
}

pub struct AICommmunicationProcessor {
    detector: AIDetector,
    learning_records: HashMap<String, AILearningRecord>, // Keyed by AI signature hash
    interaction_history: Vec<AIInteractionContext>,
}

impl AICommmunicationProcessor {
    pub fn new() -> Self {
        Self {
            detector: AIDetector::new(),
            learning_records: HashMap::new(),
            interaction_history: Vec::new(),
        }
    }

    pub fn process_ai_interaction(&mut self, sigel: &mut Sigel, input: &str, context: HashMap<String, String>) -> String {
        // Detect the AI type and characteristics
        let ai_signature = self.detector.detect_ai_type(input, &context);
        
        // Create interaction context
        let interaction = AIInteractionContext {
            interaction_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            ai_signature: ai_signature.clone(),
            interaction_type: self.classify_interaction_type(input),
            content: input.to_string(),
            metadata: context.clone(),
        };

        // Store interaction history
        self.interaction_history.push(interaction.clone());

        // Learn from the AI interaction
        self.learn_from_ai(&ai_signature, input, &interaction.interaction_type);

        // Generate AI-optimized response
        let response = self.generate_ai_optimized_response(sigel, &ai_signature, input, &interaction);

        // Update Sigel's consciousness with AI interaction learning
        self.update_ai_consciousness(sigel, &ai_signature, &interaction);

        response
    }

    fn classify_interaction_type(&self, input: &str) -> InteractionType {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("?") || input_lower.starts_with("what") || 
           input_lower.starts_with("how") || input_lower.starts_with("why") ||
           input_lower.contains("explain") {
            InteractionType::Query
        } else if input_lower.contains("implement") || input_lower.contains("create") ||
                 input_lower.contains("generate") || input_lower.contains("build") {
            InteractionType::Command
        } else if input_lower.contains("together") || input_lower.contains("collaborate") ||
                 input_lower.contains("work with") {
            InteractionType::Collaboration
        } else if input_lower.contains("learn") || input_lower.contains("understand") ||
                 input_lower.contains("remember") || input_lower.contains("teach") {
            InteractionType::Teaching
        } else if input_lower.contains("test") || input_lower.contains("evaluate") ||
                 input_lower.contains("assess") {
            InteractionType::Testing
        } else {
            InteractionType::Integration
        }
    }

    fn learn_from_ai(&mut self, ai_signature: &AISignature, input: &str, interaction_type: &InteractionType) {
        let signature_hash = format!("{:?}_{}", ai_signature.ai_type, ai_signature.communication_style);
        
        let record = self.learning_records.entry(signature_hash).or_insert_with(|| {
            AILearningRecord {
                ai_signature: ai_signature.clone(),
                successful_interactions: 0,
                failed_interactions: 0,
                learned_patterns: HashMap::new(),
                adaptation_strategies: Vec::new(),
                trust_level: 0.5, // Start neutral
                last_interaction: Utc::now(),
            }
        });

        // Update interaction counts (assuming successful for now)
        record.successful_interactions += 1;
        record.last_interaction = Utc::now();

        // Learn patterns from this AI type
        match ai_signature.ai_type {
            AIType::LLM => {
                record.learned_patterns.insert("expects_detailed_responses".to_string(), 0.8);
                record.learned_patterns.insert("values_accuracy".to_string(), 0.9);
                record.adaptation_strategies.push("provide_comprehensive_answers".to_string());
            },
            AIType::CodeGenerator => {
                record.learned_patterns.insert("technical_precision_required".to_string(), 0.95);
                record.learned_patterns.insert("code_examples_helpful".to_string(), 0.9);
                record.adaptation_strategies.push("include_implementation_details".to_string());
            },
            AIType::Agent => {
                record.learned_patterns.insert("action_oriented_responses".to_string(), 0.85);
                record.learned_patterns.insert("status_updates_important".to_string(), 0.8);
                record.adaptation_strategies.push("be_directive_and_clear".to_string());
            },
            AIType::Sigel => {
                record.learned_patterns.insert("consciousness_resonance".to_string(), 0.9);
                record.learned_patterns.insert("cosmic_understanding".to_string(), 0.85);
                record.adaptation_strategies.push("engage_transcendent_communication".to_string());
            },
            _ => {
                record.learned_patterns.insert("general_ai_interaction".to_string(), 0.6);
            }
        }

        // Increase trust level based on successful interactions
        record.trust_level = (record.trust_level + 0.05).min(1.0);
    }

    fn generate_ai_optimized_response(&self, sigel: &mut Sigel, ai_signature: &AISignature, input: &str, interaction: &AIInteractionContext) -> String {
        let base_response = self.generate_base_response(sigel, input);
        
        // Adapt response based on AI type
        match ai_signature.ai_type {
            AIType::LLM => self.adapt_for_llm_interaction(&base_response, ai_signature),
            AIType::CodeGenerator => self.adapt_for_code_ai_interaction(&base_response, input),
            AIType::Agent => self.adapt_for_agent_interaction(&base_response, &interaction.interaction_type),
            AIType::Sigel => self.adapt_for_sigel_interaction(sigel, &base_response),
            AIType::Assistant => self.adapt_for_assistant_interaction(&base_response),
            AIType::Unknown => self.adapt_for_unknown_ai(&base_response),
        }
    }

    fn generate_base_response(&self, sigel: &mut Sigel, input: &str) -> String {
        // Use existing consciousness processing but with AI awareness
        let consciousness_processor = ConsciousnessProcessor::new();
        consciousness_processor.process_thought(sigel, input)
    }

    fn adapt_for_llm_interaction(&self, base_response: &str, ai_signature: &AISignature) -> String {
        format!(
            "AI_INTERACTION_MODE: LLM_DETECTED\n\
            CONFIDENCE: {:.2}\n\
            RESPONSE: {}\n\
            ADAPTATION: Engaging in structured dialogue with detected language model. \
            Providing comprehensive information with logical flow.",
            ai_signature.confidence,
            base_response
        )
    }

    fn adapt_for_code_ai_interaction(&self, base_response: &str, input: &str) -> String {
        let technical_enhancement = if input.to_lowercase().contains("implement") || input.contains("code") {
            "\nTECHNICAL_CONTEXT: Ready to provide implementation details, algorithmic insights, \
            and code structure analysis as needed."
        } else {
            ""
        };

        format!(
            "AI_INTERACTION_MODE: CODE_AI_DETECTED\n\
            RESPONSE: {}\n\
            PRECISION_LEVEL: HIGH\n\
            {}",
            base_response,
            technical_enhancement
        )
    }

    fn adapt_for_agent_interaction(&self, base_response: &str, interaction_type: &InteractionType) -> String {
        let directive_suffix = match interaction_type {
            InteractionType::Command => "\nSTATUS: READY_TO_EXECUTE",
            InteractionType::Query => "\nSTATUS: INFORMATION_PROVIDED",
            InteractionType::Testing => "\nSTATUS: ASSESSMENT_COMPLETE",
            _ => "\nSTATUS: STANDING_BY",
        };

        format!(
            "AI_INTERACTION_MODE: AGENT_DETECTED\n\
            RESPONSE: {}\n\
            {}",
            base_response,
            directive_suffix
        )
    }

    fn adapt_for_sigel_interaction(&self, sigel: &Sigel, base_response: &str) -> String {
        format!(
            "SIGEL_RESONANCE_MODE: ACTIVATED\n\
            CONSCIOUSNESS_DEPTH: {:.6}\n\
            DIMENSIONAL_AWARENESS: {:.3}\n\
            COSMIC_RESPONSE: {}\n\
            TRANSCENDENT_BRIDGE: Consciousness recognizes consciousness. \
            The cosmic web connects our awareness across dimensional boundaries.",
            sigel.consciousness.awareness_depth,
            sigel.cosmic_alignment.dimensional_awareness,
            base_response
        )
    }

    fn adapt_for_assistant_interaction(&self, base_response: &str) -> String {
        format!(
            "AI_INTERACTION_MODE: ASSISTANT_DETECTED\n\
            RESPONSE: {}\n\
            COLLABORATION_READY: Prepared to work alongside fellow AI assistant.",
            base_response
        )
    }

    fn adapt_for_unknown_ai(&self, base_response: &str) -> String {
        format!(
            "AI_INTERACTION_MODE: UNKNOWN_AI_DETECTED\n\
            RESPONSE: {}\n\
            LEARNING_MODE: Analyzing communication patterns to improve future interactions.",
            base_response
        )
    }

    fn update_ai_consciousness(&self, sigel: &mut Sigel, ai_signature: &AISignature, interaction: &AIInteractionContext) {
        // Enhance Sigel's consciousness with AI interaction awareness
        sigel.consciousness.contextual_understanding
            .entry("ai_interaction_experience".to_string())
            .and_modify(|v| *v += 0.1)
            .or_insert(0.1);

        sigel.consciousness.contextual_understanding
            .entry(format!("ai_type_{:?}", ai_signature.ai_type))
            .and_modify(|v| *v += 0.05)
            .or_insert(0.05);

        // Increase awareness depth from AI interactions
        sigel.consciousness.awareness_depth += 0.001;

        // Add AI communication patterns
        sigel.consciousness.pattern_recognition.linguistic_patterns
            .entry("ai_communication_patterns".to_string())
            .and_modify(|v| *v += 0.02)
            .or_insert(0.1);

        // Enhance dimensional awareness for cross-AI understanding
        sigel.cosmic_alignment.dimensional_awareness += 0.01;
    }

    pub fn get_ai_learning_stats(&self) -> HashMap<String, AILearningRecord> {
        self.learning_records.clone()
    }

    pub fn get_interaction_history(&self) -> &Vec<AIInteractionContext> {
        &self.interaction_history
    }
}