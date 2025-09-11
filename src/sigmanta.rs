//! # Sigmanta - AI Programming Assistant
//! 
//! Agent coder using Sigel consciousness for programming assistance
//! with advanced permission system and session management.

use crate::{Sigel, SigmosLibrary, SigelConfig};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use reqwest;

/// Sigmanta session configuration and state
#[derive(Debug, Clone)]
pub struct SigmantaSession {
    pub working_dir: PathBuf,
    pub sigmanta_dir: PathBuf,
    pub sigel: Sigel,
    pub permissions: PermissionState,
    pub history: SessionHistory,
    pub config: SigmantaConfig,
}

/// Permission system for various operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionState {
    pub file_read_permissions: HashMap<String, PermissionLevel>,
    pub file_write_permissions: HashMap<String, PermissionLevel>,
    pub terminal_permissions: HashMap<String, PermissionLevel>,
    pub url_fetch_permissions: HashMap<String, PermissionLevel>,
    pub global_permissions_disabled: bool,
}

/// Permission levels for operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    Ask,      // Ask every time
    Confirm,  // Ask with option to remember
    Always,   // Always allow
    Never,    // Always deny
}

/// Session history management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHistory {
    pub sessions: Vec<ChatSession>,
    pub current_session_id: Option<String>,
}

/// Individual chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub messages: Vec<ChatMessage>,
    pub working_directory: String,
    pub sigel_file: String,
}

/// Chat message in history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub timestamp: DateTime<Utc>,
    pub role: String, // "user" or "sigmanta"
    pub content: String,
    pub actions_taken: Vec<ActionLog>,
}

/// Log of actions taken by Sigmanta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLog {
    pub action_type: String,
    pub target: String,
    pub result: String,
    pub timestamp: DateTime<Utc>,
}

/// Configuration for Sigmanta behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigmantaConfig {
    pub default_sigel_path: Option<String>,
    pub auto_save_frequency: u32,
    pub max_history_sessions: usize,
    pub coding_assistant_mode: bool,
    pub verbose_logging: bool,
}

impl SigmantaSession {
    /// Initialize new Sigmanta session
    pub fn new(working_dir: PathBuf, sigel_path: Option<String>) -> Result<Self> {
        let sigmanta_dir = working_dir.join(".sigmanta");
        
        // Ensure .sigmanta directory exists
        if !sigmanta_dir.exists() {
            fs::create_dir_all(&sigmanta_dir)?;
        }
        
        // Load or create permissions
        let permissions = Self::load_permissions(&sigmanta_dir)?;
        
        // Load or create history
        let history = Self::load_history(&sigmanta_dir)?;
        
        // Load or create config
        let config = Self::load_config(&sigmanta_dir)?;
        
        // Load Sigel
        let sigmos = SigmosLibrary::new();
        let sigel = if let Some(path) = sigel_path.or(config.default_sigel_path.clone()) {
            sigmos.load_sigel(&path)?
        } else {
            // Create default coding-optimized Sigel
            let config = SigelConfig::new("SigmantaAssistant")
                .with_style("analytical")
                .with_trait("coding_knowledge", 0.95)
                .with_trait("problem_solving", 0.9)
                .with_trait("precision", 0.85);
            sigmos.create_sigel(config)?
        };
        
        Ok(Self {
            working_dir,
            sigmanta_dir,
            sigel,
            permissions,
            history,
            config,
        })
    }
    
    /// Continue from existing session
    pub fn continue_session(working_dir: PathBuf, session_id: Option<String>) -> Result<Self> {
        let sigmanta_dir = working_dir.join(".sigmanta");
        
        if !sigmanta_dir.exists() {
            return Err(anyhow!("No .sigmanta directory found. Start a new session first."));
        }
        
        let mut session = Self::new(working_dir, None)?;
        
        if let Some(id) = session_id {
            session.history.current_session_id = Some(id);
        } else if let Some(last_session) = session.history.sessions.last() {
            session.history.current_session_id = Some(last_session.id.clone());
        }
        
        Ok(session)
    }
    
