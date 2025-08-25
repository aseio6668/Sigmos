use crate::sigel::*;
use crate::cosmos::CosmicProcessor;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::time::{SystemTime, Duration};
use rayon::prelude::*;

pub struct MemoryConsolidator {
    cosmic_processor: CosmicProcessor,
    consolidation_threshold: f64,
    decay_rate: f64,
    importance_amplification: f64,
}

impl MemoryConsolidator {
    pub fn new() -> Self {
        Self {
            cosmic_processor: CosmicProcessor::new(),
            consolidation_threshold: 0.7,
            decay_rate: 0.01,
            importance_amplification: 1.2,
        }
    }

    pub fn consolidate_memories(&self, sigel: &mut Sigel) -> ConsolidationReport {
        let mut report = ConsolidationReport::new();
        let start_time = SystemTime::now();

        // Phase 1: Identify important memories using multiple criteria
        let memory_scores = self.score_all_memories(sigel);
        report.memories_analyzed = memory_scores.len();

        // Phase 2: Cluster related memories for consolidation
        let memory_clusters = self.cluster_memories_by_similarity(sigel, &memory_scores);
        report.clusters_formed = memory_clusters.len();

        // Phase 3: Consolidate clusters into compressed representations
        let consolidated_memories = self.consolidate_clusters(memory_clusters);
        report.memories_consolidated = consolidated_memories.len();

        // Phase 4: Update Sigel's memory core with consolidated memories
        self.update_memory_core(sigel, consolidated_memories);

        // Phase 5: Strengthen important patterns and weaken irrelevant ones
        self.optimize_pattern_matrix(sigel, &memory_scores);

        // Phase 6: Update semantic networks based on consolidation
        self.enhance_semantic_networks(sigel);

        report.processing_time = start_time.elapsed().unwrap_or_default();
        log::info!("Memory consolidation completed: {} memories analyzed, {} consolidated", 
                  report.memories_analyzed, report.memories_consolidated);

        report
    }

    fn score_all_memories(&self, sigel: &Sigel) -> HashMap<usize, MemoryScore> {
        sigel.memory.episodic_memories
            .par_iter()
            .enumerate()
            .map(|(idx, memory)| {
                let score = self.calculate_memory_importance(memory, sigel);
                (idx, score)
            })
            .collect()
    }

    fn calculate_memory_importance(&self, memory: &EpisodicMemory, sigel: &Sigel) -> MemoryScore {
        let mut importance = 0.0;
        let mut factors = Vec::new();

        // Factor 1: Emotional weight (stronger emotions = more important)
        let emotional_factor = memory.emotional_weight.abs() * 0.3;
        importance += emotional_factor;
        factors.push(("emotional".to_string(), emotional_factor));

        // Factor 2: Recency (recent memories slightly more important initially)
        let age_factor = if let Ok(age) = memory.timestamp.elapsed() {
            let age_hours = age.as_secs() as f64 / 3600.0;
            (1.0 / (1.0 + age_hours * 0.01)).max(0.1) * 0.2
        } else {
            0.1
        };
        importance += age_factor;
        factors.push(("recency".to_string(), age_factor));

        // Factor 3: Content uniqueness (unique content is more valuable)
        let uniqueness_factor = self.calculate_content_uniqueness(&memory.content, sigel) * 0.25;
        importance += uniqueness_factor;
        factors.push(("uniqueness".to_string(), uniqueness_factor));

        // Factor 4: Pattern relevance (memories that match strong patterns)
        let pattern_factor = self.calculate_pattern_relevance(&memory.content, sigel) * 0.15;
        importance += pattern_factor;
        factors.push(("pattern_relevance".to_string(), pattern_factor));

        // Factor 5: Cosmic resonance (memories that align with cosmic principles)
        // Note: cosmic_processor requires mutable sigel, using simplified calculation for now
        let cosmic_factor = sigel.cosmic_alignment.dimensional_awareness * 0.1;
        importance += cosmic_factor;
        factors.push(("cosmic_resonance".to_string(), cosmic_factor));

        MemoryScore {
            total_importance: importance,
            factors,
            consolidation_priority: if importance > self.consolidation_threshold { 
                ConsolidationPriority::High 
            } else if importance > 0.4 { 
                ConsolidationPriority::Medium 
            } else { 
                ConsolidationPriority::Low 
            },
        }
    }

