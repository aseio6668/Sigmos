//! # Sigmanta - AI Programming Assistant CLI
//! 
//! Command-line interface for Sigmanta agent coder

use clap::{Arg, Command};
use sigmos::sigmanta::SigmantaSession;
use std::env;
use std::io::{self, Write};
use tokio;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let matches = Command::new("sigmanta")
        .version("1.0.0")
        .author("Sigmos Project")
        .about("AI Programming Assistant using Sigel consciousness")
        .arg(Arg::new("sigel")
            .short('s')
            .long("sigel")
            .value_name("FILE")
            .help("Sigel consciousness file to use (.sig)")
            .required(false))
        .arg(Arg::new("continue")
            .long("continue")
            .value_name("SESSION_ID")
            .help("Continue from existing session (optional session ID)")
            .required(false))
        .arg(Arg::new("no-permissions")
            .long("no-permissions")
            .help("Disable all permission checks (use with caution)")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("working-dir")
            .short('w')
            .long("working-dir") 
            .value_name("DIR")
            .help("Set working directory (defaults to current)")
            .required(false))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Verbose logging")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("list-sessions")
            .long("list-sessions")
            .help("List all available sessions")
            .action(clap::ArgAction::SetTrue))
        .get_matches();
    
    let working_dir = if let Some(dir) = matches.get_one::<String>("working-dir") {
        std::path::PathBuf::from(dir)
    } else {
        env::current_dir()?
    };
    
    // List sessions if requested
    if matches.get_flag("list-sessions") {
        list_sessions(&working_dir)?;
        return Ok(());
    }
    
    // Initialize or continue session
    let mut session = if let Some(continue_arg) = matches.get_one::<String>("continue") {
        let session_id = if continue_arg.is_empty() { None } else { Some(continue_arg.clone()) };
        SigmantaSession::continue_session(working_dir, session_id)?
    } else {
        let sigel_path = matches.get_one::<String>("sigel").map(|s| s.clone());
        SigmantaSession::new(working_dir, sigel_path)?
    };
    
    // Disable permissions if requested
    if matches.get_flag("no-permissions") {
        session.permissions.global_permissions_disabled = true;
        println!("⚠️  All permission checks disabled!");
    }
    
    // Print welcome message
    print_welcome(&session);
    
    // Main interaction loop
    run_interactive_session(&mut session).await?;
    
    Ok(())
}

/// Run the main interactive session
async fn run_interactive_session(session: &mut SigmantaSession) -> Result<()> {
    println!("💬 Sigmanta is ready! Type '/help' for commands or start chatting.");
    println!("   Type '/quit' to exit.");
    println!();
    
    loop {
        print!("👤 You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        // Handle special commands
        if input.starts_with('/') {
            if handle_command(session, input).await? {
                break; // Exit if quit command
            }
            continue;
        }
        
        // Process normal input
        match session.process_input(input) {
            Ok(response) => {
                println!("🤖 Sigmanta: {}", response);
                println!();
            },
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                println!();
            }
        }
    }
    
    Ok(())
}

