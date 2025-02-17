use anyhow::Result;
use std::fs;

pub fn get_file_content(filepath: &str) -> Result<String> {
    let content = fs::read_to_string(filepath)?;
    Ok(content)
}
