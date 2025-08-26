use crate::objects::{commit as commit_object, tree, update};
use crate::repository::{config, index, refs, utils};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Result};
use std::path::Path;

/// Orchestrates the entire commit process.
pub fn commit(message: &str) -> Result<()> {
    // 1. --- Build Tree from Index ---
    let index_path = Path::new(".xit").join("index");
    if !index_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Nothing to commit. The index is empty.",
        ));
    }

    let index_entries = index::read_index(&index_path)?;
    if index_entries.is_empty() {
        println!("Nothing to commit, index is empty.");
        return Ok(());
    }

    let tree_hash = create_tree_from_index(index_entries)?;

    // 2. --- Find Parent Commit ---
    let head_ref_path = refs::get_head_ref_path()?;
    let parent_hash = refs::get_commit_hash(&head_ref_path).ok();

    // 3. --- Get Author and Committer Info ---
    let user_config = config::get_user_config()?;
    let author = format!("{} <{}>", user_config.name, user_config.email);
    // For this project, the author and committer are the same.
    let committer = &author;

    // 4. --- Create the Commit Object ---
    let new_commit_hash = commit_object::create_commit(
        &tree_hash,
        parent_hash.as_deref(),
        &author,
        committer,
        message,
    )?;

    // 5. --- Update the Branch Reference (HEAD) ---
    update::update_reference(&head_ref_path, &new_commit_hash)?;

    // 6. --- Clear the Index ---
    fs::remove_file(&index_path)?;

    //    println!("Committed to [{}]: {}", &new_commit_hash[..7], message);
    Ok(())
}

/// Builds a tree object from the current index and returns its hash.
fn create_tree_from_index(index: HashMap<String, String>) -> Result<String> {
    let mut tree_entries: Vec<tree::TreeEntry> = Vec::new();
    for (path, hash_hex) in index {
        let hash_bytes = utils::hex_to_bytes(&hash_hex)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid hash in index"))?;
        tree_entries.push(tree::TreeEntry {
            mode: "100644".to_string(), // Assuming normal file mode for simplicity
            obj_type: "blob".to_string(),
            hash: hash_bytes,
            name: path,
        });
    }
    // Call the low-level tree creation function from the objects module.
    tree::create_tree(tree_entries)
}