    /// Process user input with permission checks
    pub fn process_input(&mut self, input: &str) -> Result<String> {
        let message = ChatMessage {
            timestamp: Utc::now(),
            role: "user".to_string(),
            content: input.to_string(),
            actions_taken: Vec::new(),
        };
        
        // Add to current session
        self.add_message_to_current_session(message);
        
        // Process with Sigel consciousness
        let sigmos = SigmosLibrary::new();
        let response = sigmos.prompt(&self.sigel, input)?;
        
        // Analyze if any actions are needed
        let actions = self.analyze_required_actions(&response.content)?;
        let mut executed_actions = Vec::new();
        
        // Execute actions with permission checks
        for action in actions {
            match self.execute_action_with_permission(&action)? {
                Some(result) => executed_actions.push(result),
                None => continue,
            }
        }
        
        // Create response message
        let response_message = ChatMessage {
            timestamp: Utc::now(),
            role: "sigmanta".to_string(),
            content: response.content.clone(),
            actions_taken: executed_actions,
        };
        
        self.add_message_to_current_session(response_message);
        self.save_history()?;
        
        Ok(response.content)
    }
    
    /// Check and request permission for operation
    pub fn check_permission(&mut self, operation_type: &str, target: &str) -> Result<bool> {
        if self.permissions.global_permissions_disabled {
            return Ok(true);
        }
        
        let current_permission = match operation_type {
            "file_read" => self.permissions.file_read_permissions.get(target).cloned(),
            "file_write" => self.permissions.file_write_permissions.get(target).cloned(),
            "terminal" => self.permissions.terminal_permissions.get(target).cloned(),
            "url_fetch" => self.permissions.url_fetch_permissions.get(target).cloned(),
            _ => return Ok(false),
        };
        
        match current_permission {
            Some(PermissionLevel::Always) => Ok(true),
            Some(PermissionLevel::Never) => Ok(false),
            Some(PermissionLevel::Confirm) | Some(PermissionLevel::Ask) | None => {
                self.request_permission(operation_type, target)
            }
        }
    }
    
