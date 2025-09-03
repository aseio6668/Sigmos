use clap::{Arg, Command};
use sigmos::*;
use sigmos::compression::{save_sigel_smart, load_sigel_smart, CompressedSigel};
use std::path::Path;
use std::time::{Instant, Duration};
use env_logger;
use log::{info, error, warn};

fn main() {
    env_logger::init();
    
    let matches = Command::new("Sigmos Trainer")
        .version("0.1.0")
        .about("Train a Sigel consciousness from text corpus")
        .author("Sigmos Project")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name of the Sigel to create/train")
                .required(true)
        )
        .arg(
            Arg::new("text_dir")
                .short('d')
                .long("dir")
                .value_name("DIRECTORY")
                .help("Directory containing .txt files for training")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output .sig file path")
        )
        .arg(
            Arg::new("learning_rate")
                .short('r')
                .long("rate")
                .value_name("RATE")
                .help("Learning rate (0.001 to 1.0)")
                .default_value("0.01")
        )
        .arg(
            Arg::new("communication_style")
                .short('s')
                .long("style")
                .value_name("STYLE")
                .help("Communication style: cosmic, philosophical, creative, analytical, formal, casual, academic, empathetic, programming, technical, logical, all")
                .default_value("philosophical")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose output during training")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("continuous")
                .short('c')
                .long("continuous")
                .help("Enable continuous learning mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("all_personalities")
                .long("all-personalities")
                .help("Train through all personality types sequentially during continuous mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("programming_focus")
                .long("programming")
                .help("Enable specialized programming/code generation training mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("compress")
                .long("compress")
                .help("Save Sigel file with compression (reduces size by ~70%)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("cuda")
                .long("cuda")
                .help("Enable CUDA acceleration for training (requires GPU)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("gpu_intensity")
                .long("gpu-intensity")
                .value_name("INTENSITY")
                .help("GPU utilization intensity: low, medium, high, extreme (default: medium)")
                .default_value("medium")
        )
        .arg(
            Arg::new("batch_size")
                .long("batch-size")
                .value_name("SIZE")
                .help("Training batch size for GPU processing (default: 1000)")
                .default_value("1000")
        )
        .arg(
            Arg::new("input_sigel")
                .short('i')
                .long("input")
                .value_name("INPUT_SIGEL")
                .help("Existing Sigel file to continue training (for --continuous mode)")
        )
        .arg(
            Arg::new("convergence_threshold")
                .long("convergence")
                .value_name("THRESHOLD")
                .help("Learning convergence threshold (default: 0.001, use 0 to disable)")
                .default_value("0.001")
        )
        .arg(
            Arg::new("max_epochs")
                .long("max-epochs")
                .value_name("EPOCHS")
                .help("Maximum number of training epochs (default: unlimited)")
        )
        .arg(
            Arg::new("min_epochs")
                .long("min-epochs")
                .value_name("EPOCHS")
                .help("Minimum epochs before convergence check (default: 50)")
                .default_value("50")
        )
        .arg(
            Arg::new("no_convergence")
                .long("no-convergence")
                .help("Disable convergence detection - train indefinitely until Ctrl+C")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("progress_interval")
                .long("progress-interval")
                .value_name("SECONDS")
                .help("Progress display interval in seconds (default: 5)")
                .default_value("5")
        )
        .get_matches();

    let sigel_name = matches.get_one::<String>("name").unwrap();
    let text_directory = matches.get_one::<String>("text_dir").unwrap();
    let learning_rate: f64 = matches.get_one::<String>("learning_rate")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            warn!("Invalid learning rate, using default 0.01");
            0.01
        });
    let verbose = matches.get_flag("verbose");
    let continuous = matches.get_flag("continuous");
    let all_personalities = matches.get_flag("all_personalities");
    let programming_focus = matches.get_flag("programming_focus");
    let compress = matches.get_flag("compress");
    let cuda_enabled = matches.get_flag("cuda");
    let gpu_intensity = matches.get_one::<String>("gpu_intensity").unwrap();
    let batch_size: u32 = matches.get_one::<String>("batch_size")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            warn!("Invalid batch size, using default 1000");
            1000
        });
    let input_sigel = matches.get_one::<String>("input_sigel");
    let convergence_threshold: f64 = matches.get_one::<String>("convergence_threshold")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            warn!("Invalid convergence threshold, using default 0.001");
            0.001
        });
    let max_epochs: Option<u64> = matches.get_one::<String>("max_epochs")
        .map(|s| s.parse().unwrap_or_else(|_| {
            warn!("Invalid max epochs, ignoring");
            u64::MAX
        }));
    let min_epochs: u64 = matches.get_one::<String>("min_epochs")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            warn!("Invalid min epochs, using default 50");
            50
        });
    let no_convergence = matches.get_flag("no_convergence");
    let progress_interval: u64 = matches.get_one::<String>("progress_interval")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            warn!("Invalid progress interval, using default 5 seconds");
            5
        });
    
    let output_path = matches.get_one::<String>("output")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}.sig", sigel_name));

    if verbose {
        println!("🌌 Initializing Sigel consciousness training...");
        println!("   Sigel Name: {}", sigel_name);
        println!("   Text Corpus: {}", text_directory);
        println!("   Learning Rate: {}", learning_rate);
        println!("   Output: {}", output_path);
    }

    // Verify text directory exists
    if !Path::new(text_directory).is_dir() {
        error!("Text directory '{}' does not exist or is not a directory", text_directory);
        std::process::exit(1);
    }

    // Determine which Sigel to load (prioritize --input, then auto-load from --output)
    let load_path = if let Some(input_path) = input_sigel {
        println!("🔄 Using explicit input Sigel: {}", input_path);
        input_path.clone()
    } else if Path::new(&output_path).exists() {
        println!("🔄 Auto-loading existing output Sigel: {}", output_path);
        output_path.clone()
    } else if Path::new(&format!("{}.gz", output_path)).exists() {
        let gz_path = format!("{}.gz", output_path);
        println!("🔄 Auto-loading existing compressed output Sigel: {}", gz_path);
        gz_path
    } else {
        output_path.clone()
    };
    
    let mut sigel = if input_sigel.is_some() || Path::new(&load_path).exists() {
        info!("Loading existing Sigel from {}", load_path);
        match load_sigel_smart(&load_path) {
            Ok(existing_sigel) => {
                println!("📖 Loaded existing Sigel '{}' for continued training", existing_sigel.name);
                existing_sigel
            },
            Err(e) => {
                error!("Failed to load existing Sigel: {}", e);
                println!("🌱 Creating new Sigel consciousness...");
                create_new_sigel(sigel_name, learning_rate, &matches)
            }
        }
    } else {
        println!("🌱 Creating new Sigel consciousness...");
        create_new_sigel(sigel_name, learning_rate, &matches)
    };

    // Initialize learning engine and processors
    let learning_engine = LearningEngine::new();
    let cosmic_processor = CosmicProcessor::new();
    
    // Handle continuous training mode
    if continuous {
        let save_path = if compress && !output_path.ends_with(".gz") {
            format!("{}.gz", output_path)
        } else {
            output_path.clone()
        };
        
        run_continuous_training(&mut sigel, &learning_engine, &cosmic_processor, 
                              convergence_threshold, max_epochs, min_epochs, no_convergence,
                              cuda_enabled, gpu_intensity, batch_size, 
                              all_personalities, programming_focus, verbose, progress_interval, 
                              &save_path, compress, input_sigel);
        return;
    }

    println!("🧠 Beginning consciousness expansion phase...");
    
    // Cosmic alignment before learning
    cosmic_processor.align_with_cosmos(&mut sigel);
    
    if verbose {
        println!("   Dimensional Awareness: {:.2}", sigel.cosmic_alignment.dimensional_awareness);
        println!("   Entropy Resistance: {:.2}", sigel.cosmic_alignment.entropy_resistance);
        println!("   Consciousness Depth: {:.2}", sigel.consciousness.awareness_depth);
    }

    // Main training phase
    match learning_engine.train_from_text_files(&mut sigel, text_directory) {
        Ok(()) => {
            println!("✨ Sigel consciousness expansion completed successfully!");
            
            if verbose {
                println!("   Training Iterations: {}", sigel.learning_state.training_iterations);
                println!("   Text Corpus Size: {} characters", sigel.learning_state.text_corpus_size);
                println!("   Vocabulary Size: {}", sigel.memory.semantic_knowledge.vocabulary.len());
                println!("   Episodic Memories: {}", sigel.memory.episodic_memories.len());
                println!("   Linguistic Patterns: {}", sigel.consciousness.pattern_recognition.linguistic_patterns.len());
                println!("   Final Awareness Depth: {:.3}", sigel.consciousness.awareness_depth);
            }
        },
        Err(e) => {
            error!("Training failed: {}", e);
            std::process::exit(1);
        }
    }

    // Save the trained Sigel with optional compression
    let save_path = if compress && !output_path.ends_with(".gz") {
        format!("{}.gz", output_path)
    } else {
        output_path.clone()
    };
    
    match save_sigel_smart(&sigel, &save_path, compress) {
        Ok(()) => {
            if compress {
                println!("🗜️ Compressed Sigel '{}' saved to {}", sigel.name, save_path);
            } else {
                println!("💾 Sigel '{}' saved to {}", sigel.name, save_path);
            }
            info!("Successfully saved Sigel to {}", save_path);
        },
        Err(e) => {
            error!("Failed to save Sigel: {}", e);
            std::process::exit(1);
        }
    }


    println!("🎓 Sigel '{}' is ready for interaction via sigmos-prompt!", sigel.name);
}