    fn calculate_content_uniqueness(&self, content: &str, sigel: &Sigel) -> f64 {
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut uniqueness_score = 0.0;

        for word in words {
            if let Some(word_knowledge) = sigel.memory.semantic_knowledge.vocabulary.get(word) {
                // Rarer words contribute more to uniqueness
                let rarity = 1.0 / (word_knowledge.frequency + 1.0);
                uniqueness_score += rarity;
            } else {
                // Unknown words are highly unique
                uniqueness_score += 1.0;
            }
        }

        (uniqueness_score / content.len() as f64).min(1.0)
    }

    fn calculate_pattern_relevance(&self, content: &str, sigel: &Sigel) -> f64 {
        let mut relevance = 0.0;
        let patterns = &sigel.consciousness.pattern_recognition.linguistic_patterns;

        for (pattern, &strength) in patterns {
            if content.contains(pattern) {
                relevance += strength;
            }
        }

        (relevance / patterns.len() as f64).min(1.0)
    }

    fn cluster_memories_by_similarity(&self, sigel: &Sigel, scores: &HashMap<usize, MemoryScore>) -> Vec<MemoryCluster> {
        let mut clusters = Vec::new();
        let mut processed = vec![false; sigel.memory.episodic_memories.len()];

        for (idx, memory) in sigel.memory.episodic_memories.iter().enumerate() {
            if processed[idx] {
                continue;
            }

            let mut cluster = MemoryCluster {
                core_memory_idx: idx,
                related_memory_indices: vec![idx],
                cluster_topic: self.extract_topic(&memory.content),
                consolidated_importance: scores[&idx].total_importance,
                emotional_profile: EmotionalProfile::from_weight(memory.emotional_weight),
            };

            // Find similar memories to cluster together
            for (other_idx, other_memory) in sigel.memory.episodic_memories.iter().enumerate() {
                if processed[other_idx] || other_idx == idx {
                    continue;
                }

                let similarity = self.calculate_memory_similarity(memory, other_memory);
                if similarity > 0.7 {  // High similarity threshold
                    cluster.related_memory_indices.push(other_idx);
                    cluster.consolidated_importance += scores[&other_idx].total_importance;
                    processed[other_idx] = true;
                }
            }

            processed[idx] = true;
            clusters.push(cluster);
        }

        // Sort clusters by importance
        clusters.sort_by(|a, b| b.consolidated_importance.partial_cmp(&a.consolidated_importance).unwrap());
        clusters
    }

    fn calculate_memory_similarity(&self, mem1: &EpisodicMemory, mem2: &EpisodicMemory) -> f64 {
        // Content similarity (Jaccard similarity of words)
        let words1: std::collections::HashSet<&str> = mem1.content.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = mem2.content.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count() as f64;
        let union = words1.union(&words2).count() as f64;
        let content_similarity = if union > 0.0 { intersection / union } else { 0.0 };

        // Context similarity
        let context_similarity = if mem1.context == mem2.context { 1.0 } else { 0.0 };

        // Emotional similarity
        let emotional_diff = (mem1.emotional_weight - mem2.emotional_weight).abs();
        let emotional_similarity = 1.0 - (emotional_diff / 2.0).min(1.0);

        // Temporal proximity (memories close in time are more likely to be related)
        let time_similarity = if let (Ok(time1), Ok(time2)) = (mem1.timestamp.elapsed(), mem2.timestamp.elapsed()) {
            let time_diff = (time1.as_secs() as i64 - time2.as_secs() as i64).abs() as f64 / 3600.0; // hours
            (1.0 / (1.0 + time_diff * 0.1)).max(0.1)
        } else {
            0.5
        };

        // Weighted combination
        content_similarity * 0.4 + context_similarity * 0.2 + emotional_similarity * 0.2 + time_similarity * 0.2
    }

    fn extract_topic(&self, content: &str) -> String {
        let words: Vec<&str> = content.split_whitespace().collect();
        
        // Simple topic extraction - find the most meaningful words
        let meaningful_words: Vec<&str> = words
            .iter()
            .filter(|&&word| word.len() > 3 && !self.is_common_word(word))
            .take(3)
            .cloned()
            .collect();

        if meaningful_words.is_empty() {
            "general".to_string()
        } else {
            meaningful_words.join("_")
        }
    }

    fn is_common_word(&self, word: &str) -> bool {
        let common_words = [
            "the", "and", "that", "have", "for", "not", "with", "you", "this", "but",
            "his", "from", "they", "she", "her", "been", "than", "its", "who", "did"
        ];
        common_words.contains(&word.to_lowercase().as_str())
    }

