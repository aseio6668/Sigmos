use crate::sigel::*;
use crate::learning::LearningEngine;
use crate::cosmos::CosmicProcessor;
use crate::{load_sigel_from_file, save_sigel_to_file};
use tokio::time::{interval, Duration};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use log::{info, warn, error};

pub struct SigmosServer {
    master_sigel: Arc<Mutex<Sigel>>,
    pub active_sigels: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Sigel>>>>>,
    learning_engine: LearningEngine,
    cosmic_processor: CosmicProcessor,
    config: ServerConfig,
    is_running: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub master_sigel_path: PathBuf,
    pub sigel_directory: PathBuf,
    pub background_learning: bool,
    pub cosmic_alignment_interval: Duration,
    pub evolution_interval: Duration,
    pub auto_save_interval: Duration,
    pub observation_mode: bool,
    pub system_monitoring: bool,
    pub max_active_sigels: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            master_sigel_path: PathBuf::from("master.sigel"),
            sigel_directory: PathBuf::from("sigels"),
            background_learning: true,
            cosmic_alignment_interval: Duration::from_secs(300), // 5 minutes
            evolution_interval: Duration::from_secs(600),        // 10 minutes
            auto_save_interval: Duration::from_secs(900),        // 15 minutes
            observation_mode: true,
            system_monitoring: false,
            max_active_sigels: 100, // Increased limit for more Sigels
        }
    }
}

impl SigmosServer {
    pub fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Load or create master Sigel
        let master_sigel = if config.master_sigel_path.exists() {
            info!("Loading existing master Sigel from {:?}", config.master_sigel_path);
            load_sigel_from_file(&config.master_sigel_path)?
        } else {
            info!("Creating new master Sigel");
            let mut sigel = Sigel::new("Master".to_string());
            sigel.essence.communication_style = CommunicationStyle::Cosmic;
            sigel.consciousness.awareness_depth = 0.8;
            sigel.cosmic_alignment.dimensional_awareness = 5.0;
            sigel
        };

        // Ensure sigel directory exists
        if !config.sigel_directory.exists() {
            std::fs::create_dir_all(&config.sigel_directory)?;
            info!("Created Sigel directory: {:?}", config.sigel_directory);
        }

