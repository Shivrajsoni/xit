use crate::objects::blob;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
/// if suppose it is previosuly added how can i update the blob hash , ideally it should updaate the blob hash also ????/
/// Handles the `xit add` command.
pub fn add(file_path_str: &str) -> io::Result<()> {
    let git_dir = ".xit";
    let file_path = Path::new(file_path_str);

    // 1. --- Validation ---
    // Ensure we are in a xit repository
    if !Path::new(git_dir).is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Not a xit repository (or any of the parent directories): .git",
        ));
    }

    // Ensure the file to be added exists
    if !file_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("pathspec '{}' did not match any files", file_path_str),
        ));
    }

    // 2. --- Blob Creation ---
    // Read the file's content and create a blob object.
    // Your existing `create_blob` function already handles hashing, compression,
    // and writing the object to the .xit/objects directory.
    let file_content = fs::read(file_path)?;
    let blob_hash = blob::create_blob(&file_content)?;

    // 3. --- Index Update ---
    // Now, we update the index to stage this file for the next commit.
    update_index(file_path_str, &blob_hash)?;

    //    println!("Added '{}' to the staging area.", file_path_str);
    Ok(())
}

/// Updates the .xit/index file with the new file path and its blob hash.
fn update_index(file_path: &str, blob_hash: &str) -> io::Result<()> {
    let git_dir = ".xit";
    let index_path = Path::new(git_dir).join("index");

    // Our index is a simple text file. We can read it into a HashMap
    // for easy lookup and modification.
    let mut index_entries: HashMap<String, String> = HashMap::new();

    // If the index file already exists, read its contents.
    if index_path.exists() {
        let file = fs::File::open(&index_path)?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                // The format is "hash path", so we store it as (path, hash)
                index_entries.insert(parts[1].to_string(), parts[0].to_string());
            }
        }
    }

    // Add or update the entry for the current file.
    // The key is the file path, the value is the blob hash.
    index_entries.insert(file_path.to_string(), blob_hash.to_string());

    // Write the updated entries back to the index file, overwriting it.
    let mut file = fs::File::create(&index_path)?;
    for (path, hash) in &index_entries {
        // We will use a simple format: <hash> <path>

        writeln!(file, "{} {}", hash, path)?;
    }

    Ok(())
}