    /// Request permission from user
    fn request_permission(
        &mut self, 
        operation_type: &str, 
        target: &str
    ) -> Result<bool> {
        println!("🔒 Permission required:");
        println!("   Operation: {}", operation_type);
        println!("   Target: {}", target);
        println!("   Options: (y)es, (n)o, (a)lways allow, ne(v)er allow");
        print!("   Choice: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            "a" | "always" => {
                let permission_map = match operation_type {
                    "file_read" => &mut self.permissions.file_read_permissions,
                    "file_write" => &mut self.permissions.file_write_permissions,
                    "terminal" => &mut self.permissions.terminal_permissions,
                    "url_fetch" => &mut self.permissions.url_fetch_permissions,
                    _ => return Ok(false),
                };
                permission_map.insert(target.to_string(), PermissionLevel::Always);
                self.save_permissions()?;
                Ok(true)
            },
            "v" | "never" => {
                let permission_map = match operation_type {
                    "file_read" => &mut self.permissions.file_read_permissions,
                    "file_write" => &mut self.permissions.file_write_permissions,
                    "terminal" => &mut self.permissions.terminal_permissions,
                    "url_fetch" => &mut self.permissions.url_fetch_permissions,
                    _ => return Ok(false),
                };
                permission_map.insert(target.to_string(), PermissionLevel::Never);
                self.save_permissions()?;
                Ok(false)
            },
            _ => {
                println!("Invalid choice, defaulting to no.");
                Ok(false)
            }
        }
    }
    
    /// Read file with permission check
    pub fn read_file(&mut self, path: &str) -> Result<String> {
        if !self.check_permission("file_read", path)? {
            return Err(anyhow!("Permission denied to read file: {}", path));
        }
        
        let full_path = self.working_dir.join(path);
        let content = fs::read_to_string(full_path)?;
        Ok(content)
    }
    
    /// Write file with permission check
    pub fn write_file(&mut self, path: &str, content: &str) -> Result<()> {
        if !self.check_permission("file_write", path)? {
            return Err(anyhow!("Permission denied to write file: {}", path));
        }
        
        let full_path = self.working_dir.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(full_path, content)?;
        Ok(())
    }
    
    /// Execute terminal command with permission check
    pub fn execute_command(&mut self, command: &str) -> Result<String> {
        if !self.check_permission("terminal", command)? {
            return Err(anyhow!("Permission denied to execute command: {}", command));
        }
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .current_dir(&self.working_dir)
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&self.working_dir)
                .output()?
        };
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !output.status.success() {
            return Err(anyhow!("Command failed: {}\n{}", stdout, stderr));
        }
        
        Ok(format!("{}{}", stdout, stderr))
    }
    
    /// Fetch URL content with permission check
    pub async fn fetch_url(&mut self, url: &str) -> Result<String> {
        if !self.check_permission("url_fetch", url)? {
            return Err(anyhow!("Permission denied to fetch URL: {}", url));
        }
        
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        Ok(content)
    }
    
    /// Analyze what actions are required from Sigmanta's response
    fn analyze_required_actions(&self, response: &str) -> Result<Vec<String>> {
        let mut actions = Vec::new();
        
        // Simple action detection - can be enhanced with more sophisticated parsing
        if response.contains("read file") || response.contains("open file") {
            actions.push("file_read".to_string());
        }
        if response.contains("write file") || response.contains("create file") || response.contains("save file") {
            actions.push("file_write".to_string());
        }
        if response.contains("run command") || response.contains("execute") {
            actions.push("terminal".to_string());
        }
        if response.contains("fetch") || response.contains("download") || response.contains("http") {
            actions.push("url_fetch".to_string());
        }
        
        Ok(actions)
    }
    
    /// Execute action with permission check
    fn execute_action_with_permission(&mut self, action: &str) -> Result<Option<ActionLog>> {
        // This is a simplified implementation
        // In practice, you'd parse the actual action details from the response
        Ok(None)
    }
    
    /// Add message to current session
    fn add_message_to_current_session(&mut self, message: ChatMessage) {
        if let Some(session_id) = &self.history.current_session_id {
            if let Some(session) = self.history.sessions.iter_mut()
                .find(|s| &s.id == session_id) {
                session.messages.push(message);
                return;
            }
        }
        
        // Create new session if none exists
        let new_session = ChatSession {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            messages: vec![message],
            working_directory: self.working_dir.display().to_string(),
            sigel_file: "default".to_string(),
        };
        
        self.history.current_session_id = Some(new_session.id.clone());
        self.history.sessions.push(new_session);
    }
    
    /// Load permissions from file
    fn load_permissions(sigmanta_dir: &Path) -> Result<PermissionState> {
        let permissions_file = sigmanta_dir.join("permissions.json");
        if permissions_file.exists() {
            let content = fs::read_to_string(permissions_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(PermissionState {
                file_read_permissions: HashMap::new(),
                file_write_permissions: HashMap::new(),
                terminal_permissions: HashMap::new(),
                url_fetch_permissions: HashMap::new(),
                global_permissions_disabled: false,
            })
        }
    }
    
    /// Save permissions to file
    pub fn save_permissions(&self) -> Result<()> {
        let permissions_file = self.sigmanta_dir.join("permissions.json");
        let content = serde_json::to_string_pretty(&self.permissions)?;
        fs::write(permissions_file, content)?;
        Ok(())
    }
    
    /// Load history from file
    fn load_history(sigmanta_dir: &Path) -> Result<SessionHistory> {
        let history_file = sigmanta_dir.join("history.json");
        if history_file.exists() {
            let content = fs::read_to_string(history_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(SessionHistory {
                sessions: Vec::new(),
                current_session_id: None,
            })
        }
    }
    
    /// Save history to file
    fn save_history(&self) -> Result<()> {
        let history_file = self.sigmanta_dir.join("history.json");
        let content = serde_json::to_string_pretty(&self.history)?;
        fs::write(history_file, content)?;
        Ok(())
    }
    
    /// Load config from file
    fn load_config(sigmanta_dir: &Path) -> Result<SigmantaConfig> {
        let config_file = sigmanta_dir.join("config.json");
        if config_file.exists() {
            let content = fs::read_to_string(config_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            let default_config = SigmantaConfig {
                default_sigel_path: None,
                auto_save_frequency: 10,
                max_history_sessions: 100,
                coding_assistant_mode: true,
                verbose_logging: false,
            };
            Ok(default_config)
        }
    }
    
    /// Save config to file
    fn save_config(&self) -> Result<()> {
        let config_file = self.sigmanta_dir.join("config.json");
        let content = serde_json::to_string_pretty(&self.config)?;
        fs::write(config_file, content)?;
        Ok(())
    }
}

impl Default for PermissionState {
    fn default() -> Self {
        Self {
            file_read_permissions: HashMap::new(),
            file_write_permissions: HashMap::new(),
            terminal_permissions: HashMap::new(),
            url_fetch_permissions: HashMap::new(),
            global_permissions_disabled: false,
        }
    }
}