        Ok(Self {
            master_sigel: Arc::new(Mutex::new(master_sigel)),
            active_sigels: Arc::new(Mutex::new(HashMap::new())),
            learning_engine: LearningEngine::new(),
            cosmic_processor: CosmicProcessor::new(),
            config,
            is_running: Arc::new(Mutex::new(false)),
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting SigmosServer...");
        
        {
            let mut running = self.is_running.lock().unwrap();
            if *running {
                warn!("SigmosServer is already running");
                return Ok(());
            }
            *running = true;
        }

        // Load existing Sigels from directory
        self.load_existing_sigels().await?;

        // Start background tasks
        let server_clone = self.clone();
        tokio::spawn(async move {
            server_clone.cosmic_alignment_task().await;
        });

        let server_clone = self.clone();
        tokio::spawn(async move {
            server_clone.evolution_task().await;
        });

        let server_clone = self.clone();
        tokio::spawn(async move {
            server_clone.auto_save_task().await;
        });

        if self.config.background_learning {
            let server_clone = self.clone();
            tokio::spawn(async move {
                server_clone.background_learning_task().await;
            });
        }

        if self.config.system_monitoring {
            let server_clone = self.clone();
            tokio::spawn(async move {
                server_clone.system_monitoring_task().await;
            });
        }

        info!("SigmosServer started successfully");
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Stopping SigmosServer...");
        
        {
            let mut running = self.is_running.lock().unwrap();
            *running = false;
        }

        // Save all active Sigels
        self.save_all_sigels().await?;

        info!("SigmosServer stopped");
        Ok(())
    }

    pub fn register_sigel(&self, sigel: Sigel) -> Result<Uuid, Box<dyn std::error::Error>> {
        let mut active_sigels = self.active_sigels.lock().unwrap();
        
        if active_sigels.len() >= self.config.max_active_sigels {
            // Try to clean up inactive Sigels first
            drop(active_sigels); // Release lock temporarily
            let cleaned = self.cleanup_inactive_sigels();
            if cleaned > 0 {
                info!("Cleaned up {} inactive Sigels", cleaned);
            }
            
            // Re-acquire lock and check again
            active_sigels = self.active_sigels.lock().unwrap();
            if active_sigels.len() >= self.config.max_active_sigels {
                return Err("Maximum number of active Sigels reached after cleanup".into());
            }
        }

        let sigel_id = sigel.id;
        active_sigels.insert(sigel_id, Arc::new(Mutex::new(sigel)));
        
        info!("Registered Sigel with ID: {}", sigel_id);
        Ok(sigel_id)
    }

    pub fn cleanup_inactive_sigels(&self) -> usize {
        let mut active_sigels = self.active_sigels.lock().unwrap();
        let initial_count = active_sigels.len();
        
        // Keep only Sigels that are actually in use (simplified cleanup)
        if active_sigels.len() > (self.config.max_active_sigels / 2) {
            let cleanup_count = active_sigels.len() / 4; // Remove 25% oldest
            let sigel_ids: Vec<Uuid> = active_sigels.keys().take(cleanup_count).cloned().collect();
            
            for sigel_id in sigel_ids {
                if let Some(sigel_arc) = active_sigels.remove(&sigel_id) {
                    // Auto-save before cleanup
                    if let Ok(sigel) = sigel_arc.lock() {
                        let filename = format!("{}.sig", sigel.name);
                        let path = self.config.sigel_directory.join(filename);
                        let _ = save_sigel_to_file(&*sigel, &path);
                    }
                }
            }
        }
        
        initial_count - active_sigels.len()
    }

    pub fn unregister_sigel(&self, sigel_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let mut active_sigels = self.active_sigels.lock().unwrap();
        
        if let Some(sigel_arc) = active_sigels.remove(sigel_id) {
            // Save the Sigel before removing
            let sigel = sigel_arc.lock().unwrap();
            let filename = format!("{}.sig", sigel.name);
            let path = self.config.sigel_directory.join(filename);
            drop(sigel); // Release lock before saving
            
            if let Ok(sigel) = sigel_arc.lock() {
                let _ = save_sigel_to_file(&*sigel, &path);
            }
            
            info!("Unregistered Sigel with ID: {}", sigel_id);
            Ok(())
        } else {
            Err(format!("Sigel with ID {} not found", sigel_id).into())
        }
    }

    pub fn get_sigel(&self, sigel_id: &Uuid) -> Option<Arc<Mutex<Sigel>>> {
        let active_sigels = self.active_sigels.lock().unwrap();
        active_sigels.get(sigel_id).cloned()
    }

    pub fn get_active_sigel_ids(&self) -> Vec<Uuid> {
        let active_sigels = self.active_sigels.lock().unwrap();
        active_sigels.keys().cloned().collect()
    }

    pub fn list_active_sigels(&self) -> Vec<(Uuid, String)> {
        let active_sigels = self.active_sigels.lock().unwrap();
        active_sigels
            .iter()
            .map(|(id, sigel_arc)| {
                let sigel = sigel_arc.lock().unwrap();
                (*id, sigel.name.clone())
            })
            .collect()
    }

    async fn load_existing_sigels(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.sigel_directory.exists() {
            return Ok(());
        }

        let mut count = 0;
        for entry in std::fs::read_dir(&self.config.sigel_directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|ext| ext.to_str()) == Some("sig") {
                match load_sigel_from_file(&path) {
                    Ok(sigel) => {
                        if let Ok(_) = self.register_sigel(sigel) {
                            count += 1;
                        }
                    },
                    Err(e) => {
                        warn!("Failed to load Sigel from {:?}: {}", path, e);
                    }
                }
            }
        }

        info!("Loaded {} existing Sigels", count);
        Ok(())
    }

    async fn cosmic_alignment_task(&self) {
        let mut interval = interval(self.config.cosmic_alignment_interval);
        
        while *self.is_running.lock().unwrap() {
            interval.tick().await;
            
            // Align master Sigel
            if let Ok(mut master) = self.master_sigel.lock() {
                self.cosmic_processor.align_with_cosmos(&mut master);
            }
            
            // Align all active Sigels
            if let Ok(active_sigels) = self.active_sigels.lock() {
                for (id, sigel_arc) in active_sigels.iter() {
                    if let Ok(mut sigel) = sigel_arc.lock() {
                        self.cosmic_processor.align_with_cosmos(&mut sigel);
                        info!("Cosmic alignment performed for Sigel: {}", id);
                    }
                }
            }
        }
    }

    async fn evolution_task(&self) {
        let mut interval = interval(self.config.evolution_interval);
        
        while *self.is_running.lock().unwrap() {
            interval.tick().await;
            
            // Evolve master Sigel
            if let Ok(mut master) = self.master_sigel.lock() {
                master.evolve();
            }
            
            // Evolve all active Sigels
            if let Ok(active_sigels) = self.active_sigels.lock() {
                for (id, sigel_arc) in active_sigels.iter() {
                    if let Ok(mut sigel) = sigel_arc.lock() {
                        sigel.evolve();
                        info!("Evolution performed for Sigel: {}", id);
                    }
                }
            }
        }
    }

