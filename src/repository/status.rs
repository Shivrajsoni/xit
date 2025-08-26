use crate::objects::blob;
use crate::objects::read as object_read;
use crate::repository::{index, refs};
use std::fs;
use std::{
    collections::{HashMap, HashSet},
    io,
    path::{Path, PathBuf},
};
use term_colr::{green, red};

/// Represents the result of a status check, categorized into staged, unstaged, and untracked.
struct StatusResult {
    staged: HashMap<String, String>,
    unstaged: HashMap<String, String>,
    untracked: HashSet<String>,
}

/// Main function to check the status of the repository.
/// It compares HEAD, the index, and the working directory, then prints the status.
pub fn check_status() -> io::Result<()> {
    let index_entries = get_index_entries()?;
    let head_tree_entries = get_head_tree_entries()?;
    let ignore_patterns = read_ignore_file(".xitignore")?;

    let (unstaged_changes, untracked_files) =
        get_unstaged_and_untracked(&index_entries, &ignore_patterns)?;

    let mut status_result = StatusResult {
        staged: get_staged_changes(&index_entries, &head_tree_entries),
        unstaged: unstaged_changes,
        untracked: untracked_files,
    };

    // Exclude files that are staged for addition from the untracked list.
    for path in status_result.staged.keys() {
        if status_result.untracked.contains(path) {
            status_result.untracked.remove(path);
        }
    }

    print_status(&status_result);
    Ok(())
}

/// Reads the index file and returns its entries.
fn get_index_entries() -> io::Result<HashMap<String, String>> {
    let index_path = Path::new(".xit").join("index");
    if index_path.exists() {
        index::read_index(&index_path)
    } else {
        Ok(HashMap::new())
    }
}

/// Reads the HEAD commit's tree and returns its file entries.
fn get_head_tree_entries() -> io::Result<HashMap<String, String>> {
    if let Ok(head_ref_path) = refs::get_head_ref_path() {
        if let Ok(head_commit_hash) = refs::get_commit_hash(&head_ref_path) {
            if let Ok(tree_hash) = object_read::get_commit_tree_hash(&head_commit_hash) {
                return object_read::list_files_in_tree(&tree_hash);
            }
        }
    }
    // No HEAD commit exists yet or other error
    Ok(HashMap::new())
}

/// Compares HEAD and the index to find staged changes.
fn get_staged_changes(
    index_entries: &HashMap<String, String>,
    head_tree_entries: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut staged_changes = HashMap::new();

    // Check for new files and modifications
    for (path, index_hash) in index_entries {
        match head_tree_entries.get(path) {
            Some(head_hash) if head_hash != index_hash => {
                staged_changes.insert(path.clone(), "modified".to_string());
            }
            None => {
                staged_changes.insert(path.clone(), "new file".to_string());
            }
            _ => {} // Unchanged
        }
    }

    // Check for deletions
    for (path, _head_hash) in head_tree_entries {
        if !index_entries.contains_key(path) {
            staged_changes.insert(path.clone(), "deleted".to_string());
        }
    }

    staged_changes
}

/// Compares the index and working directory for unstaged changes and untracked files.
fn get_unstaged_and_untracked(
    index_entries: &HashMap<String, String>,
    ignore_patterns: &HashSet<String>,
) -> io::Result<(HashMap<String, String>, HashSet<String>)> {
    let mut unstaged_changes = HashMap::new();
    let mut untracked_files = HashSet::new();
    let mut working_dir_files = HashSet::new();

    for entry in walkdir::WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| !is_ignored(e, ignore_patterns))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path_to_string(path)?;
            working_dir_files.insert(relative_path.clone());

            if let Some(index_hash) = index_entries.get(&relative_path) {
                // File is tracked, check for modifications.
                let content_bytes = fs::read(path)?;
                // Normalize line endings (CRLF -> LF) before hashing to prevent platform issues.
                let content_str = String::from_utf8_lossy(&content_bytes);
                let normalized_content = content_str.replace("\r\n", "\n");
                let wd_hash = blob::hash_to_hex(&blob::compute_sha1(normalized_content.as_bytes()));

                if &wd_hash != index_hash {
                    unstaged_changes.insert(relative_path, "modified".to_string());
                }
            } else {
                // File is not tracked.
                untracked_files.insert(relative_path);
            }
        }
    }

    // Check for deleted files (in index but not in working dir)
    for path in index_entries.keys() {
        if !working_dir_files.contains(path) {
            unstaged_changes.insert(path.clone(), "deleted".to_string());
        }
    }

    Ok((unstaged_changes, untracked_files))
}

/// Prints the final status output to the console with colors.
fn print_status(result: &StatusResult) {
    if !result.staged.is_empty() {
        println!("Changes to be committed:");
        println!("  (use \"xit restore --staged <file>...\" to unstage)\\n");
        print_changes(&result.staged, "green");
        println!();
    }

    if !result.unstaged.is_empty() {
        println!("Changes not staged for commit:");
        println!("  (use \"xit add <file>...\" to update what will be committed)");
        println!("  (use \"xit restore <file>...\" to discard changes in working directory)\\n");
        print_changes(&result.unstaged, "red");
        println!();
    }

    if !result.untracked.is_empty() {
        println!("Untracked files:");
        println!("  (use \"xit add <file>...\" to include in what will be committed)\\n");
        let mut sorted_untracked: Vec<_> = result.untracked.iter().collect();
        sorted_untracked.sort();
        for path in sorted_untracked {
            println!("    {}", red!("{}", path));
        }
        println!();
    }

    if result.staged.is_empty() && result.unstaged.is_empty() && result.untracked.is_empty() {
        println!("nothing to commit, working tree clean");
    }
}

/// Helper to print a list of changes to the console with color.
fn print_changes(changes: &HashMap<String, String>, color: &str) {
    let mut sorted_changes: Vec<_> = changes.iter().collect();
    sorted_changes.sort_by_key(|(path, _)| *path);
    for (path, status) in sorted_changes {
        let status_str = format!("{:<10}", status);
        match color {
            "green" => println!("    {}  {}", green!("{}", status_str), green!("{}", path)),
            "red" => println!("    {}  {}", red!("{}", status_str), red!("{}", path)),
            _ => println!("    {}  {}", status_str, path),
        }
    }
}

/// Reads a .xitignore file and returns a set of patterns.
fn read_ignore_file(file_name: &str) -> io::Result<HashSet<String>> {
    let mut patterns = HashSet::new();
    patterns.insert(".xit".to_string()); // Always ignore the .xit directory
    patterns.insert(".git".to_string()); // Also ignore .git
    patterns.insert("target".to_string()); // Ignore rust build directory

    if let Ok(content) = fs::read_to_string(file_name) {
        for line in content.lines() {
            if !line.trim().is_empty() && !line.starts_with('#') {
                patterns.insert(line.trim().to_string());
            }
        }
    }
    Ok(patterns)
}

/// Checks if a directory entry should be ignored.
fn is_ignored(entry: &walkdir::DirEntry, ignore_patterns: &HashSet<String>) -> bool {
    entry
        .path()
        .components()
        .any(|component| match component.as_os_str().to_str() {
            Some(s) => ignore_patterns.contains(s),
            None => false,
        })
}

/// Converts a Path to a String, ensuring it's a valid relative path.
fn path_to_string(path: &Path) -> io::Result<String> {
    path.strip_prefix("./")
        .unwrap_or(path)
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Path contains invalid UTF-8"))
}