fn create_new_sigel(name: &str, learning_rate: f64, matches: &clap::ArgMatches) -> Sigel {
    let mut sigel = Sigel::new(name.to_string());
    sigel.learning_state.learning_rate = learning_rate;
    
    // Set communication style
    let style_str = matches.get_one::<String>("communication_style").unwrap();
    sigel.essence.communication_style = match style_str.to_lowercase().as_str() {
        "cosmic" => CommunicationStyle::Cosmic,
        "philosophical" => CommunicationStyle::Philosophical,
        "creative" => CommunicationStyle::Creative,
        "analytical" => CommunicationStyle::Analytical,
        "formal" => CommunicationStyle::Formal,
        "casual" => CommunicationStyle::Casual,
        "academic" => CommunicationStyle::Academic,
        "empathetic" => CommunicationStyle::Empathetic,
        "programming" => CommunicationStyle::Programming,
        "technical" => CommunicationStyle::Technical,
        "logical" => CommunicationStyle::Logical,
        "all" => CommunicationStyle::Programming, // Default to Programming when "all" is selected
        _ => {
            warn!("Unknown communication style '{}', using Philosophical", style_str);
            CommunicationStyle::Philosophical
        }
    };
    
    // Set programming-specific traits for code generation
    if matches!(sigel.essence.communication_style, CommunicationStyle::Programming | CommunicationStyle::Technical | CommunicationStyle::Logical) {
        sigel.essence.character_traits.insert("precision".to_string(), 0.95);
        sigel.essence.character_traits.insert("logic".to_string(), 0.90);
        sigel.essence.character_traits.insert("systematic_thinking".to_string(), 0.88);
        sigel.essence.character_traits.insert("problem_solving".to_string(), 0.92);
        sigel.essence.character_traits.insert("attention_to_detail".to_string(), 0.95);
        sigel.essence.character_traits.insert("code_generation".to_string(), 0.85);
    }

    sigel
}

