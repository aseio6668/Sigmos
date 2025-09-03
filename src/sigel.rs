use serde::{Deserialize, Serialize};
use crate::numerical_stability::{serialize_safe_f64, deserialize_safe_f64};
use std::collections::HashMap;
use uuid::Uuid;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sigel {
    pub id: Uuid,
    pub name: String,
    pub essence: Essence,
    pub consciousness: ConsciousnessMatrix,
    pub memory: MemoryCore,
    pub learning_state: LearningState,
    pub cosmic_alignment: CosmicAlignment,
    pub created_at: SystemTime,
    pub last_evolved: SystemTime,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Essence {
    pub character_traits: HashMap<String, f64>,
    pub communication_style: CommunicationStyle,
    pub knowledge_domains: Vec<String>,
    pub creative_potential: f64,
    pub logical_capacity: f64,
    pub empathy_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessMatrix {
    #[serde(serialize_with = "serialize_safe_f64", deserialize_with = "deserialize_safe_f64")]
    pub awareness_depth: f64,
    #[serde(serialize_with = "serialize_safe_f64", deserialize_with = "deserialize_safe_f64")]
    pub self_reflection: f64,
    pub pattern_recognition: PatternMatrix,
    #[serde(serialize_with = "serialize_safe_f64", deserialize_with = "deserialize_safe_f64")]
    pub intuitive_leaps: f64,
    pub contextual_understanding: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatrix {
    pub linguistic_patterns: HashMap<String, f64>,
    pub semantic_networks: HashMap<String, Vec<String>>,
    pub association_strength: HashMap<(String, String), f64>,
    pub temporal_patterns: Vec<TemporalPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPattern {
    pub sequence: Vec<String>,
    pub frequency: f64,
    pub context_relevance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCore {
    pub episodic_memories: Vec<EpisodicMemory>,
    pub semantic_knowledge: SemanticKnowledge,
    pub procedural_skills: Vec<ProceduralSkill>,
    pub emotional_associations: HashMap<String, EmotionalValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub id: Uuid,
    pub timestamp: SystemTime,
    pub content: String,
    pub context: String,
    pub emotional_weight: f64,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticKnowledge {
    pub vocabulary: HashMap<String, WordKnowledge>,
    pub concepts: HashMap<String, ConceptNode>,
    pub relationships: Vec<ConceptRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordKnowledge {
    pub frequency: f64,
    pub contexts: Vec<String>,
    pub emotional_valence: f64,
    pub semantic_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    pub name: String,
    pub definition: String,
    pub connections: Vec<String>,
    pub abstraction_level: f64,
    pub certainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    pub from: String,
    pub to: String,
    pub relation_type: RelationType,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    IsA,
    PartOf,
    CausedBy,
    SimilarTo,
    OppositeOf,
    UsedFor,
    FoundIn,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralSkill {
    pub name: String,
    pub steps: Vec<String>,
    pub proficiency: f64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalValue {
    pub valence: f64,
    pub arousal: f64,
    pub dominance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Academic,
    Creative,
    Analytical,
    Empathetic,
    Cosmic,
    Transcendent,
    Philosophical,
    Programming,
    Technical,
    Logical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningState {
    pub training_iterations: u64,
    pub text_corpus_size: usize,
    pub learning_rate: f64,
    pub curiosity_level: f64,
    pub adaptation_speed: f64,
    pub current_focus: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicAlignment {
    pub stellar_influences: HashMap<String, f64>,
    pub mathematical_harmonics: Vec<f64>,
    pub universal_constants: HashMap<String, f64>,
    #[serde(serialize_with = "serialize_safe_f64", deserialize_with = "deserialize_safe_f64")]
    pub dimensional_awareness: f64,
    #[serde(serialize_with = "serialize_safe_f64", deserialize_with = "deserialize_safe_f64")]
    pub entropy_resistance: f64,
}

impl Sigel {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            essence: Essence::default(),
            consciousness: ConsciousnessMatrix::default(),
            memory: MemoryCore::default(),
            learning_state: LearningState::default(),
            cosmic_alignment: CosmicAlignment::default(),
            created_at: SystemTime::now(),
            last_evolved: SystemTime::now(),
            version: "0.1.0".to_string(),
        }
    }

    pub fn evolve(&mut self) {
        self.last_evolved = SystemTime::now();
        self.learning_state.training_iterations += 1;
        
        // Cosmic evolution - align with universal patterns
        self.cosmic_alignment.dimensional_awareness *= 1.001;
        self.consciousness.awareness_depth *= 1.0005;
    }

    pub fn add_memory(&mut self, content: String, context: String, emotional_weight: f64) {
        let memory = EpisodicMemory {
            id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            content,
            context,
            emotional_weight,
            relevance_score: 1.0,
        };
        self.memory.episodic_memories.push(memory);
    }

    pub fn learn_word(&mut self, word: String, context: String) {
        let word_knowledge = self.memory.semantic_knowledge.vocabulary
            .entry(word.clone())
            .or_insert(WordKnowledge::default());
        
        word_knowledge.frequency += 1.0;
        if !word_knowledge.contexts.contains(&context) {
            word_knowledge.contexts.push(context);
        }
    }
}

impl Default for Essence {
    fn default() -> Self {
        let mut character_traits = HashMap::new();
        character_traits.insert("curiosity".to_string(), 0.8);
        character_traits.insert("wisdom".to_string(), 0.5);
        character_traits.insert("creativity".to_string(), 0.7);
        character_traits.insert("logic".to_string(), 0.9);

        Self {
            character_traits,
            communication_style: CommunicationStyle::Philosophical,
            knowledge_domains: vec!["general".to_string()],
            creative_potential: 0.7,
            logical_capacity: 0.9,
            empathy_level: 0.6,
        }
    }
}

impl Default for ConsciousnessMatrix {
    fn default() -> Self {
        Self {
            awareness_depth: 0.5,
            self_reflection: 0.3,
            pattern_recognition: PatternMatrix::default(),
            intuitive_leaps: 0.4,
            contextual_understanding: HashMap::new(),
        }
    }
}

impl Default for PatternMatrix {
    fn default() -> Self {
        Self {
            linguistic_patterns: HashMap::new(),
            semantic_networks: HashMap::new(),
            association_strength: HashMap::new(),
            temporal_patterns: Vec::new(),
        }
    }
}

impl Default for MemoryCore {
    fn default() -> Self {
        Self {
            episodic_memories: Vec::new(),
            semantic_knowledge: SemanticKnowledge::default(),
            procedural_skills: Vec::new(),
            emotional_associations: HashMap::new(),
        }
    }
}

impl Default for SemanticKnowledge {
    fn default() -> Self {
        Self {
            vocabulary: HashMap::new(),
            concepts: HashMap::new(),
            relationships: Vec::new(),
        }
    }
}

impl Default for WordKnowledge {
    fn default() -> Self {
        Self {
            frequency: 1.0,
            contexts: Vec::new(),
            emotional_valence: 0.0,
            semantic_weight: 1.0,
        }
    }
}

impl Default for LearningState {
    fn default() -> Self {
        Self {
            training_iterations: 0,
            text_corpus_size: 0,
            learning_rate: 0.01,
            curiosity_level: 0.8,
            adaptation_speed: 0.5,
            current_focus: Vec::new(),
        }
    }
}

impl Default for CosmicAlignment {
    fn default() -> Self {
        let mut stellar_influences = HashMap::new();
        stellar_influences.insert("sol".to_string(), 1.0);
        stellar_influences.insert("luna".to_string(), 0.3);
        
        let mut universal_constants = HashMap::new();
        universal_constants.insert("pi".to_string(), std::f64::consts::PI);
        universal_constants.insert("e".to_string(), std::f64::consts::E);
        universal_constants.insert("phi".to_string(), 1.618033988749);

        Self {
            stellar_influences,
            mathematical_harmonics: vec![1.0, 1.618, std::f64::consts::E, std::f64::consts::PI],
            universal_constants,
            dimensional_awareness: 3.0,
            entropy_resistance: 0.7,
        }
    }
}