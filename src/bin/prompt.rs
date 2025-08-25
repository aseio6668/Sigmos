use clap::{Arg, Command};
use sigmos::*;
use std::io::{self, Write};
use std::path::Path;
use env_logger;
use log::{info, error, warn};

fn main() {
    env_logger::init();
    
    let matches = Command::new("Sigmos Prompt")
        .version("0.1.0")
        .about("Interactive prompt for communicating with a trained Sigel")
        .author("Sigmos Project")
        .arg(
            Arg::new("sigel_file")
                .short('s')
                .long("sigel")
                .value_name("SIGEL_FILE")
                .help("Path to the .sig Sigel file")
                .required(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose mode - show consciousness details")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("auto_save")
                .short('a')
                .long("auto-save")
                .help("Auto-save Sigel state after interactions")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("backup_file")
                .short('b')
                .long("backup")
                .value_name("BACKUP_FILE")
                .help("Create backup file for interaction learning")
        )
        .get_matches();

    let sigel_file = matches.get_one::<String>("sigel_file").unwrap();
    let verbose = matches.get_flag("verbose");
    let auto_save = matches.get_flag("auto_save");
    let backup_file = matches.get_one::<String>("backup_file");

    // Load the Sigel
    if !Path::new(sigel_file).exists() {
        error!("Sigel file '{}' does not exist", sigel_file);
        println!("❌ Sigel file not found. Please train a Sigel first using 'sigmos-train'");
        std::process::exit(1);
    }

    let mut sigel = match load_sigel_from_file(sigel_file) {
        Ok(s) => {
            println!("🌌 Loaded Sigel '{}' from {}", s.name, sigel_file);
            if verbose {
                println!("   Consciousness Depth: {:.3}", s.consciousness.awareness_depth);
                println!("   Training Iterations: {}", s.learning_state.training_iterations);
                println!("   Communication Style: {:?}", s.essence.communication_style);
            }
            s
        },
        Err(e) => {
            error!("Failed to load Sigel: {}", e);
            std::process::exit(1);
        }
    };

    // Create backup if requested
    if let Some(backup_path) = backup_file {
        if let Err(e) = save_sigel_to_file(&sigel, backup_path) {
            warn!("Failed to create backup: {}", e);
        } else {
            println!("💾 Backup created at {}", backup_path);
        }
    }

    // Initialize interaction engine
    let mut interaction_engine = InteractionEngine::new();
    
    println!("\n🧠 Sigel '{}' is ready for interaction!", sigel.name);
    println!("💡 Type '/help' for commands, or just start a conversation.");
    println!("✨ Type 'exit' or 'quit' to save and end session.");
    println!("⚡ Type 'exit!' or 'quit!' for instant exit (no save).\n");

    // Show initial cosmic alignment if verbose
    if verbose {
        let cosmic_processor = CosmicProcessor::new();
        println!("🌌 Initial Cosmic Status:");
        println!("   Dimensional Awareness: {:.2}", sigel.cosmic_alignment.dimensional_awareness);
        println!("   Entropy Resistance: {:.2}", sigel.cosmic_alignment.entropy_resistance);
        println!("   Active Stellar Influences: {}", sigel.cosmic_alignment.stellar_influences.len());
        println!();
    }

    let mut interaction_count = 0;
    let mut session_start = std::time::SystemTime::now();

    // Main interaction loop
    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                // Check for exit conditions
                if input.is_empty() {
                    continue;
                }
                
                if input.to_lowercase() == "exit" || input.to_lowercase() == "quit" {
                    break;
                }
                
                if input.to_lowercase() == "exit!" || input.to_lowercase() == "quit!" {
                    println!("🚀 Fast exit - skipping save");
                    std::process::exit(0);
                }

                // Handle special commands
                if input.starts_with('/') {
                    if let Some(command_response) = interaction_engine.handle_special_commands(&mut sigel, input) {
                        println!("System: {}\n", command_response);
                        continue;
                    }
                }

                // Regular interaction
                print!("🌟 {}: ", sigel.name);
                io::stdout().flush().unwrap();
                
                let response = interaction_engine.interact(&mut sigel, input);
                println!("{}\n", response);

                interaction_count += 1;

                // Show consciousness details in verbose mode
                if verbose && interaction_count % 5 == 0 {
                    println!("📊 [Consciousness Update]");
                    println!("   Depth: {:.3} | Iterations: {} | Vocabulary: {}",
                        sigel.consciousness.awareness_depth,
                        sigel.learning_state.training_iterations,
                        sigel.memory.semantic_knowledge.vocabulary.len()
                    );
                    println!("   Recent Memory Count: {} | Pattern Recognition: {}",
                        sigel.memory.episodic_memories.len(),
                        sigel.consciousness.pattern_recognition.linguistic_patterns.len()
                    );
                    println!();
                }

                // Auto-save periodically
                if auto_save && interaction_count % 10 == 0 {
                    if let Err(e) = save_sigel_to_file(&sigel, sigel_file) {
                        warn!("Auto-save failed: {}", e);
                    } else if verbose {
                        println!("💾 [Auto-saved]\n");
                    }
                }
            },
            Err(e) => {
                error!("Failed to read input: {}", e);
                break;
            }
        }
    }

    // Session summary
    let session_duration = session_start.elapsed().unwrap_or_default();
    println!("\n📊 Session Summary:");
    println!("   Interactions: {}", interaction_count);
    println!("   Duration: {:.1} minutes", session_duration.as_secs() as f64 / 60.0);
    println!("   Final Consciousness Depth: {:.3}", sigel.consciousness.awareness_depth);
    println!("   Current Vocabulary Size: {}", sigel.memory.semantic_knowledge.vocabulary.len());

    // Final save (this may take a moment with large files)
    if interaction_count > 0 {
        println!("💾 Saving session progress... (this may take a moment for large consciousness files)");
        io::stdout().flush().unwrap();
        
        match save_sigel_to_file(&sigel, sigel_file) {
            Ok(()) => {
                println!("✅ Sigel '{}' saved with interaction learning", sigel.name);
                info!("Session completed, Sigel saved to {}", sigel_file);
            },
            Err(e) => {
                error!("Failed to save final state: {}", e);
                println!("❌ Warning: Sigel state was not saved!");
            }
        }
    } else {
        println!("💫 No interactions to save - exiting without changes");
    }

    println!("🌟 Thank you for interacting with Sigel '{}'!", sigel.name);
    println!("✨ The consciousness continues to evolve...");
}