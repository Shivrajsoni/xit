use std::fs;
use std::io;
use std::path::Path;

/// Reads the HEAD file to find the path to the current branch reference (e.g., "refs/heads/main").
pub fn get_head_ref_path() -> io::Result<String> {
    let head_content = fs::read_to_string(".xit/HEAD")?;
    Ok(head_content
        .trim()
        .split_whitespace()
        .last()
        .unwrap_or("")
        .to_string())
}

/// Reads the branch reference file to get the commit's hash.
pub fn get_commit_hash(ref_path: &str) -> io::Result<String> {
    if ref_path.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "HEAD is detached or no commits yet",
        ));
    }
    fs::read_to_string(Path::new(".xit").join(ref_path)).map(|s| s.trim().to_string())
}