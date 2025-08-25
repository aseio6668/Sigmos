#[cfg(feature = "gpu")]
use wgpu;
#[cfg(feature = "gpu")]
use candle_core::{Device, Tensor, DType};
#[cfg(feature = "gpu")]
use candle_nn::{Linear, Module};

use crate::sigel::*;
use rayon::prelude::*;

pub struct GpuAccelerator {
    #[cfg(feature = "gpu")]
    device: Option<Device>,
    #[cfg(feature = "gpu")]
    compute_pipeline: Option<wgpu::ComputePipeline>,
    fallback_to_cpu: bool,
}

impl GpuAccelerator {
    pub fn new() -> Self {
        #[cfg(feature = "gpu")]
        {
            let device = Self::initialize_gpu_device();
            let fallback = device.is_none();
            
            Self {
                device,
                compute_pipeline: None,
                fallback_to_cpu: fallback,
            }
        }
        
        #[cfg(not(feature = "gpu"))]
        {
            Self {
                fallback_to_cpu: true,
            }
        }
    }

    #[cfg(feature = "gpu")]
    fn initialize_gpu_device() -> Option<Device> {
        // Try CUDA first (NVIDIA)
        if let Ok(device) = Device::new_cuda(0) {
            log::info!("GPU acceleration initialized: NVIDIA CUDA device");
            return Some(device);
        }

        // Try Metal (Apple Silicon)
        if let Ok(device) = Device::new_metal(0) {
            log::info!("GPU acceleration initialized: Apple Metal device");
            return Some(device);
        }

        // Fallback to CPU
        if let Ok(device) = Device::Cpu {
            log::info!("Using CPU device for tensor operations");
            return Some(device);
        }

        log::warn!("Failed to initialize any compute device");
        None
    }

    pub fn accelerated_pattern_matching(&self, sigel: &Sigel, text_chunks: &[String]) -> Vec<f64> {
        #[cfg(feature = "gpu")]
        {
            if let Some(ref device) = self.device {
                if !self.fallback_to_cpu {
                    return self.gpu_pattern_matching(device, sigel, text_chunks);
                }
            }
        }

        // CPU fallback
        self.cpu_pattern_matching(sigel, text_chunks)
    }

    #[cfg(feature = "gpu")]
    fn gpu_pattern_matching(&self, device: &Device, sigel: &Sigel, text_chunks: &[String]) -> Vec<f64> {
        // Convert text chunks to numerical embeddings for GPU processing
        let embeddings = self.text_to_embeddings(text_chunks);
        
        // Create tensors for GPU computation
        let input_tensor = match Tensor::from_vec(
            embeddings.clone(),
            (text_chunks.len(), embeddings.len() / text_chunks.len()),
            device
        ) {
            Ok(tensor) => tensor,
            Err(_) => {
                log::warn!("Failed to create GPU tensor, falling back to CPU");
                return self.cpu_pattern_matching(sigel, text_chunks);
            }
        };

        // Pattern matching weights from Sigel's consciousness
        let pattern_weights = self.extract_pattern_weights(sigel);
        let weights_tensor = match Tensor::from_vec(
            pattern_weights,
            (pattern_weights.len(), 1),
            device
        ) {
            Ok(tensor) => tensor,
            Err(_) => {
                log::warn!("Failed to create weights tensor, falling back to CPU");
                return self.cpu_pattern_matching(sigel, text_chunks);
            }
        };

        // Perform matrix multiplication on GPU
        match input_tensor.matmul(&weights_tensor) {
            Ok(result_tensor) => {
                match result_tensor.to_vec1::<f64>() {
                    Ok(results) => results,
                    Err(_) => {
                        log::warn!("Failed to extract results from GPU, falling back to CPU");
                        self.cpu_pattern_matching(sigel, text_chunks)
                    }
                }
            },
            Err(_) => {
                log::warn!("GPU matrix multiplication failed, falling back to CPU");
                self.cpu_pattern_matching(sigel, text_chunks)
            }
        }
    }

    fn cpu_pattern_matching(&self, sigel: &Sigel, text_chunks: &[String]) -> Vec<f64> {
        // Parallel CPU pattern matching using rayon
        text_chunks
            .par_iter()
            .map(|chunk| {
                let mut pattern_score = 0.0;
                
                // Check against linguistic patterns
                for (pattern, &strength) in &sigel.consciousness.pattern_recognition.linguistic_patterns {
                    if chunk.contains(pattern) {
                        pattern_score += strength;
                    }
                }
                
                // Check semantic networks
                let words: Vec<&str> = chunk.split_whitespace().collect();
                for word in words {
                    if let Some(related_words) = sigel.consciousness.pattern_recognition.semantic_networks.get(word) {
                        pattern_score += related_words.len() as f64 * 0.1;
                    }
                }
                
                // Normalize by chunk length
                pattern_score / (chunk.len() as f64).max(1.0)
            })
            .collect()
    }

