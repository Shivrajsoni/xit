use std::io::Result;

use crate::objects::blob::{compress_zlib, compute_sha1, hash_to_hex};

#[derive(Debug)]
pub struct TreeEntry {
    pub mode: String,
    pub obj_type: String, // tree or blob
    pub hash: Vec<u8>,
    pub name: String,
}

pub fn create_tree(entries: Vec<TreeEntry>) -> Result<String> {
    // Sort entries by name (Git requirement)
    let mut sorted_entries = entries;
    sorted_entries.sort_by(|a, b| a.name.cmp(&b.name));

    let mut data = Vec::new();

    for entry in sorted_entries {
        // Validate entry data
        if entry.hash.len() != 20 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid hash length for entry '{}': expected 20 bytes, got {}",
                    entry.name,
                    entry.hash.len()
                ),
            ));
        }

        if entry.name.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Tree entry name cannot be empty",
            ));
        }

        // Build tree entry format: mode name\0hash
        data.extend_from_slice(entry.mode.as_bytes());
        data.push(b' ');
        data.extend_from_slice(entry.name.as_bytes());
        data.push(b'\0');
        data.extend_from_slice(&entry.hash);
    }

    // Create tree header: "tree {size}\0"
    let header = format!("tree {}\0", data.len());
    let full_data = [header.as_bytes(), &data].concat();

    // Compute hash and compress
    let hash = compute_sha1(&full_data);
    let compressed_data = compress_zlib(&full_data)?;
    let hash_str = hash_to_hex(&hash);

    // Create directory structure and write file
    let dir_path = format!(".xit/objects/{}", &hash_str[0..2]);
    std::fs::create_dir_all(&dir_path)?;

    let path = format!("{}/{}", dir_path, &hash_str[2..]);
    std::fs::write(path, compressed_data)?;

    Ok(hash_str)
}

// Helper function to create a blob entry
pub fn create_blob_entry(mode: &str, hash: &[u8; 20], name: &str) -> TreeEntry {
    TreeEntry {
        mode: mode.to_string(),
        obj_type: "blob".to_string(),
        hash: hash.to_vec(),
        name: name.to_string(),
    }
}

// Helper function to create a tree entry
pub fn create_tree_entry(mode: &str, hash: &[u8; 20], name: &str) -> TreeEntry {
    TreeEntry {
        mode: mode.to_string(),
        obj_type: "tree".to_string(),
        hash: hash.to_vec(),
        name: name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_tree() {
        let blob_hash = compute_sha1(b"hello world");
        let entries = vec![create_blob_entry("100644", &blob_hash, "hello.txt")];

        let hash = create_tree(entries).unwrap();

        // Clean up created files
        let dir_path = format!(".xit/objects/{}", &hash[0..2]);
        fs::remove_dir_all(dir_path).unwrap();
    }
}
