use std::io::{self, Write};
use term_colr::{green, red, yellow};

pub mod objects;
pub mod repository;

pub fn run_command(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        println!("{}", yellow!("Usage: xit <command> [<args>]"));
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "init" => {
            repository::repo::create_repository();
            println!("{}", green!("Initialized empty Xit repository."));
        }
        "setup" => {
            if let Err(e) = interactive_setup() {
                println!("{}", red!("Error during setup: {}", e));
            } else {
                println!("{}", green!("Global user setup successful."));
            }
        }
        "add" => {
            if args.len() < 3 {
                println!("{}", yellow!("Usage: xit add <file>"));
                return Ok(());
            }
            let file_path = &args[2];
            if let Err(e) = repository::add::add(file_path) {
                println!("{}", red!("Error: {}", e));
            } else {
                println!("{}", green!("Added '{}' to the index.", file_path));
            }
        }
        "commit" => {
            if args.len() < 4 || args[2] != "-m" {
                println!("{}", yellow!("Usage: xit commit -m <message>"));
                return Ok(());
            }
            let message = &args[3];
            if let Err(e) = repository::commit::commit(message) {
                println!("{}", red!("Error: {}", e));
            } else {
                println!("{}", green!("Committed changes."));
            }
        }
        "status" => {
            if let Err(e) = repository::status::check_status() {
                println!("{}", red!("Error: {}", e));
            }
        }
        "diff" => {
            if args.len() < 3 {
                println!("{}", red!("Usage : xit diff "));
            }
            // checkl if any changes or anything new added to the index file , we need to keep
            // track of previous file and show the changes we make
        }
        _ => println!("{}", red!("Unknown command: {}", command)),
    }
    Ok(())
}

/// Handles the interactive setup for the user's global identity.
pub fn interactive_setup() -> io::Result<()> {
    print!("Enter your name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    print!("Enter your email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;

    repository::config::setup_global_user(name.trim(), email.trim())
}

#[cfg(test)]
mod tests {
    use super::repository;
    use std::fs;
    use std::path::Path;

    /// A full integration test for the init -> add -> commit workflow.
    #[test]
    fn test_full_workflow() {
        // 1. SETUP: Create a temporary directory for our test repository
        let temp_dir = std::env::temp_dir().join("xit_test_repo");
        // Clean up any previous test runs
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir(&temp_dir).unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // --- Test Execution ---

        // Set up a dummy global config for the test
        repository::config::setup_global_user("Test User", "test@example.com").unwrap();

        // 2. TEST `init`
        repository::repo::create_repository();
        assert!(temp_dir.join(".xit").is_dir());
        assert!(temp_dir.join(".xit/HEAD").is_file());

        // 3. TEST `add`
        let test_file_path = "hello.txt";
        fs::write(test_file_path, "hello world").unwrap();
        repository::add::add(test_file_path).unwrap();
        let index_path = temp_dir.join(".xit/index");
        assert!(index_path.is_file());
        let index_content = fs::read_to_string(index_path).unwrap();
        assert!(index_content.contains("hello.txt"));

        // 4. TEST `commit`
        let commit_message = "Initial test commit";
        repository::commit::commit(commit_message).unwrap();
        // The index should be gone after a successful commit
        assert!(!temp_dir.join(".xit/index").exists());
        // The HEAD ref should now exist and contain a commit hash
        let head_ref_path = temp_dir.join(".xit/refs/heads/main");
        assert!(head_ref_path.is_file());
        let commit_hash = fs::read_to_string(head_ref_path).unwrap();
        assert_eq!(commit_hash.trim().len(), 40);

        // --- Teardown ---

        // 5. CLEANUP: Go back to the original directory and delete the temp directory
        std::env::set_current_dir(original_dir).unwrap();
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
