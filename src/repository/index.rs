use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads the .xit/index file and returns a map of file paths to their blob hashes.
pub fn read_index(path: &Path) -> io::Result<HashMap<String, String>> {
    let mut entries = HashMap::new();
    let file = fs::File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            // The format is <hash> <path>
            entries.insert(parts[1].to_string(), parts[0].to_string());
        }
    }
    Ok(entries)
}