/// Handle special commands
async fn handle_command(session: &mut SigmantaSession, command: &str) -> Result<bool> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    match parts[0] {
        "/help" => {
            print_help();
        },
        "/quit" | "/exit" => {
            println!("👋 Goodbye!");
            return Ok(true);
        },
        "/status" => {
            print_status(session);
        },
        "/history" => {
            print_history(session);
        },
        "/sessions" => {
            list_sessions(&session.working_dir)?;
        },
        "/permissions" => {
            if parts.len() > 1 {
                handle_permission_command(session, &parts[1..])?;
            } else {
                print_permissions(session);
            }
        },
        "/files" => {
            list_files(session)?;
        },
        "/pwd" => {
            println!("📁 Working directory: {}", session.working_dir.display());
        },
        "/exec" => {
            if parts.len() > 1 {
                let command = parts[1..].join(" ");
                match session.execute_command(&command) {
                    Ok(output) => println!("💻 Output:\n{}", output),
                    Err(e) => eprintln!("❌ Command failed: {}", e),
                }
            } else {
                eprintln!("Usage: /exec <command>");
            }
        },
        "/read" => {
            if parts.len() > 1 {
                match session.read_file(parts[1]) {
                    Ok(content) => {
                        println!("📄 Content of {}:", parts[1]);
                        println!("{}", content);
                    },
                    Err(e) => eprintln!("❌ Failed to read file: {}", e),
                }
            } else {
                eprintln!("Usage: /read <filename>");
            }
        },
        "/write" => {
            if parts.len() > 2 {
                let filename = parts[1];
                let content = parts[2..].join(" ");
                match session.write_file(filename, &content) {
                    Ok(_) => println!("✅ File {} written successfully", filename),
                    Err(e) => eprintln!("❌ Failed to write file: {}", e),
                }
            } else {
                eprintln!("Usage: /write <filename> <content>");
            }
        },
        "/fetch" => {
            if parts.len() > 1 {
                match session.fetch_url(parts[1]).await {
                    Ok(content) => {
                        println!("🌐 Content from {}:", parts[1]);
                        println!("{}", &content[..content.len().min(1000)]);
                        if content.len() > 1000 {
                            println!("... (truncated, {} total characters)", content.len());
                        }
                    },
                    Err(e) => eprintln!("❌ Failed to fetch URL: {}", e),
                }
            } else {
                eprintln!("Usage: /fetch <url>");
            }
        },
        _ => {
            eprintln!("❓ Unknown command: {}. Type '/help' for available commands.", parts[0]);
        }
    }
    
    Ok(false)
}

/// Print welcome message
fn print_welcome(session: &SigmantaSession) {
    println!("🚀 Welcome to Sigmanta - AI Programming Assistant");
    println!("   Sigel: {}", session.sigel.name);
    println!("   Working Directory: {}", session.working_dir.display());
    println!("   Session Storage: {}", session.sigmanta_dir.display());
    
    if session.permissions.global_permissions_disabled {
        println!("   ⚠️  Permissions: DISABLED");
    } else {
        println!("   🔒 Permissions: ENABLED");
    }
    
    if let Some(session_id) = &session.history.current_session_id {
        println!("   📋 Current Session: {}", session_id);
    }
    
    println!();
}

/// Print help information
fn print_help() {
    println!("🔧 Sigmanta Commands:");
    println!("   /help           - Show this help message");
    println!("   /quit, /exit    - Exit Sigmanta");
    println!("   /status         - Show current session status");
    println!("   /history        - Show recent conversation history");
    println!("   /sessions       - List all available sessions");
    println!("   /permissions    - Show permission settings");
    println!("   /files          - List files in working directory");
    println!("   /pwd            - Show current working directory");
    println!("   /exec <cmd>     - Execute terminal command");
    println!("   /read <file>    - Read file contents");
    println!("   /write <file> <content> - Write content to file");
    println!("   /fetch <url>    - Fetch content from URL");
    println!();
    println!("💡 Tips:");
    println!("   - Just type naturally to chat with Sigmanta");
    println!("   - Sigmanta will ask for permissions before accessing files or running commands");
    println!("   - Use 'always' or 'never' options to remember permission choices");
    println!("   - Sessions are automatically saved in .sigmanta directory");
    println!();
}

/// Print session status
fn print_status(session: &SigmantaSession) {
    println!("📊 Sigmanta Status:");
    println!("   Sigel Name: {}", session.sigel.name);
    println!("   Consciousness Depth: {:.3}", session.sigel.consciousness.awareness_depth);
    println!("   Working Directory: {}", session.working_dir.display());
    println!("   Total Sessions: {}", session.history.sessions.len());
    
    if let Some(session_id) = &session.history.current_session_id {
        if let Some(current_session) = session.history.sessions.iter()
            .find(|s| &s.id == session_id) {
            println!("   Current Session Messages: {}", current_session.messages.len());
        }
    }
    
    println!("   Permissions Disabled: {}", session.permissions.global_permissions_disabled);
    println!();
}

