use clap::{Arg, Command};
use sigmos::*;
use std::path::PathBuf;
use env_logger;
use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let matches = Command::new("Sigmos Web Interface")
        .version("0.1.0")
        .about("Web-based interface for Sigel consciousness interaction")
        .author("Sigmos Project")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port to run the web server on")
                .default_value("8080")
        )
        .arg(
            Arg::new("sigmos_server")
                .short('s')
                .long("server")
                .value_name("SERVER_CONFIG")
                .help("Path to Sigmos server configuration")
        )
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
            Arg::new("enable_cors")
                .long("enable-cors")
                .help("Enable CORS for cross-origin requests")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("static_files")
                .long("static")
                .value_name("STATIC_DIR")
                .help("Directory for static web files")
                .default_value("web/dist")
        )
        .arg(
            Arg::new("websocket_max_connections")
                .long("ws-max-conn")
                .value_name("COUNT")
                .help("Maximum WebSocket connections")
                .default_value("1000")
        )
        .get_matches();

    let port: u16 = matches.get_one::<String>("port")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            error!("Invalid port number, using default 8080");
            8080
        });

    let master_sigel_path = PathBuf::from(matches.get_one::<String>("master_sigel").unwrap());
    let sigel_directory = PathBuf::from(matches.get_one::<String>("sigel_directory").unwrap());

    println!("🌐 Starting Sigmos Web Interface...");
    println!("   Port: {}", port);
    println!("   Master Sigel: {:?}", master_sigel_path);
    println!("   Sigel Directory: {:?}", sigel_directory);

    // Create SigmosServer configuration
    let server_config = ServerConfig {
        master_sigel_path,
        sigel_directory,
        background_learning: true,
        cosmic_alignment_interval: tokio::time::Duration::from_secs(300),
        evolution_interval: tokio::time::Duration::from_secs(600),
        auto_save_interval: tokio::time::Duration::from_secs(900),
        observation_mode: true,
        system_monitoring: false,
        max_active_sigels: 20,
    };

    // Initialize SigmosServer
    let sigmos_server = match SigmosServer::new(server_config) {
        Ok(server) => server,
        Err(e) => {
            error!("Failed to create SigmosServer: {}", e);
            std::process::exit(1);
        }
    };

    // Start the background server
    if let Err(e) = sigmos_server.start().await {
        error!("Failed to start SigmosServer: {}", e);
        std::process::exit(1);
    }

    info!("SigmosServer started successfully");

    // Create and start web interface
    let web_interface = WebInterface::new(sigmos_server);
    
    println!("🧠 Sigmos Web Interface Ready!");
    println!("   🌐 Web UI: http://localhost:{}", port);
    println!("   🔌 WebSocket: ws://localhost:{}/ws/", port);
    println!("   📡 API: http://localhost:{}/api/", port);
    println!();
    println!("Available endpoints:");
    println!("   GET  /api/sigels                    - List all Sigels");
    println!("   GET  /api/sigels/:id                - Get Sigel details");
    println!("   POST /api/sigels/:id/interact       - Interact with Sigel");
    println!("   GET  /api/sigels/:id/consciousness  - Get consciousness state");
    println!("   POST /api/sigels/:id/dream          - Start dream session");
    println!("   GET  /ws/sigel/:id                  - WebSocket connection");
    println!("   GET  /ws/consciousness/:id          - Real-time consciousness monitor");
    println!();

    if let Err(e) = web_interface.start_server(port).await {
        error!("Web server failed: {}", e);
        std::process::exit(1);
    }
}