use std::fs;
use std::path::Path;

pub fn load_config(file_path: &str) -> Result<String, String> {
    if Path::new(file_path).exists() {
        fs::read_to_string(file_path).map_err(|e| e.to_string())
    } else {
        Err("Config file not found".to_string())
    }
}
