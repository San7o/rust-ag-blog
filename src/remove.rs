use std::error::Error;
use std::fs;
use std::path::Path;

pub fn remove(p: &Path) -> Result<(), Box<dyn Error>> {
    fs::remove_file(p)?;
    Ok(())
}