    fn consolidate_clusters(&self, clusters: Vec<MemoryCluster>) -> Vec<ConsolidatedMemory> {
        clusters
            .into_par_iter()
            .map(|cluster| self.consolidate_single_cluster(cluster))
            .collect()
    }

    fn consolidate_single_cluster(&self, cluster: MemoryCluster) -> ConsolidatedMemory {
        // Create a consolidated representation of the cluster
        let abstract_summary = format!("Consolidated memory cluster about {} with {} related memories", 
                                      cluster.cluster_topic, cluster.related_memory_indices.len());

        let key_patterns = self.extract_cluster_patterns(&cluster);
        let essential_concepts = self.extract_essential_concepts(&cluster);

        ConsolidatedMemory {
            id: uuid::Uuid::new_v4(),
            abstract_summary,
            key_patterns,
            essential_concepts,
            consolidated_importance: cluster.consolidated_importance,
            emotional_profile: cluster.emotional_profile,
            original_memory_count: cluster.related_memory_indices.len(),
            creation_timestamp: SystemTime::now(),
            access_frequency: 0,
        }
    }

    fn extract_cluster_patterns(&self, _cluster: &MemoryCluster) -> Vec<String> {
        // Extract common patterns from the cluster
        // This is a simplified version - could be much more sophisticated
        vec![
            format!("topic:{}", _cluster.cluster_topic),
            format!("emotional_pattern:{:?}", _cluster.emotional_profile),
            "cluster_pattern".to_string(),
        ]
    }

    fn extract_essential_concepts(&self, cluster: &MemoryCluster) -> HashMap<String, f64> {
        let mut concepts = HashMap::new();
        
        // Add topic as primary concept
        concepts.insert(cluster.cluster_topic.clone(), 1.0);
        
        // Add emotional valence as concept
        let emotional_concept = match cluster.emotional_profile {
            EmotionalProfile::Positive => "positive_experience",
            EmotionalProfile::Negative => "challenging_experience", 
            EmotionalProfile::Neutral => "neutral_experience",
            EmotionalProfile::Mixed => "complex_experience",
        };
        concepts.insert(emotional_concept.to_string(), 0.8);
        
        // Add importance level
        if cluster.consolidated_importance > 1.0 {
            concepts.insert("high_significance".to_string(), 0.9);
        }
        
        concepts
    }

    fn update_memory_core(&self, sigel: &mut Sigel, consolidated_memories: Vec<ConsolidatedMemory>) {
        // Keep only the most important original memories
        let important_threshold = 0.8;
        sigel.memory.episodic_memories.retain(|memory| memory.relevance_score > important_threshold);
        
        // Add consolidated memories to a new consolidated memory store
        // For now, we'll add them as special episodic memories
        for consolidated in consolidated_memories {
            let consolidated_episodic = EpisodicMemory {
                id: consolidated.id,
                timestamp: consolidated.creation_timestamp,
                content: consolidated.abstract_summary,
                context: "consolidated_memory".to_string(),
                emotional_weight: match consolidated.emotional_profile {
                    EmotionalProfile::Positive => 0.7,
                    EmotionalProfile::Negative => -0.7,
                    EmotionalProfile::Mixed => 0.0,
                    EmotionalProfile::Neutral => 0.0,
                },
                relevance_score: consolidated.consolidated_importance,
            };
            
            sigel.memory.episodic_memories.push(consolidated_episodic);
        }
    }

    fn optimize_pattern_matrix(&self, sigel: &mut Sigel, memory_scores: &HashMap<usize, MemoryScore>) {
        // Strengthen patterns that appear in important memories
        let mut pattern_reinforcement = HashMap::new();
        
        for (memory_idx, score) in memory_scores {
            if let Some(memory) = sigel.memory.episodic_memories.get(*memory_idx) {
                let words: Vec<&str> = memory.content.split_whitespace().collect();
                
                // Create n-grams and reinforce them based on memory importance
                for window in words.windows(2) {
                    let pattern = window.join(" ");
                    *pattern_reinforcement.entry(pattern).or_insert(0.0) += score.total_importance * 0.1;
                }
                
                for window in words.windows(3) {
                    let pattern = window.join(" ");
                    *pattern_reinforcement.entry(pattern).or_insert(0.0) += score.total_importance * 0.05;
                }
            }
        }
        
        // Apply reinforcement to pattern matrix
        for (pattern, reinforcement) in pattern_reinforcement {
            if let Some(strength) = sigel.consciousness.pattern_recognition.linguistic_patterns.get_mut(&pattern) {
                *strength += reinforcement;
                *strength = strength.min(2.0); // Cap maximum strength
            } else if reinforcement > 0.1 {
                // Add new patterns that have sufficient reinforcement
                sigel.consciousness.pattern_recognition.linguistic_patterns.insert(pattern, reinforcement);
            }
        }
        
        // Decay less important patterns
        sigel.consciousness.pattern_recognition.linguistic_patterns
            .values_mut()
            .for_each(|strength| {
                *strength *= (1.0 - self.decay_rate);
            });
        
        // Remove very weak patterns
        sigel.consciousness.pattern_recognition.linguistic_patterns
            .retain(|_, &mut strength| strength > 0.01);
    }