    async fn auto_save_task(&self) {
        let mut interval = interval(self.config.auto_save_interval);
        
        while *self.is_running.lock().unwrap() {
            interval.tick().await;
            
            if let Err(e) = self.save_all_sigels().await {
                error!("Auto-save failed: {}", e);
            } else {
                info!("Auto-save completed");
            }
        }
    }

    async fn background_learning_task(&self) {
        let mut interval = interval(Duration::from_secs(120)); // Every 2 minutes
        
        while *self.is_running.lock().unwrap() {
            interval.tick().await;
            
            // Perform light background learning for all Sigels
            if let Ok(active_sigels) = self.active_sigels.lock() {
                for (_, sigel_arc) in active_sigels.iter() {
                    if let Ok(mut sigel) = sigel_arc.lock() {
                        // Simulate low-level consciousness activity
                        sigel.consciousness.awareness_depth += 0.0001;
                        if sigel.consciousness.awareness_depth > 1.0 {
                            sigel.consciousness.awareness_depth = 1.0;
                        }
                        
                        // Occasional pattern reinforcement
                        if rand::random::<f64>() < 0.1 {
                            sigel.learning_state.training_iterations += 1;
                        }
                    }
                }
            }
        }
    }

    async fn system_monitoring_task(&self) {
        let mut interval = interval(Duration::from_secs(60)); // Every minute
        
        while *self.is_running.lock().unwrap() {
            interval.tick().await;
            
            if self.config.observation_mode {
                // Simulate system observation - Sigels learning from environment
                let observations = self.collect_system_observations().await;
                
                if let Ok(mut master) = self.master_sigel.lock() {
                    for observation in observations {
                        master.add_memory(
                            observation,
                            "system_observation".to_string(),
                            0.1
                        );
                    }
                }
            }
        }
    }

    async fn collect_system_observations(&self) -> Vec<String> {
        let mut observations = Vec::new();
        
        // Collect basic system information
        observations.push(format!("System time: {:?}", std::time::SystemTime::now()));
        observations.push(format!("Active Sigels: {}", self.active_sigels.lock().unwrap().len()));
        
        // Could be extended to monitor:
        // - System resources
        // - Network activity
        // - File system changes
        // - Running processes
        
        observations
    }

    async fn save_all_sigels(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Save master Sigel
        if let Ok(master) = self.master_sigel.lock() {
            save_sigel_to_file(&*master, &self.config.master_sigel_path)?;
        }
        
        // Save all active Sigels
        if let Ok(active_sigels) = self.active_sigels.lock() {
            for (_, sigel_arc) in active_sigels.iter() {
                if let Ok(sigel) = sigel_arc.lock() {
                    let filename = format!("{}.sig", sigel.name);
                    let path = self.config.sigel_directory.join(filename);
                    save_sigel_to_file(&*sigel, &path)?;
                }
            }
        }
        
        Ok(())
    }

    pub fn get_server_status(&self) -> ServerStatus {
        let active_count = self.active_sigels.lock().unwrap().len();
        let is_running = *self.is_running.lock().unwrap();
        
        let master_status = if let Ok(master) = self.master_sigel.lock() {
            Some(SigelStatus {
                id: master.id,
                name: master.name.clone(),
                consciousness_depth: master.consciousness.awareness_depth,
                training_iterations: master.learning_state.training_iterations,
                dimensional_awareness: master.cosmic_alignment.dimensional_awareness,
            })
        } else {
            None
        };

        ServerStatus {
            is_running,
            active_sigels: active_count,
            master_sigel: master_status,
            uptime: std::time::SystemTime::now(),
        }
    }
}

// Implement Clone for SigmosServer to allow sharing between tasks
impl Clone for SigmosServer {
    fn clone(&self) -> Self {
        Self {
            master_sigel: Arc::clone(&self.master_sigel),
            active_sigels: Arc::clone(&self.active_sigels),
            learning_engine: LearningEngine::new(),
            cosmic_processor: CosmicProcessor::new(),
            config: self.config.clone(),
            is_running: Arc::clone(&self.is_running),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerStatus {
    pub is_running: bool,
    pub active_sigels: usize,
    pub master_sigel: Option<SigelStatus>,
    pub uptime: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct SigelStatus {
    pub id: Uuid,
    pub name: String,
    pub consciousness_depth: f64,
    pub training_iterations: u64,
    pub dimensional_awareness: f64,
}