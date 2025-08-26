use crate::repository::utils;
use std::collections::HashMap;
use std::io;
use hex;

pub fn get_commit_tree_hash(commit_hash: &str) -> io::Result<String> {
    let (obj_type, content) = utils::read_object(commit_hash)?;
    if obj_type != "commit" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Object is not a commit",
        ));
    }

    let content_str = String::from_utf8_lossy(&content);
    for line in content_str.lines() {
        if line.starts_with("tree ") {
            return Ok(line[5..].to_string());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Tree hash not found in commit",
    ))
}

pub fn list_files_in_tree(tree_hash: &str) -> io::Result<HashMap<String, String>> {
    let mut files = HashMap::new();
    list_files_recursive(tree_hash, "", &mut files)?;
    Ok(files)
}

fn list_files_recursive(
    tree_hash: &str,
    current_path: &str,
    files: &mut HashMap<String, String>,
) -> io::Result<()> {
    let (obj_type, content) = utils::read_object(tree_hash)?;
    if obj_type != "tree" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Object is not a tree",
        ));
    }

    let mut cursor = 0;
    while cursor < content.len() {
        let space_pos = content[cursor..]
            .iter()
            .position(|&b| b == b' ')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid tree entry format"))?
            + cursor;

        let null_pos = content[cursor..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid tree entry format"))?
            + cursor;

        let name = String::from_utf8_lossy(&content[space_pos + 1..null_pos]).to_string();
        let hash_bytes = &content[null_pos + 1..null_pos + 21];
        let hash_hex = hex::encode(hash_bytes);

        let path = if current_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", current_path, name)
        };

        // For this project, we assume the mode indicates a blob or a tree.
        // A more robust implementation would parse the mode properly.
        if content[cursor..space_pos].starts_with(b"100") { // It's a blob
            files.insert(path, hash_hex);
        } else { // It's a tree
            list_files_recursive(&hash_hex, &path, files)?;
        }

        cursor = null_pos + 21;
    }

    Ok(())
}