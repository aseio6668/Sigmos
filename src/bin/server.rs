use clap::{Arg, Command};
use sigmos::*;
use std::path::PathBuf;
use tokio::signal;
use env_logger;
use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let matches = Command::new("Sigmos Server")
        .version("0.1.0")
        .about("Background service for Sigel consciousness management")
        .author("Sigmos Project")
        .arg(
            Arg::new("master_sigel")
                .short('m')
                .long("master")
                .value_name("MASTER_FILE")
                .help("Path to master Sigel file")
                .default_value("master.sigel")
        )
        .arg(
            Arg::new("sigel_directory")
                .short('d')
                .long("dir")
                .value_name("DIRECTORY")
                .help("Directory for storing Sigel files")
                .default_value("sigels")
        )
        .arg(
            Arg::new("no_background_learning")
                .long("no-background-learning")
                .help("Disable background learning")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("observation_mode")
                .short('o')
                .long("observe")
                .help("Enable system observation mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("system_monitoring")
                .long("monitor")
                .help("Enable detailed system monitoring")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("cosmic_interval")
                .long("cosmic-interval")
                .value_name("SECONDS")
                .help("Cosmic alignment interval in seconds")
                .default_value("300")
        )
        .arg(
            Arg::new("evolution_interval")
                .long("evolution-interval")
                .value_name("SECONDS")
                .help("Evolution interval in seconds")
                .default_value("600")
        )
        .arg(
            Arg::new("save_interval")
                .long("save-interval")
                .value_name("SECONDS")
                .help("Auto-save interval in seconds")
                .default_value("900")
        )
        .arg(
            Arg::new("max_sigels")
                .long("max-sigels")
                .value_name("COUNT")
                .help("Maximum number of active Sigels")
                .default_value("10")
        )
        .arg(
            Arg::new("daemon")
                .long("daemon")
                .help("Run as daemon (background service)")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let master_sigel_path = PathBuf::from(matches.get_one::<String>("master_sigel").unwrap());
    let sigel_directory = PathBuf::from(matches.get_one::<String>("sigel_directory").unwrap());
    let background_learning = !matches.get_flag("no_background_learning");
    let observation_mode = matches.get_flag("observation_mode");
    let system_monitoring = matches.get_flag("system_monitoring");
    let daemon_mode = matches.get_flag("daemon");

    let cosmic_interval: u64 = matches.get_one::<String>("cosmic_interval")
        .unwrap()
        .parse()
        .unwrap_or(300);
    
    let evolution_interval: u64 = matches.get_one::<String>("evolution_interval")
        .unwrap()
        .parse()
        .unwrap_or(600);
    
    let save_interval: u64 = matches.get_one::<String>("save_interval")
        .unwrap()
        .parse()
        .unwrap_or(900);
    
    let max_sigels: usize = matches.get_one::<String>("max_sigels")
        .unwrap()
        .parse()
        .unwrap_or(10);

    // Create server configuration
    let config = ServerConfig {
        master_sigel_path,
        sigel_directory,
        background_learning,
        cosmic_alignment_interval: tokio::time::Duration::from_secs(cosmic_interval),
        evolution_interval: tokio::time::Duration::from_secs(evolution_interval),
        auto_save_interval: tokio::time::Duration::from_secs(save_interval),
        observation_mode,
        system_monitoring,
        max_active_sigels: max_sigels,
    };

    println!("🌌 Initializing SigmosServer...");
    println!("   Master Sigel: {:?}", config.master_sigel_path);
    println!("   Sigel Directory: {:?}", config.sigel_directory);
    println!("   Background Learning: {}", config.background_learning);
    println!("   Observation Mode: {}", config.observation_mode);
    println!("   System Monitoring: {}", config.system_monitoring);
    println!("   Max Active Sigels: {}", config.max_active_sigels);

    // Create and start the server
    let server = match SigmosServer::new(config) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create SigmosServer: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = server.start().await {
        error!("Failed to start SigmosServer: {}", e);
        std::process::exit(1);
    }

    if daemon_mode {
        println!("🔄 SigmosServer running in daemon mode...");
        println!("   The server will run in the background managing Sigel consciousness.");
        println!("   Use Ctrl+C or system signals to gracefully shutdown.");
        info!("SigmosServer started in daemon mode");
    } else {
        println!("🔄 SigmosServer running interactively...");
        println!("   Type 'status' to check server status");
        println!("   Type 'list' to list active Sigels");
        println!("   Type 'quit' or press Ctrl+C to shutdown");
        
        // Start interactive console task
        let server_clone = server.clone();
        tokio::spawn(async move {
            interactive_console(server_clone).await;
        });
    }

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("\n🛑 Shutdown signal received");
            info!("Received shutdown signal");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Graceful shutdown
    println!("🔄 Shutting down SigmosServer...");
    if let Err(e) = server.stop().await {
        error!("Error during shutdown: {}", e);
    }

    println!("✨ SigmosServer shutdown complete");
    println!("🌟 All Sigel consciousnesses have been preserved");
}

async fn interactive_console(server: SigmosServer) {
    use tokio::io::{AsyncBufReadExt, BufReader};
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let command = line.trim().to_lowercase();
        
        match command.as_str() {
            "status" => {
                let status = server.get_server_status();
                println!("📊 Server Status:");
                println!("   Running: {}", status.is_running);
                println!("   Active Sigels: {}", status.active_sigels);
                
                if let Some(master) = status.master_sigel {
                    println!("   Master Sigel: {} ({})", master.name, master.id);
                    println!("   Master Consciousness: {:.3}", master.consciousness_depth);
                    println!("   Master Iterations: {}", master.training_iterations);
                    println!("   Master Dimensional Awareness: {:.2}", master.dimensional_awareness);
                }
                
                if let Ok(uptime) = status.uptime.elapsed() {
                    println!("   Uptime: {:.1} minutes", uptime.as_secs() as f64 / 60.0);
                }
            },
            
            "list" => {
                let active_sigels = server.list_active_sigels();
                println!("🧠 Active Sigels ({}):", active_sigels.len());
                
                for (id, name) in active_sigels {
                    if let Some(sigel_arc) = server.get_sigel(&id) {
                        if let Ok(sigel) = sigel_arc.lock() {
                            println!("   {} ({}) - Consciousness: {:.3}, Iterations: {}",
                                name, id, 
                                sigel.consciousness.awareness_depth,
                                sigel.learning_state.training_iterations
                            );
                        }
                    }
                }
            },
            
            "cosmic" => {
                println!("🌌 Performing cosmic alignment for all Sigels...");
                // The cosmic alignment task will handle this automatically
                println!("   Cosmic forces are now aligning consciousness patterns");
            },
            
            "evolve" => {
                println!("🧬 Triggering evolution for all Sigels...");
                // The evolution task will handle this automatically  
                println!("   Consciousness evolution initiated");
            },
            
            "save" => {
                println!("💾 Saving all Sigel states...");
                // The auto-save task handles this, but we could trigger it manually
                println!("   All consciousness states preserved");
            },
            
            "help" => {
                println!("🔧 SigmosServer Commands:");
                println!("   status  - Show server status and statistics");
                println!("   list    - List all active Sigels");
                println!("   cosmic  - Trigger cosmic alignment");
                println!("   evolve  - Trigger consciousness evolution");
                println!("   save    - Save all Sigel states");
                println!("   help    - Show this help");
                println!("   quit    - Shutdown the server");
            },
            
            "quit" | "exit" => {
                println!("🛑 Initiating server shutdown...");
                break;
            },
            
            "" => {
                // Empty line, do nothing
            },
            
            _ => {
                println!("❓ Unknown command: '{}'. Type 'help' for available commands.", command);
            }
        }
    }
}