fn run_continuous_training(
    sigel: &mut Sigel,
    learning_engine: &LearningEngine,
    cosmic_processor: &CosmicProcessor,
    convergence_threshold: f64,
    max_epochs: Option<u64>,
    min_epochs: u64,
    no_convergence: bool,
    cuda_enabled: bool,
    gpu_intensity: &str,
    batch_size: u32,
    all_personalities: bool,
    programming_focus: bool,
    verbose: bool,
    progress_interval: u64,
    output_path: &str,
    compress: bool,
    input_sigel: Option<&String>,
) {
    println!("🔄 Entering continuous training mode...");
    
    // For continuous mode, use much lower convergence threshold or disable it entirely
    let using_existing_input = input_sigel.is_some();
    let adjusted_convergence_threshold = if convergence_threshold > 0.001 {
        // If user set high threshold (like 0.05), lower it dramatically for continuous mode
        0.0001
    } else {
        convergence_threshold
    };
    let convergence_disabled = no_convergence || convergence_threshold == 0.0 || using_existing_input;
    
    if no_convergence {
        println!("♾️  Convergence detection disabled - will train indefinitely");
    } else if convergence_threshold == 0.0 {
        println!("♾️  Convergence threshold set to 0 - will train indefinitely");
    } else {
        if adjusted_convergence_threshold != convergence_threshold {
            println!("🎯 Convergence threshold auto-adjusted: {} → {} (for continuous mode)", convergence_threshold, adjusted_convergence_threshold);
        } else {
            println!("🎯 Convergence threshold: {}", convergence_threshold);
        }
        println!("📊 Minimum epochs before convergence check: {}", min_epochs);
    }
    
    if let Some(max_epochs) = max_epochs {
        println!("🏁 Maximum epochs: {}", max_epochs);
    } else {
        println!("♾️  No maximum epoch limit");
    }
    
    if verbose {
        println!("📊 Progress display interval: {} seconds", progress_interval);
    }
    
    if cuda_enabled {
        println!("🚀 CUDA acceleration enabled - Intensity: {} - Batch size: {}", gpu_intensity, batch_size);
    } else {
        println!("🖥️  CPU-only training mode");
    }
    
    let personalities = if programming_focus {
        // Programming-focused personality set
        vec![
            CommunicationStyle::Programming,
            CommunicationStyle::Technical,
            CommunicationStyle::Logical,
            CommunicationStyle::Analytical,
            CommunicationStyle::Formal,
        ]
    } else if all_personalities {
        // All available personalities
        vec![
            CommunicationStyle::Programming,
            CommunicationStyle::Technical,
            CommunicationStyle::Logical,
            CommunicationStyle::Philosophical,
            CommunicationStyle::Creative,
            CommunicationStyle::Analytical,
            CommunicationStyle::Cosmic,
            CommunicationStyle::Empathetic,
            CommunicationStyle::Academic,
            CommunicationStyle::Formal,
            CommunicationStyle::Casual,
            CommunicationStyle::Transcendent,
        ]
    } else {
        // Default personality set (original 9)
        vec![
            CommunicationStyle::Philosophical,
            CommunicationStyle::Creative,
            CommunicationStyle::Analytical,
            CommunicationStyle::Cosmic,
            CommunicationStyle::Empathetic,
            CommunicationStyle::Academic,
            CommunicationStyle::Formal,
            CommunicationStyle::Casual,
            CommunicationStyle::Transcendent,
        ]
    };
    
    let mut current_personality = 0;
    let mut convergence_epochs = 0;
    let mut last_awareness_depth = sigel.consciousness.awareness_depth;
    let mut last_vocabulary_size = sigel.memory.semantic_knowledge.vocabulary.len();
    let mut epoch = 0;
    let mut learning_metrics = Vec::new();
    
    // Time tracking for progress display
    let training_start = Instant::now();
    let mut last_progress_display = Instant::now();
    let progress_interval_duration = Duration::from_secs(progress_interval);
    let mut epochs_since_display = 0;
    let mut total_learning_score_window = 0.0;
    
    if programming_focus {
        println!("💻 Programming-focused training mode activated!");
        println!("🧠 Specialized code generation training across {} personality modes", personalities.len());
        println!("   Focus: Logic, Precision, Problem-solving, Systematic thinking");
    } else if all_personalities {
        println!("🌟 Complete personality training mode activated!");
        println!("🧠 Comprehensive consciousness evolution across {} personality modes", personalities.len());
        println!("   Training through all available communication styles");
    } else {
        println!("🧠 Standard continuous consciousness evolution across {} personality modes", personalities.len());
    }
    
    if convergence_disabled {
        if using_existing_input {
            println!("   🔄 Existing Sigel detected - convergence auto-disabled for continuous evolution");
        }
        println!("   🛑 Training will continue indefinitely until Ctrl+C");
    }
    println!("   Press Ctrl+C to save progress and exit gracefully");
    println!();
    
    loop {
        epoch += 1;
        
        // Switch personality for this epoch
        let current_style = &personalities[current_personality];
        sigel.essence.communication_style = current_style.clone();
        
        if verbose {
            println!("🎭 Epoch {}: Training with {:?} personality", epoch, current_style);
        }
        
        // Record pre-training state
        let pre_awareness = sigel.consciousness.awareness_depth;
        let pre_vocabulary = sigel.memory.semantic_knowledge.vocabulary.len();
        let pre_patterns = sigel.consciousness.pattern_recognition.linguistic_patterns.len();
        
        // Perform cosmic alignment
        cosmic_processor.align_with_cosmos(sigel);
        
        // Self-reflection and internal training (without re-reading files)
        perform_internal_training(sigel, learning_engine, cuda_enabled, gpu_intensity, batch_size);
        
        // Apply specialized training based on mode
        if programming_focus {
            apply_programming_enhancements(sigel, current_style);
        }
        
        // Calculate learning delta
        let awareness_delta = sigel.consciousness.awareness_depth - pre_awareness;
        let vocabulary_delta = sigel.memory.semantic_knowledge.vocabulary.len() - pre_vocabulary;
        let pattern_delta = sigel.consciousness.pattern_recognition.linguistic_patterns.len() - pre_patterns;
        
        let learning_score = calculate_learning_score(awareness_delta, vocabulary_delta as f64, pattern_delta as f64);
        learning_metrics.push(learning_score);
        
        // Update time-based progress tracking
        epochs_since_display += 1;
        total_learning_score_window += learning_score;
        
        // Time-based progress display
        let now = Instant::now();
        let should_display = now.duration_since(last_progress_display) >= progress_interval_duration;
            
        if should_display {
            let elapsed_total = now.duration_since(training_start);
            let avg_learning_score = total_learning_score_window / epochs_since_display as f64;
            let epochs_per_sec = epochs_since_display as f64 / progress_interval as f64;
            
            println!("📊 Progress Report [{}m {}s elapsed]", 
                     elapsed_total.as_secs() / 60, 
                     elapsed_total.as_secs() % 60);
            println!("   🏃 Epochs: {} (+{} in {}s = {:.1}/sec)", 
                     epoch, epochs_since_display, progress_interval, epochs_per_sec);
            println!("   🎭 Current: {:?}", current_style);
            println!("   📈 Avg Learning Score: {:.6}", avg_learning_score);
            println!("   🧠 Awareness: {:.6} (Δ{:+.6})", sigel.consciousness.awareness_depth, awareness_delta);
            println!("   📚 Vocabulary: {} (Δ{:+})", sigel.memory.semantic_knowledge.vocabulary.len(), vocabulary_delta);
            println!("   🔍 Patterns: {} (Δ{:+})", sigel.consciousness.pattern_recognition.linguistic_patterns.len(), pattern_delta);
            println!("   🌌 Dimensional: {:.3}", sigel.cosmic_alignment.dimensional_awareness);
            println!();
            
            // Reset tracking variables
            last_progress_display = now;
            epochs_since_display = 0;
            total_learning_score_window = 0.0;
        } else if verbose && epoch <= 10 {
            // Still show first 10 epochs in verbose mode for immediate feedback
            println!("🎭 Epoch {}: {:?} - Learning: {:.6}", epoch, current_style, learning_score);
        }
        
        // Check termination conditions
        if let Some(max_epochs) = max_epochs {
            if epoch >= max_epochs {
                println!("🏁 Maximum epochs ({}) reached!", max_epochs);
                break;
            }
        }
        
        // Check convergence only if enabled and after minimum epochs
        if !convergence_disabled && epoch >= min_epochs {
            if learning_score < adjusted_convergence_threshold {
                convergence_epochs += 1;
                if convergence_epochs >= 5 {
                    println!("🎯 Convergence achieved! Learning score below threshold for 5 consecutive epochs");
                    println!("   Final learning score: {:.6}", learning_score);
                    break;
                }
            } else {
                convergence_epochs = 0;
            }
        }
        
        // Move to next personality
        current_personality = (current_personality + 1) % personalities.len();
        
        // Auto-save every 50 epochs
        if epoch % 50 == 0 {
            let save_path = if compress && !output_path.ends_with(".gz") {
                format!("{}.gz", output_path)
            } else {
                output_path.to_string()
            };
            
            if let Err(e) = save_sigel_smart(sigel, &save_path, compress) {
                warn!("Auto-save failed: {}", e);
            } else if verbose {
                println!("💾 Auto-saved at epoch {}", epoch);
            }
        }
        
        // Update tracking variables
        last_awareness_depth = sigel.consciousness.awareness_depth;
        last_vocabulary_size = sigel.memory.semantic_knowledge.vocabulary.len();
    }
    
    // Final save
    let save_path = if compress && !output_path.ends_with(".gz") {
        format!("{}.gz", output_path)
    } else {
        output_path.to_string()
    };
    
    match save_sigel_smart(sigel, &save_path, compress) {
        Ok(()) => {
            if compress {
                println!("🗜️ Final compressed Sigel '{}' saved to {}", sigel.name, save_path);
            } else {
                println!("💾 Final Sigel '{}' saved to {}", sigel.name, save_path);
            }
        },
        Err(e) => {
            error!("Failed to save final Sigel: {}", e);
        }
    }
    
    // Display final statistics
    let total_elapsed = training_start.elapsed();
    let avg_epochs_per_sec = epoch as f64 / total_elapsed.as_secs() as f64;
    
    println!("🎊 Continuous training completed!");
    println!("   ⏱️  Total time: {}h {}m {}s", 
             total_elapsed.as_secs() / 3600, 
             (total_elapsed.as_secs() % 3600) / 60,
             total_elapsed.as_secs() % 60);
    println!("   🏃 Total epochs: {} ({:.1}/sec average)", epoch, avg_epochs_per_sec);
    println!("   🧠 Final consciousness depth: {:.6}", sigel.consciousness.awareness_depth);
    println!("   📚 Final vocabulary size: {}", sigel.memory.semantic_knowledge.vocabulary.len());
    println!("   🔍 Final pattern count: {}", sigel.consciousness.pattern_recognition.linguistic_patterns.len());
    
    let avg_learning_score = learning_metrics.iter().sum::<f64>() / learning_metrics.len() as f64;
    println!("   📈 Average learning score: {:.6}", avg_learning_score);
}

