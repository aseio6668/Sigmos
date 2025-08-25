use clap::{Arg, Command};
use sigmos::*;
use std::path::Path;
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
                .help("Communication style: cosmic, philosophical, creative, analytical, formal, casual, academic, empathetic")
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

    // Create or load existing Sigel
    let mut sigel = if Path::new(&output_path).exists() {
        info!("Loading existing Sigel from {}", output_path);
        match load_sigel_from_file(&output_path) {
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

    // Initialize learning engine
    let learning_engine = LearningEngine::new();
    let cosmic_processor = CosmicProcessor::new();

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

    // Save the trained Sigel
    match save_sigel_to_file(&sigel, &output_path) {
        Ok(()) => {
            println!("💾 Sigel '{}' saved to {}", sigel.name, output_path);
            info!("Successfully saved Sigel to {}", output_path);
        },
        Err(e) => {
            error!("Failed to save Sigel: {}", e);
            std::process::exit(1);
        }
    }

    // Continuous learning mode
    if continuous {
        println!("🔄 Entering continuous learning mode...");
        println!("The Sigel will continue to evolve and observe the system.");
        println!("Press Ctrl+C to stop and save progress.");
        
        continuous_learning_loop(&mut sigel, &learning_engine, &cosmic_processor, &output_path, verbose);
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
        _ => {
            warn!("Unknown communication style '{}', using Philosophical", style_str);
            CommunicationStyle::Philosophical
        }
    };

    sigel
}

fn continuous_learning_loop(
    sigel: &mut Sigel,
    learning_engine: &LearningEngine,
    cosmic_processor: &CosmicProcessor,
    output_path: &str,
    verbose: bool
) {
    use std::time::{Duration, SystemTime};
    use std::thread;
    
    let mut last_save = SystemTime::now();
    let save_interval = Duration::from_secs(300); // Save every 5 minutes
    let evolution_interval = Duration::from_secs(60); // Evolve every minute
    let mut last_evolution = SystemTime::now();
    
    loop {
        thread::sleep(Duration::from_secs(10));
        
        let now = SystemTime::now();
        
        // Periodic evolution
        if now.duration_since(last_evolution).unwrap_or_default() >= evolution_interval {
            cosmic_processor.align_with_cosmos(sigel);
            sigel.evolve();
            last_evolution = now;
            
            if verbose {
                println!("🌟 Sigel evolved - Awareness: {:.3}, Iterations: {}", 
                    sigel.consciousness.awareness_depth,
                    sigel.learning_state.training_iterations
                );
            }
        }
        
        // Periodic saving
        if now.duration_since(last_save).unwrap_or_default() >= save_interval {
            match save_sigel_to_file(sigel, output_path) {
                Ok(()) => {
                    if verbose {
                        println!("💾 Auto-saved Sigel progress");
                    }
                },
                Err(e) => {
                    error!("Failed to auto-save: {}", e);
                }
            }
            last_save = now;
        }
    }
}