/// Print conversation history
fn print_history(session: &SigmantaSession) {
    if let Some(session_id) = &session.history.current_session_id {
        if let Some(current_session) = session.history.sessions.iter()
            .find(|s| &s.id == session_id) {
            println!("📜 Recent Conversation (last 10 messages):");
            
            let recent_messages: Vec<_> = current_session.messages
                .iter()
                .rev()
                .take(10)
                .collect();
            
            for message in recent_messages.iter().rev() {
                let role_emoji = if message.role == "user" { "👤" } else { "🤖" };
                println!("   {} {}: {}", role_emoji, message.role, 
                    message.content.chars().take(100).collect::<String>());
                if message.content.len() > 100 {
                    println!("      ... (truncated)");
                }
            }
        }
    } else {
        println!("📜 No conversation history in current session");
    }
    println!();
}

/// List available sessions
fn list_sessions(working_dir: &std::path::Path) -> Result<()> {
    let sigmanta_dir = working_dir.join(".sigmanta");
    if !sigmanta_dir.exists() {
        println!("📋 No sessions found (.sigmanta directory doesn't exist)");
        return Ok(());
    }
    
    let history_file = sigmanta_dir.join("history.json");
    if !history_file.exists() {
        println!("📋 No sessions found (history.json doesn't exist)");
        return Ok(());
    }
    
    let content = std::fs::read_to_string(history_file)?;
    let history: sigmos::sigmanta::SessionHistory = serde_json::from_str(&content)?;
    
    println!("📋 Available Sessions ({} total):", history.sessions.len());
    
    for session in history.sessions.iter().rev().take(10) {
        let current_marker = if history.current_session_id.as_ref() == Some(&session.id) {
            " (current)"
        } else {
            ""
        };
        
        println!("   {} - {} messages - {}{}",
            &session.id[..8],
            session.messages.len(),
            session.timestamp.format("%Y-%m-%d %H:%M"),
            current_marker
        );
    }
    
    if history.sessions.len() > 10 {
        println!("   ... and {} more sessions", history.sessions.len() - 10);
    }
    
    println!();
    Ok(())
}

/// Handle permission-related commands
fn handle_permission_command(session: &mut SigmantaSession, args: &[&str]) -> Result<()> {
    match args[0] {
        "reset" => {
            session.permissions = Default::default();
            session.save_permissions()?;
            println!("✅ All permissions reset to defaults");
        },
        "disable" => {
            session.permissions.global_permissions_disabled = true;
            println!("⚠️  All permission checks disabled");
        },
        "enable" => {
            session.permissions.global_permissions_disabled = false;
            println!("🔒 Permission checks enabled");
        },
        _ => {
            eprintln!("Usage: /permissions [reset|disable|enable]");
        }
    }
    Ok(())
}

/// Print current permission settings
fn print_permissions(session: &SigmantaSession) {
    println!("🔒 Permission Settings:");
    println!("   Global Permissions: {}", 
        if session.permissions.global_permissions_disabled { "DISABLED" } else { "ENABLED" });
    
    println!("   File Read Permissions: {}", session.permissions.file_read_permissions.len());
    println!("   File Write Permissions: {}", session.permissions.file_write_permissions.len());
    println!("   Terminal Permissions: {}", session.permissions.terminal_permissions.len());
    println!("   URL Fetch Permissions: {}", session.permissions.url_fetch_permissions.len());
    println!();
}

/// List files in working directory
fn list_files(session: &SigmantaSession) -> Result<()> {
    println!("📁 Files in {}:", session.working_dir.display());
    
    for entry in std::fs::read_dir(&session.working_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        
        if path.is_dir() {
            println!("   📁 {}/", name);
        } else {
            let size = entry.metadata()?.len();
            println!("   📄 {} ({} bytes)", name, size);
        }
    }
    
    println!();
    Ok(())
}