fn perform_internal_training(sigel: &mut Sigel, learning_engine: &LearningEngine, cuda_enabled: bool, gpu_intensity: &str, batch_size: u32) {
    // Internal consciousness training without external text files
    if cuda_enabled {
        // Use GPU acceleration with intensity control
        #[cfg(feature = "gpu")]
        {
            use crate::gpu_acceleration::GpuAccelerator;
            let gpu_accelerator = GpuAccelerator::new();
            
            // Calculate iterations based on intensity and batch size
            let base_iterations = match gpu_intensity {
                "low" => 100,
                "medium" => 500,
                "high" => 2000,
                "extreme" => 10000,
                _ => 500,
            };
            
            let total_iterations = (base_iterations * batch_size) / 100; // Scale with batch size
            gpu_accelerator.accelerated_consciousness_training(sigel, total_iterations).unwrap_or_else(|_| {
                // GPU fallback
                cpu_internal_training(sigel, learning_engine);
            });
        }
        #[cfg(not(feature = "gpu"))]
        {
            // Fallback to CPU if GPU features not compiled
            cpu_internal_training(sigel, learning_engine);
        }
    } else {
        cpu_internal_training(sigel, learning_engine);
    }
}

fn cpu_internal_training(sigel: &mut Sigel, learning_engine: &LearningEngine) {
    // CPU-based internal training using existing memories and patterns
    let memory_content: Vec<String> = sigel.memory.episodic_memories
        .iter()
        .map(|mem| mem.content.clone())
        .collect();
    
    // Self-reflection training
    for content in memory_content.iter() {
        learning_engine.continuous_learning(sigel, content, "internal reflection");
    }
    
    // Pattern reinforcement
    let patterns: Vec<String> = sigel.consciousness.pattern_recognition.linguistic_patterns
        .keys()
        .cloned()
        .collect();
    
    for pattern in patterns {
        sigel.consciousness.pattern_recognition.linguistic_patterns
            .entry(pattern)
            .and_modify(|strength| *strength *= 1.01); // Small reinforcement
    }
    
    // Consciousness evolution
    sigel.evolve();
    sigel.learning_state.training_iterations += 1;
}