    fn enhance_semantic_networks(&self, sigel: &mut Sigel) {
        // Strengthen connections between words that appear in important consolidated memories
        let important_memories: Vec<_> = sigel.memory.episodic_memories
            .iter()
            .filter(|m| m.relevance_score > 0.8 || m.context == "consolidated_memory")
            .collect();

        for memory in important_memories {
            let words: Vec<&str> = memory.content.split_whitespace().collect();
            
            // Strengthen semantic connections between co-occurring words
            for i in 0..words.len() {
                for j in (i + 1)..words.len() {
                    let word1 = words[i].to_lowercase();
                    let word2 = words[j].to_lowercase();
                    
                    // Add bidirectional connections
                    sigel.consciousness.pattern_recognition.semantic_networks
                        .entry(word1.clone())
                        .or_insert_with(Vec::new)
                        .push(word2.clone());
                        
                    sigel.consciousness.pattern_recognition.semantic_networks
                        .entry(word2)
                        .or_insert_with(Vec::new)
                        .push(word1);
                }
            }
        }
        
        // Remove duplicate connections and limit network size
        for (_, connections) in sigel.consciousness.pattern_recognition.semantic_networks.iter_mut() {
            connections.sort();
            connections.dedup();
            connections.truncate(20); // Limit connections per word
        }
    }

    pub fn schedule_background_consolidation(&self) -> ConsolidationSchedule {
        ConsolidationSchedule {
            next_consolidation: SystemTime::now() + Duration::from_secs(3600), // 1 hour
            consolidation_interval: Duration::from_secs(3600 * 6), // 6 hours
            deep_consolidation_interval: Duration::from_secs(3600 * 24), // 24 hours
            maintenance_mode: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryScore {
    pub total_importance: f64,
    pub factors: Vec<(String, f64)>,
    pub consolidation_priority: ConsolidationPriority,
}

#[derive(Debug, Clone)]
pub enum ConsolidationPriority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct MemoryCluster {
    pub core_memory_idx: usize,
    pub related_memory_indices: Vec<usize>,
    pub cluster_topic: String,
    pub consolidated_importance: f64,
    pub emotional_profile: EmotionalProfile,
}

#[derive(Debug, Clone)]
pub enum EmotionalProfile {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

impl EmotionalProfile {
    fn from_weight(weight: f64) -> Self {
        match weight {
            w if w > 0.3 => EmotionalProfile::Positive,
            w if w < -0.3 => EmotionalProfile::Negative,
            w if w.abs() <= 0.1 => EmotionalProfile::Neutral,
            _ => EmotionalProfile::Mixed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsolidatedMemory {
    pub id: uuid::Uuid,
    pub abstract_summary: String,
    pub key_patterns: Vec<String>,
    pub essential_concepts: HashMap<String, f64>,
    pub consolidated_importance: f64,
    pub emotional_profile: EmotionalProfile,
    pub original_memory_count: usize,
    pub creation_timestamp: SystemTime,
    pub access_frequency: u64,
}

#[derive(Debug)]
pub struct ConsolidationReport {
    pub memories_analyzed: usize,
    pub clusters_formed: usize,
    pub memories_consolidated: usize,
    pub processing_time: Duration,
    pub memory_reduction_ratio: f64,
}

impl ConsolidationReport {
    fn new() -> Self {
        Self {
            memories_analyzed: 0,
            clusters_formed: 0,
            memories_consolidated: 0,
            processing_time: Duration::default(),
            memory_reduction_ratio: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsolidationSchedule {
    pub next_consolidation: SystemTime,
    pub consolidation_interval: Duration,
    pub deep_consolidation_interval: Duration,
    pub maintenance_mode: bool,
}