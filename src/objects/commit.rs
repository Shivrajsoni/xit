use crate::objects::blob::{compress_zlib, compute_sha1, hash_to_hex};
use std::io::Result;

pub fn create_commit(
    tree_hash: &str,
    parent_hash: Option<&str>,
    author: &str,
    committer: &str,
    message: &str,
) -> Result<String> {
    // Validate inputs
    if tree_hash.len() != 40 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Invalid tree hash length: expected 40 characters, got {}",
                tree_hash.len()
            ),
        ));
    }

    if let Some(parent) = parent_hash {
        if parent.len() != 40 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid parent hash length: expected 40 characters, got {}",
                    parent.len()
                ),
            ));
        }
    }

    if author.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Author cannot be empty",
        ));
    }

    if committer.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Committer cannot be empty",
        ));
    }

    if message.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Commit message cannot be empty",
        ));
    }

    let mut content = String::new();

    // Build commit content in Git format
    content.push_str(&format!(
        "tree {}
",
        tree_hash
    ));

    if let Some(parent) = parent_hash {
        content.push_str(&format!(
            "parent {}
",
            parent
        ));
    }

    content.push_str(&format!(
        "author {}
",
        author
    ));
    content.push_str(&format!(
        "committer {}
",
        committer
    ));
    content.push_str(&format!(
        "\n{}
",
        message
    ));

    // Create commit header: "commit {size}\0"
    let header = format!("commit {}\0", content.len());
    let data = [header.as_bytes(), content.as_bytes()].concat();

    // Compute hash and compress
    let hash = compute_sha1(&data);
    let compressed_data = compress_zlib(&data)?;
    let hash_str = hash_to_hex(&hash);

    // Create directory structure and write file
    let dir_path = format!(".xit/objects/{}", &hash_str[0..2]);
    std::fs::create_dir_all(&dir_path)?;

    let path = format!("{}/{}", dir_path, &hash_str[2..]);
    std::fs::write(path, compressed_data)?;

    Ok(hash_str)
}

// // Helper function to create initial commit (no parent)
// pub fn create_initial_commit(
//     tree_hash: &str,
//     author: &str,
//     committer: &str,
//     message: &str,
// ) -> Result<String> {
//     create_commit(tree_hash, None, author, committer, message)
// }

// // Helper function to create a commit with a single parent
// pub fn create_commit_with_parent(
//     tree_hash: &str,
//     parent_hash: &str,
//     author: &str,
//     committer: &str,
//     message: &str,
// ) -> Result<String> {
//     create_commit(tree_hash, Some(parent_hash), author, committer, message)
// }

// // Helper function to create a commit with multiple parents (merge commit)
// pub fn create_merge_commit(
//     tree_hash: &str,
//     parent_hashes: &[&str],
//     author: &str,
//     committer: &str,
//     message: &str,
// ) -> Result<String> {
//     // Validate inputs
//     if tree_hash.len() != 40 {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             format!(
//                 "Invalid tree hash length: expected 40 characters, got {}",
//                 tree_hash.len()
//             ),
//         ));
//     }

//     for (i, parent) in parent_hashes.iter().enumerate() {
//         if parent.len() != 40 {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!(
//                     "Invalid parent hash {} length: expected 40 characters, got {}",
//                     i,
//                     parent.len()
//                 ),
//             ));
//         }
//     }

//     if author.is_empty() {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "Author cannot be empty",
//         ));
//     }

//     if committer.is_empty() {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "Committer cannot be empty",
//         ));
//     }

//     if message.is_empty() {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "Commit message cannot be empty",
//         ));
//     }

//     let mut content = String::new();

//     // Build commit content with multiple parents
//     content.push_str(&format!("tree {}
// ", tree_hash));

//     for parent in parent_hashes {
//         content.push_str(&format!("parent {}
// ", parent));
//     }

//     content.push_str(&format!("author {}
// ", author));
//     content.push_str(&format!("committer {}
// ", committer));
//     content.push_str(&format!("\n{}
// ", message));

//     // Create commit header and process
//     let header = format!("commit {}\0", content.len());
//     let data = [header.as_bytes(), content.as_bytes()].concat();

//     let hash = compute_sha1(&data);
//     let compressed_data = compress_zlib(&data)?;
//     let hash_str = hash_to_hex(&hash);

//     // Create directory structure and write file
//     let dir_path = format!(".git/objects/{}", &hash_str[0..2]);
//     std::fs::create_dir_all(&dir_path)?;

//     let path = format!("{}/{}", dir_path, &hash_str[2..]);
//     std::fs::write(path, compressed_data)?;

//     Ok(hash_str)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_commit() {
        let tree_hash = "1234567890123456789012345678901234567890";
        let parent_hash = "0987654321098765432109876543210987654321";
        let author = "Author Name <author@example.com>";
        let committer = "Committer Name <committer@example.com>";
        let message = "Test commit";

        let hash = create_commit(tree_hash, Some(parent_hash), author, committer, message).unwrap();

        // Clean up created files
        let dir_path = format!(".xit/objects/{}", &hash[0..2]);
        fs::remove_dir_all(dir_path).unwrap();
    }
}