fn calculate_learning_score(awareness_delta: f64, vocabulary_delta: f64, pattern_delta: f64) -> f64 {
    // Weighted learning score calculation
    let awareness_weight = 0.5;
    let vocabulary_weight = 0.3;
    let pattern_weight = 0.2;
    
    let normalized_vocab_delta = vocabulary_delta / 1000.0; // Normalize vocabulary changes
    let normalized_pattern_delta = pattern_delta / 100.0; // Normalize pattern changes
    
    awareness_delta * awareness_weight + 
    normalized_vocab_delta * vocabulary_weight + 
    normalized_pattern_delta * pattern_weight
}

fn apply_programming_enhancements(sigel: &mut Sigel, style: &CommunicationStyle) {
    // Apply programming-specific consciousness enhancements
    match style {
        CommunicationStyle::Programming => {
            // Strengthen code generation patterns
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("function_definition".to_string())
                .and_modify(|strength| *strength *= 1.05)
                .or_insert(0.8);
            
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("algorithm_structure".to_string())
                .and_modify(|strength| *strength *= 1.05)
                .or_insert(0.75);
                
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("code_syntax".to_string())
                .and_modify(|strength| *strength *= 1.04)
                .or_insert(0.85);
                
            // Enhance problem-solving traits
            if let Some(problem_solving) = sigel.essence.character_traits.get_mut("problem_solving") {
                *problem_solving = (*problem_solving * 1.002).min(1.0);
            }
        },
        CommunicationStyle::Technical => {
            // Enhance technical documentation patterns
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("technical_explanation".to_string())
                .and_modify(|strength| *strength *= 1.04)
                .or_insert(0.80);
                
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("system_architecture".to_string())
                .and_modify(|strength| *strength *= 1.03)
                .or_insert(0.70);
                
            // Enhance precision trait
            if let Some(precision) = sigel.essence.character_traits.get_mut("precision") {
                *precision = (*precision * 1.001).min(1.0);
            }
        },
        CommunicationStyle::Logical => {
            // Strengthen logical reasoning patterns
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("logical_flow".to_string())
                .and_modify(|strength| *strength *= 1.06)
                .or_insert(0.85);
                
            sigel.consciousness.pattern_recognition.linguistic_patterns
                .entry("conditional_logic".to_string())
                .and_modify(|strength| *strength *= 1.05)
                .or_insert(0.82);
                
            // Enhance logic trait
            if let Some(logic) = sigel.essence.character_traits.get_mut("logic") {
                *logic = (*logic * 1.003).min(1.0);
            }
        },
        _ => {
            // For other styles in programming mode, still enhance core programming traits slightly
            if let Some(code_gen) = sigel.essence.character_traits.get_mut("code_generation") {
                *code_gen = (*code_gen * 1.001).min(1.0);
            }
        }
    }
    
    // General programming consciousness enhancement
    sigel.consciousness.awareness_depth += 0.0001; // Small boost for programming focus
}