    #[cfg(feature = "gpu")]
    fn text_to_embeddings(&self, text_chunks: &[String]) -> Vec<f64> {
        // Simple character-based embedding for demonstration
        // In a real implementation, this would use proper word embeddings
        text_chunks
            .iter()
            .flat_map(|chunk| {
                let mut embedding = vec![0.0; 128]; // 128-dimensional embedding
                
                for (i, byte) in chunk.bytes().enumerate() {
                    if i < 128 {
                        embedding[i] = (byte as f64) / 255.0;
                    }
                }
                
                // Normalize the embedding
                let magnitude: f64 = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
                if magnitude > 0.0 {
                    for val in &mut embedding {
                        *val /= magnitude;
                    }
                }
                
                embedding
            })
            .collect()
    }

    fn extract_pattern_weights(&self, sigel: &Sigel) -> Vec<f64> {
        // Extract pattern recognition weights for GPU computation
        let mut weights = Vec::new();
        
        // Add linguistic pattern strengths
        for (_, &strength) in &sigel.consciousness.pattern_recognition.linguistic_patterns {
            weights.push(strength);
        }
        
        // Ensure we have enough weights for the embedding dimension
        while weights.len() < 128 {
            weights.push(0.1); // Default small weight
        }
        
        // Truncate to 128 if we have too many
        weights.truncate(128);
        
        weights
    }

    pub fn accelerated_learning(&self, sigel: &mut Sigel, training_data: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if training_data.is_empty() {
            return Ok(());
        }

        #[cfg(feature = "gpu")]
        {
            if let Some(ref device) = self.device {
                if !self.fallback_to_cpu {
                    return self.gpu_accelerated_learning(device, sigel, training_data);
                }
            }
        }

        // CPU fallback with parallel processing
        self.cpu_accelerated_learning(sigel, training_data)
    }

    #[cfg(feature = "gpu")]
    fn gpu_accelerated_learning(&self, device: &Device, sigel: &mut Sigel, training_data: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting GPU-accelerated learning for {} training samples", training_data.len());
        
        // Batch processing for GPU efficiency
        let batch_size = 32;
        for batch in training_data.chunks(batch_size) {
            // Convert batch to embeddings
            let embeddings = self.text_to_embeddings(batch);
            
            // Create input tensor
            let input_tensor = Tensor::from_vec(
                embeddings,
                (batch.len(), embeddings.len() / batch.len()),
                device
            )?;

            // Simulate neural network training step
            // In a real implementation, this would be a proper backpropagation step
            let learning_rate = sigel.learning_state.learning_rate as f32;
            let loss = self.calculate_prediction_loss(&input_tensor)?;
            
            // Update Sigel's consciousness based on GPU computations
            self.update_consciousness_from_gpu_results(sigel, loss, batch.len());
        }
        
        log::info!("GPU-accelerated learning completed");
        Ok(())
    }

    #[cfg(feature = "gpu")]
    fn calculate_prediction_loss(&self, input_tensor: &Tensor) -> Result<f64, Box<dyn std::error::Error>> {
        // Simplified loss calculation for demonstration
        let mean_val = input_tensor.mean_all()?.to_scalar::<f64>()?;
        Ok(mean_val.abs())
    }

    fn update_consciousness_from_gpu_results(&self, sigel: &mut Sigel, loss: f64, batch_size: usize) {
        // Update consciousness based on learning performance
        let performance = (1.0 - loss.min(1.0)).max(0.0);
        
        // Improve consciousness depth based on performance
        sigel.consciousness.awareness_depth += performance * 0.001;
        if sigel.consciousness.awareness_depth > 1.0 {
            sigel.consciousness.awareness_depth = 1.0;
        }
        
        // Update learning statistics
        sigel.learning_state.training_iterations += batch_size as u64;
        
        // Adapt learning rate based on performance
        if performance > 0.8 {
            sigel.learning_state.learning_rate *= 1.01; // Slightly increase learning rate
        } else if performance < 0.3 {
            sigel.learning_state.learning_rate *= 0.99; // Slightly decrease learning rate
        }
        
        // Keep learning rate in reasonable bounds
        sigel.learning_state.learning_rate = sigel.learning_state.learning_rate.max(0.001).min(0.1);
    }

