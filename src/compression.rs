use flate2::read::{GzDecoder};
use flate2::write::{GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;
use crate::sigel::Sigel;

/// Compressed Sigel file operations
pub struct CompressedSigel;

impl CompressedSigel {
    /// Save a Sigel to a compressed file
    pub fn save_compressed<P: AsRef<Path>>(sigel: &Sigel, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(&path)?;
        let buf_writer = BufWriter::new(file);
        let mut encoder = GzEncoder::new(buf_writer, Compression::default());
        
        // Serialize to JSON first
        let json_data = serde_json::to_string_pretty(sigel)?;
        encoder.write_all(json_data.as_bytes())?;
        encoder.finish()?;
        
        let compressed_size = std::fs::metadata(&path)?.len();
        let original_size = json_data.len() as u64;
        let compression_ratio = (compressed_size as f64 / original_size as f64) * 100.0;
        
        println!("💾 Compressed save: {:.1}% of original size ({} -> {} bytes)", 
                 compression_ratio, original_size, compressed_size);
        
        Ok(())
    }
    
    /// Load a Sigel from a compressed file
    pub fn load_compressed<P: AsRef<Path>>(path: P) -> Result<Sigel, Box<dyn std::error::Error>> {
        let file = File::open(&path)?;
        let buf_reader = BufReader::new(file);
        let mut decoder = GzDecoder::new(buf_reader);
        let mut json_data = String::new();
        decoder.read_to_string(&mut json_data)?;
        
        let sigel: Sigel = serde_json::from_str(&json_data)?;
        
        let compressed_size = std::fs::metadata(&path)?.len();
        let decompressed_size = json_data.len() as u64;
        
        println!("📂 Loaded compressed Sigel: {} bytes compressed, {} bytes decompressed", 
                 compressed_size, decompressed_size);
        
        Ok(sigel)
    }
    
    /// Check if a file is compressed based on extension
    pub fn is_compressed<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.ends_with("gz") || ext.ends_with("sig.gz"))
            .unwrap_or(false)
    }
    
    /// Get appropriate file extension
    pub fn get_extension(compress: bool) -> &'static str {
        if compress {
            "sig.gz"
        } else {
            "sig"
        }
    }
    
    /// Compress an existing uncompressed Sigel file
    pub fn compress_existing<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗜️ Compressing existing Sigel file...");
        
        // Load from uncompressed file
        let sigel = load_sigel_smart(&input_path)?;
        
        // Save as compressed
        Self::save_compressed(&sigel, &output_path)?;
        
        let original_size = std::fs::metadata(&input_path)?.len();
        let compressed_size = std::fs::metadata(&output_path)?.len();
        let space_saved = original_size - compressed_size;
        
        println!("✅ Compression completed: Saved {} bytes ({:.1}% reduction)", 
                 space_saved, 
                 (space_saved as f64 / original_size as f64) * 100.0);
        
        Ok(())
    }
}

/// Smart Sigel loader that automatically detects compression
pub fn load_sigel_smart<P: AsRef<Path>>(path: P) -> Result<Sigel, Box<dyn std::error::Error>> {
    if CompressedSigel::is_compressed(&path) {
        CompressedSigel::load_compressed(path)
    } else {
        // Use JSON deserialization directly since we don't have the crate function yet
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let sigel: Sigel = serde_json::from_reader(reader)?;
        Ok(sigel)
    }
}

/// Smart Sigel saver with optional compression
pub fn save_sigel_smart<P: AsRef<Path>>(sigel: &Sigel, path: P, compress: bool) -> Result<(), Box<dyn std::error::Error>> {
    if compress {
        CompressedSigel::save_compressed(sigel, path)
    } else {
        // Use JSON serialization directly since we don't have the crate function yet
        let file = File::create(&path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, sigel)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tempfile::NamedTempFile; // Comment out until we add tempfile dependency
    
    #[test]
    fn test_compression_detection() {
        assert_eq!(CompressedSigel::is_compressed("test.sig.gz"), true);
        assert_eq!(CompressedSigel::is_compressed("test.sig"), false);
        assert_eq!(CompressedSigel::get_extension(true), "sig.gz");
        assert_eq!(CompressedSigel::get_extension(false), "sig");
    }
}