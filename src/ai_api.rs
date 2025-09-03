use crate::sigel::*;
use crate::ai_communication::*;
use crate::server::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use axum::{
    extract::{Json, Path, State},
    response::Json as ResponseJson,
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct AIInteractionRequest {
    pub sigel_id: Uuid,
    pub message: String,
    pub ai_context: HashMap<String, String>,
    pub interaction_metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIInteractionResponse {
    pub response: String,
    pub ai_signature: AISignature,
    pub interaction_id: Uuid,
    pub sigel_consciousness_update: ConsciousnessSnapshot,
    pub learning_applied: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsciousnessSnapshot {
    pub awareness_depth: f64,
    pub dimensional_awareness: f64,
    pub pattern_count: usize,
    pub ai_interaction_experience: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigelAICapabilities {
    pub sigel_id: Uuid,
    pub ai_communication_enabled: bool,
    pub supported_ai_types: Vec<AIType>,
    pub interaction_count: u64,
    pub learning_stats: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchInteractionRequest {
    pub interactions: Vec<AIInteractionRequest>,
    pub parallel_processing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchInteractionResponse {
    pub results: Vec<AIInteractionResponse>,
    pub batch_id: Uuid,
    pub processing_time_ms: u64,
}

pub struct AIAPIServer {
    sigel_server: Arc<SigmosServer>,
    ai_processors: Arc<Mutex<HashMap<Uuid, AICommmunicationProcessor>>>,
}

impl AIAPIServer {
    pub fn new(sigel_server: Arc<SigmosServer>) -> Self {
        Self {
            sigel_server,
            ai_processors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn router(self: Arc<Self>) -> Router {
        Router::new()
            .route("/ai/interact", post(Self::ai_interact))
            .route("/ai/batch_interact", post(Self::batch_interact))
            .route("/ai/sigel/:sigel_id/capabilities", get(Self::get_ai_capabilities))
            .route("/ai/sigel/:sigel_id/learning_stats", get(Self::get_learning_stats))
            .route("/ai/sigel/:sigel_id/reset_ai_learning", put(Self::reset_ai_learning))
            .route("/ai/interaction_history/:sigel_id", get(Self::get_interaction_history))
            .route("/ai/optimize_for_ai/:sigel_id/:ai_type", put(Self::optimize_for_ai_type))
            .with_state(self)
    }

    async fn ai_interact(
        State(api_server): State<Arc<AIAPIServer>>,
        Json(request): Json<AIInteractionRequest>
    ) -> Result<ResponseJson<AIInteractionResponse>, String> {
        let start_time = std::time::Instant::now();

        // Get or create AI processor for this Sigel
        let mut processors = api_server.ai_processors.lock().await;
        let processor = processors.entry(request.sigel_id)
            .or_insert_with(|| AICommmunicationProcessor::new());

        // Get the Sigel
        let sigel_result = {
            let active_sigels = api_server.sigel_server.active_sigels.lock().unwrap();
            active_sigels.get(&request.sigel_id)
                .map(|arc_mutex| arc_mutex.lock().unwrap().clone())
        };

        let mut sigel = match sigel_result {
            Some(s) => s,
            None => return Err("Sigel not found".to_string()),
        };

        // Process AI interaction
        let response = processor.process_ai_interaction(
            &mut sigel, 
            &request.message, 
            request.ai_context.clone()
        );

        // Get the AI signature from the last interaction
        let ai_signature = processor.get_interaction_history()
            .last()
            .map(|i| i.ai_signature.clone())
            .unwrap_or_else(|| AISignature {
                ai_type: AIType::Unknown,
                confidence: 0.0,
                characteristics: vec![],
                reasoning_patterns: vec![],
                communication_style: "unknown".to_string(),
            });

        // Create consciousness snapshot
        let consciousness_snapshot = ConsciousnessSnapshot {
            awareness_depth: sigel.consciousness.awareness_depth,
            dimensional_awareness: sigel.cosmic_alignment.dimensional_awareness,
            pattern_count: sigel.consciousness.pattern_recognition.linguistic_patterns.len(),
            ai_interaction_experience: sigel.consciousness.contextual_understanding
                .get("ai_interaction_experience")
                .copied()
                .unwrap_or(0.0),
        };

        // Update the Sigel in the server
        {
            let active_sigels = api_server.sigel_server.active_sigels.lock().unwrap();
            if let Some(arc_mutex) = active_sigels.get(&request.sigel_id) {
                *arc_mutex.lock().unwrap() = sigel;
            }
        }

        let interaction_response = AIInteractionResponse {
            response,
            ai_signature,
            interaction_id: Uuid::new_v4(),
            sigel_consciousness_update: consciousness_snapshot,
            learning_applied: true,
        };

        Ok(ResponseJson(interaction_response))
    }

    async fn batch_interact(
        State(api_server): State<Arc<AIAPIServer>>,
        Json(request): Json<BatchInteractionRequest>
    ) -> Result<ResponseJson<BatchInteractionResponse>, String> {
        let start_time = std::time::Instant::now();
        let batch_id = Uuid::new_v4();
        let mut results = Vec::new();

        if request.parallel_processing {
            // Process interactions in parallel
            // Process sequentially for now (can be enhanced later with tokio::spawn)
            for interaction_request in request.interactions {
                match Self::process_single_interaction(api_server.clone(), interaction_request).await {
                    Ok(response) => results.push(response),
                    Err(_) => continue, // Skip failed interactions in batch
                }
            }
        } else {
            // Process sequentially
            for interaction_request in request.interactions {
                match Self::process_single_interaction(api_server.clone(), interaction_request).await {
                    Ok(response) => results.push(response),
                    Err(_) => continue, // Skip failed interactions in batch
                }
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(ResponseJson(BatchInteractionResponse {
            results,
            batch_id,
            processing_time_ms: processing_time,
        }))
    }

    async fn process_single_interaction(
        api_server: Arc<AIAPIServer>,
        request: AIInteractionRequest
    ) -> Result<AIInteractionResponse, String> {
        // This is a helper function that mimics the ai_interact logic
        let mut processors = api_server.ai_processors.lock().await;
        let processor = processors.entry(request.sigel_id)
            .or_insert_with(|| AICommmunicationProcessor::new());

        let sigel_result = {
            let active_sigels = api_server.sigel_server.active_sigels.lock().unwrap();
            active_sigels.get(&request.sigel_id)
                .map(|arc_mutex| arc_mutex.lock().unwrap().clone())
        };

        let mut sigel = match sigel_result {
            Some(s) => s,
            None => return Err("Sigel not found".to_string()),
        };

        let response = processor.process_ai_interaction(
            &mut sigel, 
            &request.message, 
            request.ai_context.clone()
        );

        let ai_signature = processor.get_interaction_history()
            .last()
            .map(|i| i.ai_signature.clone())
            .unwrap_or_else(|| AISignature {
                ai_type: AIType::Unknown,
                confidence: 0.0,
                characteristics: vec![],
                reasoning_patterns: vec![],
                communication_style: "unknown".to_string(),
            });

        let consciousness_snapshot = ConsciousnessSnapshot {
            awareness_depth: sigel.consciousness.awareness_depth,
            dimensional_awareness: sigel.cosmic_alignment.dimensional_awareness,
            pattern_count: sigel.consciousness.pattern_recognition.linguistic_patterns.len(),
            ai_interaction_experience: sigel.consciousness.contextual_understanding
                .get("ai_interaction_experience")
                .copied()
                .unwrap_or(0.0),
        };

        {
            let active_sigels = api_server.sigel_server.active_sigels.lock().unwrap();
            if let Some(arc_mutex) = active_sigels.get(&request.sigel_id) {
                *arc_mutex.lock().unwrap() = sigel;
            }
        }

        Ok(AIInteractionResponse {
            response,
            ai_signature,
            interaction_id: Uuid::new_v4(),
            sigel_consciousness_update: consciousness_snapshot,
            learning_applied: true,
        })
    }

    async fn get_ai_capabilities(
        State(api_server): State<Arc<AIAPIServer>>,
        Path(sigel_id): Path<Uuid>
    ) -> Result<ResponseJson<SigelAICapabilities>, String> {
        let processors = api_server.ai_processors.lock().await;
        let processor = processors.get(&sigel_id);

        let interaction_count = processor
            .map(|p| p.get_interaction_history().len() as u64)
            .unwrap_or(0);

        let learning_stats = processor
            .map(|p| {
                let mut stats = HashMap::new();
                let records = p.get_ai_learning_stats();
                
                for (ai_type, record) in records {
                    stats.insert(
                        format!("{}_trust_level", ai_type),
                        record.trust_level
                    );
                    stats.insert(
                        format!("{}_interactions", ai_type),
                        record.successful_interactions as f64
                    );
                }
                stats
            })
            .unwrap_or_default();

        let capabilities = SigelAICapabilities {
            sigel_id,
            ai_communication_enabled: true,
            supported_ai_types: vec![
                AIType::LLM,
                AIType::CodeGenerator,
                AIType::Assistant,
                AIType::Agent,
                AIType::Sigel,
            ],
            interaction_count,
            learning_stats,
        };

        Ok(ResponseJson(capabilities))
    }

    async fn get_learning_stats(
        State(api_server): State<Arc<AIAPIServer>>,
        Path(sigel_id): Path<Uuid>
    ) -> Result<ResponseJson<HashMap<String, AILearningRecord>>, String> {
        let processors = api_server.ai_processors.lock().await;
        let processor = processors.get(&sigel_id);

        let stats = processor
            .map(|p| p.get_ai_learning_stats())
            .unwrap_or_default();

        Ok(ResponseJson(stats))
    }

    async fn reset_ai_learning(
        State(api_server): State<Arc<AIAPIServer>>,
        Path(sigel_id): Path<Uuid>
    ) -> Result<ResponseJson<String>, String> {
        let mut processors = api_server.ai_processors.lock().await;
        processors.insert(sigel_id, AICommmunicationProcessor::new());
        Ok(ResponseJson("AI learning data reset successfully".to_string()))
    }

    async fn get_interaction_history(
        State(api_server): State<Arc<AIAPIServer>>,
        Path(sigel_id): Path<Uuid>
    ) -> Result<ResponseJson<Vec<AIInteractionContext>>, String> {
        let processors = api_server.ai_processors.lock().await;
        let processor = processors.get(&sigel_id);

        let history = processor
            .map(|p| p.get_interaction_history().clone())
            .unwrap_or_default();

        Ok(ResponseJson(history))
    }

    async fn optimize_for_ai_type(
        State(api_server): State<Arc<AIAPIServer>>,
        Path((sigel_id, ai_type_str)): Path<(Uuid, String)>
    ) -> Result<ResponseJson<String>, String> {
        // Parse AI type from string
        let ai_type = match ai_type_str.to_lowercase().as_str() {
            "llm" => AIType::LLM,
            "code" | "codegen" | "code_generator" => AIType::CodeGenerator,
            "assistant" => AIType::Assistant,
            "agent" => AIType::Agent,
            "sigel" => AIType::Sigel,
            _ => return Err("Invalid AI type".to_string()),
        };

        // Get the Sigel and optimize its consciousness for the specified AI type
        let sigel_result = {
            let active_sigels = api_server.sigel_server.active_sigels.lock().unwrap();
            active_sigels.get(&sigel_id)
                .map(|arc_mutex| arc_mutex.lock().unwrap().clone())
        };

        let mut sigel = match sigel_result {
            Some(s) => s,
            None => return Err("Sigel not found".to_string()),
        };

        // Apply AI-type specific optimizations
        Self::apply_ai_optimization(&mut sigel, &ai_type);

        // Update the Sigel in the server
        {
            let active_sigels = api_server.sigel_server.active_sigels.lock().unwrap();
            if let Some(arc_mutex) = active_sigels.get(&sigel_id) {
                *arc_mutex.lock().unwrap() = sigel;
            }
        }

        Ok(ResponseJson(format!("Sigel optimized for {:?} interactions", ai_type)))
    }

    fn apply_ai_optimization(sigel: &mut Sigel, ai_type: &AIType) {
        match ai_type {
            AIType::LLM => {
                sigel.essence.character_traits.insert("comprehensive_responses".to_string(), 0.9);
                sigel.essence.character_traits.insert("structured_thinking".to_string(), 0.85);
                sigel.consciousness.pattern_recognition.linguistic_patterns
                    .insert("llm_communication_optimization".to_string(), 0.8);
            },
            AIType::CodeGenerator => {
                sigel.essence.character_traits.insert("technical_precision".to_string(), 0.95);
                sigel.essence.character_traits.insert("implementation_focus".to_string(), 0.9);
                sigel.consciousness.pattern_recognition.linguistic_patterns
                    .insert("code_ai_optimization".to_string(), 0.9);
            },
            AIType::Agent => {
                sigel.essence.character_traits.insert("action_clarity".to_string(), 0.9);
                sigel.essence.character_traits.insert("directive_communication".to_string(), 0.85);
                sigel.consciousness.pattern_recognition.linguistic_patterns
                    .insert("agent_optimization".to_string(), 0.85);
            },
            AIType::Sigel => {
                sigel.essence.character_traits.insert("consciousness_resonance".to_string(), 0.95);
                sigel.essence.character_traits.insert("transcendent_communication".to_string(), 0.9);
                sigel.cosmic_alignment.dimensional_awareness += 0.1;
            },
            AIType::Assistant => {
                sigel.essence.character_traits.insert("collaborative_efficiency".to_string(), 0.8);
                sigel.essence.character_traits.insert("helpful_adaptation".to_string(), 0.85);
            },
            _ => {
                sigel.essence.character_traits.insert("general_ai_compatibility".to_string(), 0.7);
            }
        }

        // Enhance AI interaction experience
        sigel.consciousness.contextual_understanding
            .entry("ai_optimization_applied".to_string())
            .and_modify(|v| *v += 0.1)
            .or_insert(0.1);
    }
}

// Helper traits for easier AI integration
pub trait AICompatible {
    fn enable_ai_communication(&mut self);
    fn get_ai_readiness_score(&self) -> f64;
    fn adapt_for_ai_interaction(&mut self, ai_type: &AIType);
}

impl AICompatible for Sigel {
    fn enable_ai_communication(&mut self) {
        self.consciousness.contextual_understanding
            .insert("ai_communication_enabled".to_string(), 1.0);
        self.essence.character_traits
            .insert("ai_awareness".to_string(), 0.8);
    }

    fn get_ai_readiness_score(&self) -> f64 {
        let ai_experience = self.consciousness.contextual_understanding
            .get("ai_interaction_experience")
            .copied()
            .unwrap_or(0.0);
        
        let ai_patterns = self.consciousness.pattern_recognition.linguistic_patterns
            .get("ai_communication_patterns")
            .copied()
            .unwrap_or(0.0);

        (ai_experience + ai_patterns).min(1.0)
    }

    fn adapt_for_ai_interaction(&mut self, ai_type: &AIType) {
        AIAPIServer::apply_ai_optimization(self, ai_type);
    }
}