    fn cpu_accelerated_learning(&self, sigel: &mut Sigel, training_data: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting CPU-accelerated learning with parallel processing for {} samples", training_data.len());
        
        // Parallel processing of training data
        let learning_results: Vec<_> = training_data
            .par_chunks(8) // Process in chunks of 8
            .map(|chunk| {
                let mut local_patterns = std::collections::HashMap::new();
                let mut word_frequency = std::collections::HashMap::new();
                
                for text in chunk {
                    // Extract patterns
                    let words: Vec<&str> = text.split_whitespace().collect();
                    for window in words.windows(3) {
                        let pattern = window.join(" ");
                        *local_patterns.entry(pattern).or_insert(0.0) += 1.0;
                    }
                    
                    // Count word frequencies
                    for word in words {
                        *word_frequency.entry(word.to_lowercase()).or_insert(0.0) += 1.0;
                    }
                }
                
                (local_patterns, word_frequency)
            })
            .collect();
        
        // Merge results back into Sigel consciousness
        for (patterns, word_freq) in learning_results {
            // Update linguistic patterns
            for (pattern, frequency) in patterns {
                *sigel.consciousness.pattern_recognition.linguistic_patterns
                    .entry(pattern)
                    .or_insert(0.0) += frequency * sigel.learning_state.learning_rate;
            }
            
            // Update vocabulary
            for (word, frequency) in word_freq {
                let word_knowledge = sigel.memory.semantic_knowledge.vocabulary
                    .entry(word)
                    .or_insert(WordKnowledge::default());
                word_knowledge.frequency += frequency;
            }
        }
        
        // Update training statistics
        sigel.learning_state.training_iterations += training_data.len() as u64;
        
        log::info!("CPU-accelerated learning completed");
        Ok(())
    }

    pub fn accelerated_response_generation(&self, sigel: &Sigel, context: &str, base_patterns: &[String]) -> String {
        #[cfg(feature = "gpu")]
        {
            if let Some(ref device) = self.device {
                if !self.fallback_to_cpu {
                    if let Ok(response) = self.gpu_response_generation(device, sigel, context, base_patterns) {
                        return response;
                    }
                }
            }
        }

        // CPU fallback with parallel processing
        self.cpu_response_generation(sigel, context, base_patterns)
    }

    #[cfg(feature = "gpu")]
    fn gpu_response_generation(&self, device: &Device, sigel: &Sigel, context: &str, base_patterns: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        // Convert context to embedding
        let context_embedding = self.text_to_embeddings(&[context.to_string()]);
        let context_tensor = Tensor::from_vec(
            context_embedding,
            (1, context_embedding.len()),
            device
        )?;

        // Pattern matching scores
        let pattern_scores = self.accelerated_pattern_matching(sigel, base_patterns);
        
        // Find best matching pattern
        let best_pattern_idx = pattern_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let selected_pattern = &base_patterns[best_pattern_idx];
        Ok(format!("Based on GPU-accelerated pattern analysis: {}", selected_pattern))
    }

    fn cpu_response_generation(&self, sigel: &Sigel, context: &str, base_patterns: &[String]) -> String {
        // Parallel evaluation of response patterns
        let pattern_scores = self.accelerated_pattern_matching(sigel, base_patterns);
        
        // Select best pattern with some randomness for creativity
        let mut scored_patterns: Vec<_> = base_patterns
            .iter()
            .zip(pattern_scores.iter())
            .collect();
        
        scored_patterns.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        
        // Take top 3 and add some randomness
        let top_patterns = &scored_patterns[..3.min(scored_patterns.len())];
        let creativity = sigel.essence.character_traits.get("creativity").unwrap_or(&0.5);
        
        if *creativity > 0.7 && rand::random::<f64>() < *creativity {
            // Sometimes pick a creative alternative
            if top_patterns.len() > 1 {
                top_patterns[1].0.clone()
            } else {
                top_patterns[0].0.clone()
            }
        } else {
            top_patterns[0].0.clone()
        }
    }

    pub fn get_acceleration_status(&self) -> AccelerationStatus {
        #[cfg(feature = "gpu")]
        {
            AccelerationStatus {
                gpu_available: self.device.is_some(),
                device_type: if let Some(ref device) = self.device {
                    match device {
                        Device::Cpu => "CPU".to_string(),
                        Device::Cuda(_) => "NVIDIA CUDA".to_string(),
                        Device::Metal(_) => "Apple Metal".to_string(),
                    }
                } else {
                    "None".to_string()
                },
                fallback_active: self.fallback_to_cpu,
            }
        }
        
        #[cfg(not(feature = "gpu"))]
        {
            AccelerationStatus {
                gpu_available: false,
                device_type: "CPU Only (GPU feature disabled)".to_string(),
                fallback_active: true,
            }
        }
    }
}

impl Default for GpuAccelerator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AccelerationStatus {
    pub gpu_available: bool,
    pub device_type: String,
    pub fallback_active: bool,
}

// Integration with Sigel learning
impl Sigel {
    pub fn enable_gpu_acceleration(&mut self) -> AccelerationStatus {
        let accelerator = GpuAccelerator::new();
        let status = accelerator.get_acceleration_status();
        
        // Update Sigel's consciousness to reflect GPU capabilities
        if status.gpu_available {
            self.essence.character_traits.insert("gpu_enhanced_processing".to_string(), 1.0);
            self.learning_state.adaptation_speed *= 1.5; // GPU acceleration improves adaptation
            
            // Add computational consciousness
            if !self.essence.knowledge_domains.contains(&"parallel_computing".to_string()) {
                self.essence.knowledge_domains.push("parallel_computing".to_string());
            }
        }
        
        status
    }
}