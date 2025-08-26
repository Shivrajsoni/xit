use std::io::Result;
use std::path::Path;

/// Update a Git reference to point to a specific commit
pub fn update_reference(ref_path: &str, commit_hash: &str) -> Result<()> {
    // Validate inputs
    if ref_path.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Reference path cannot be empty",
        ));
    }

    if commit_hash.len() != 40 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Invalid commit hash length: expected 40 characters, got {}",
                commit_hash.len()
            ),
        ));
    }

    // Validate commit hash format (hexadecimal)
    if !commit_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Commit hash must contain only hexadecimal characters",
        ));
    }

    // Ensure .git directory exists
    if !Path::new(".xit").exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            ".xit directory not found. Are you in a Xit repository?",
        ));
    }

    let path = format!(".git/{}", ref_path);

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(&path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write the reference with a newline (Git standard)
    std::fs::write(&path, format!("{}\n", commit_hash))?;

    Ok(())
}

/// Update HEAD reference to point to a specific commit
pub fn update_head(commit_hash: &str) -> Result<()> {
    update_reference("HEAD", commit_hash)
}

/// Update a branch reference
pub fn update_branch(branch_name: &str, commit_hash: &str) -> Result<()> {
    // Validate branch name
    if branch_name.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Branch name cannot be empty",
        ));
    }

    // Check for invalid characters in branch name
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' '];
    if branch_name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Branch name '{}' contains invalid characters", branch_name),
        ));
    }

    // Check for reserved names
    let reserved_names = ["HEAD", "ORIGIN_HEAD", "FETCH_HEAD", "MERGE_HEAD"];
    if reserved_names.contains(&branch_name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("'{}' is a reserved reference name", branch_name),
        ));
    }

    let ref_path = format!("refs/heads/{}", branch_name);
    update_reference(&ref_path, commit_hash)
}

/// Update a tag reference
pub fn update_tag(tag_name: &str, commit_hash: &str) -> Result<()> {
    // Validate tag name
    if tag_name.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Tag name cannot be empty",
        ));
    }

    // Check for invalid characters in tag name
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' '];
    if tag_name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Tag name '{}' contains invalid characters", tag_name),
        ));
    }

    let ref_path = format!("refs/tags/{}", tag_name);
    update_reference(&ref_path, commit_hash)
}

/// Create a new branch pointing to a commit
pub fn create_branch(branch_name: &str, commit_hash: &str) -> Result<()> {
    // Check if branch already exists
    let ref_path = format!("refs/heads/{}", branch_name);
    let full_path = format!(".xit/{}", ref_path);

    if Path::new(&full_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Branch '{}' already exists", branch_name),
        ));
    }

    update_branch(branch_name, commit_hash)
}

/// Create a new tag pointing to a commit
pub fn create_tag(tag_name: &str, commit_hash: &str) -> Result<()> {
    // Check if tag already exists
    let ref_path = format!("refs/tags/{}", tag_name);
    let full_path = format!(".xit/{}", ref_path);

    if Path::new(&full_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Tag '{}' already exists", tag_name),
        ));
    }

    update_tag(tag_name, commit_hash)
}

/// Delete a branch reference
pub fn delete_branch(branch_name: &str) -> Result<()> {
    if branch_name.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Branch name cannot be empty",
        ));
    }

    let ref_path = format!("refs/heads/{}", branch_name);
    let full_path = format!(".xit/{}", ref_path);

    if !Path::new(&full_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Branch '{}' does not exist", branch_name),
        ));
    }

    std::fs::remove_file(&full_path)?;
    Ok(())
}

/// Delete a tag reference
pub fn delete_tag(tag_name: &str) -> Result<()> {
    if tag_name.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Tag name cannot be empty",
        ));
    }

    let ref_path = format!("refs/tags/{}", tag_name);
    let full_path = format!(".xit/{}", ref_path);

    if !Path::new(&full_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Tag '{}' does not exist", tag_name),
        ));
    }

    std::fs::remove_file(&full_path)?;
    Ok(())
}

/// Read a reference and return the commit hash it points to
pub fn read_reference(ref_path: &str) -> Result<String> {
    if ref_path.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Reference path cannot be empty",
        ));
    }

    let path = format!(".git/{}", ref_path);

    if !Path::new(&path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Reference '{}' does not exist", ref_path),
        ));
    }

    let content = std::fs::read_to_string(&path)?;
    let commit_hash = content.trim_end_matches('\n');

    // Validate the read hash
    if commit_hash.len() != 40 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Invalid commit hash in reference: expected 40 characters, got {}",
                commit_hash.len()
            ),
        ));
    }

    if !commit_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid commit hash format in reference",
        ));
    }

    Ok(commit_hash.to_string())
}

/// Check if a reference exists
pub fn reference_exists(ref_path: &str) -> bool {
    if ref_path.is_empty() {
        return false;
    }

    let path = format!(".git/{}", ref_path);
    Path::new(&path).exists()
}

/// List all branch references
pub fn list_branches() -> Result<Vec<String>> {
    let heads_dir = ".git/refs/heads";

    if !Path::new(heads_dir).exists() {
        return Ok(Vec::new());
    }

    let mut branches = Vec::new();

    for entry in std::fs::read_dir(heads_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                branches.push(name.to_string());
            }
        }
    }

    branches.sort();
    Ok(branches)
}

/// List all tag references
pub fn list_tags() -> Result<Vec<String>> {
    let tags_dir = ".git/refs/tags";

    if !Path::new(tags_dir).exists() {
        return Ok(Vec::new());
    }

    let mut tags = Vec::new();

    for entry in std::fs::read_dir(tags_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                tags.push(name.to_string());
            }
        }
    }

    tags.sort();
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_update_reference() {
        let git_dir = ".git";
        fs::create_dir_all(git_dir).unwrap();

        let ref_path = "refs/heads/test-branch";
        let commit_hash = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";

        update_reference(ref_path, commit_hash).unwrap();

        let content = fs::read_to_string(format!("{}/{}", git_dir, ref_path)).unwrap();
        assert_eq!(content, format!("{}\n", commit_hash));

        // Clean up created files
        fs::remove_dir_all(git_dir).unwrap();
    }
}
