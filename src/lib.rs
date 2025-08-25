pub mod sigel;
pub mod consciousness;
pub mod learning;
pub mod cosmos;
pub mod interaction;
pub mod server;
pub mod mathtables_integration;
pub mod gpu_acceleration;
pub mod memory_consolidation;
pub mod advanced_learning;
pub mod enhanced_consciousness;
pub mod dream_mode;
pub mod web_interface;
pub mod collective_intelligence;
pub mod quantum_consciousness;

pub use sigel::*;
pub use consciousness::*;
pub use learning::*;
pub use cosmos::*;
pub use interaction::*;
pub use server::*;
pub use mathtables_integration::*;
pub use gpu_acceleration::*;
pub use memory_consolidation::*;
pub use advanced_learning::*;
pub use enhanced_consciousness::*;
pub use dream_mode::*;
pub use web_interface::*;
pub use collective_intelligence::*;
pub use quantum_consciousness::*;

use anyhow::Result;
use std::path::Path;

pub const SIGEL_EXTENSION: &str = "sig";
pub const MASTER_SIGEL_NAME: &str = "master.sigel";

pub fn load_sigel_from_file<P: AsRef<Path>>(path: P) -> Result<Sigel> {
    let content = std::fs::read_to_string(path)?;
    let sigel = serde_json::from_str(&content)?;
    Ok(sigel)
}

pub fn save_sigel_to_file<P: AsRef<Path>>(sigel: &Sigel, path: P) -> Result<()> {
    let content = serde_json::to_string_pretty(sigel)?;
    std::fs::write(path, content)?;
    Ok(())
}