use std::env;
use std::fs;
use std::io::{self, BufRead, Result, Write};
use std::path::{Path, PathBuf};

/// Represents the user's identity as found in the .xit/config file.
#[derive(Debug, Clone)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
}

/// Gets the path to the global xit config file (e.g., ~/.xit/config)
fn get_global_config_path() -> Result<PathBuf> {
    // Find the user's home directory.
    let home_dir = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find HOME directory"))?;
    let config_dir = Path::new(&home_dir).join(".xit");
    Ok(config_dir.join("config"))
}

/// Saves the user's name and email to the global config file.
pub fn setup_global_user(name: &str, email: &str) -> Result<()> {
    // Validate inputs
    if name.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "User name cannot be empty",
        ));
    }

    if email.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "User email cannot be empty",
        ));
    }

    // Basic email validation
    if !email.contains('@') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid email format",
        ));
    }

    let config_path = get_global_config_path()?;
    // Ensure the parent directory (e.g., ~/.xit) exists.
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(&config_path)?;
    writeln!(file, "[user]")?;
    writeln!(file, "    name = {}", name.trim())?;
    writeln!(file, "    email = {}", email.trim())?;

    println!("User identity saved globally to: {}", config_path.display());
    Ok(())
}

/// Reads the .xit/config file and extracts the user's name and email.
///
/// This function uses a simple line-by-line parser that looks for the `[user]`
/// section and then extracts the `name` and `email` key-value pairs.
/// Reads config from local and global files to find the user's identity.
pub fn get_user_config() -> Result<UserConfig> {
    // 1. Try to read from the local repository config first.
    let local_path = Path::new(".xit").join("config");
    if let Ok(Some(config)) = read_user_from_path(&local_path) {
        return Ok(config);
    }

    // 2. If not found locally, try the global config file.
    let global_path = get_global_config_path()?;
    if let Ok(Some(config)) = read_user_from_path(&global_path) {
        return Ok(config);
    }

    // 3. If not found anywhere, return an error.
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "User identity not found. Please run `xit setup` to configure your identity.",
    ))
}

/// A generic function to parse a user config from a given file path.
fn read_user_from_path(path: &Path) -> Result<Option<UserConfig>> {
    if !path.exists() {
        return Ok(None);
    }

    let file = fs::File::open(path)?;

    let mut in_user_section = false;
    let mut name: Option<String> = None;
    let mut email: Option<String> = None;

    for line in io::BufReader::new(file).lines() {
        let line = line?.trim().to_string();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        if line == "[user]" {
            in_user_section = true;
            continue;
        }

        // If we encounter another section, we're no longer in the user section.
        if line.starts_with('[') && line != "[user]" {
            in_user_section = false;
            continue;
        }

        if in_user_section {
            let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                match parts[0] {
                    "name" => name = Some(parts[1].to_string()),
                    "email" => email = Some(parts[1].to_string()),
                    _ => (),
                }
            }
        }
    }

    // Check if we found both name and email.
    match (name, email) {
        (Some(n), Some(e)) => Ok(Some(UserConfig { name: n, email: e })),
        _ => Ok(None),
    }
}

// /// Sets up local repository user configuration
// pub fn setup_local_user(name: &str, email: &str) -> Result<()> {
//     // Validate inputs
//     if name.trim().is_empty() {
//         return Err(io::Error::new(
//             io::ErrorKind::InvalidInput,
//             "User name cannot be empty",
//         ));
//     }

//     if email.trim().is_empty() {
//         return Err(io::Error::new(
//             io::ErrorKind::InvalidInput,
//             "User email cannot be empty",
//         ));
//     }

//     // Check if we're in a git repository
//     if !Path::new(".xit").exists() {
//         return Err(io::Error::new(
//             io::ErrorKind::NotFound,
//             "Not in a git repository. Run `xit init` first.",
//         ));
//     }

//     let config_path = Path::new(".xit").join("config");

//     // Read existing config or create new one
//     let mut config_content = String::new();
//     if config_path.exists() {
//         config_content = fs::read_to_string(&config_path)?;
//     }

//     // Check if [user] section already exists
//     if config_content.contains("[user]") {
//         // Update existing user section
//         let lines: Vec<&str> = config_content.lines().collect();
//         let mut new_lines: Vec<String> = Vec::new();
//         let mut in_user_section = false;
//         let mut user_section_updated = false;

//         for line in lines {
//             if line.trim() == "[user]" {
//                 in_user_section = true;
//                 new_lines.push(line.to_string());
//                 new_lines.push(format!("    name = {}", name.trim()));
//                 new_lines.push(format!("    email = {}", email.trim()));
//                 user_section_updated = true;
//             } else if in_user_section && line.trim().starts_with('[') {
//                 // End of user section
//                 in_user_section = false;
//                 new_lines.push(line.to_string());
//             } else if !in_user_section
//                 || (!line.trim().starts_with("name") && !line.trim().starts_with("email"))
//             {
//                 new_lines.push(line.to_string());
//             }
//         }

//         if !user_section_updated {
//             // Add user section at the end
//             new_lines.push("[user]".to_string());
//             new_lines.push(format!("    name = {}", name.trim()));
//             new_lines.push(format!("    email = {}", email.trim()));
//         }

//         config_content = new_lines.join("\n");
//     } else {
//         // Add new user section
//         if !config_content.is_empty() && !config_content.ends_with('\n') {
//             config_content.push('\n');
//         }
//         config_content.push_str(&format!(
//             "[user]\n    name = {}\n    email = {}\n",
//             name.trim(),
//             email.trim()
//         ));
//     }

//     fs::write(&config_path, config_content)?;
//     println!("User identity saved locally to: {}", config_path.display());
//     Ok(())
// }

// /// Gets the current working directory's repository config path
// pub fn get_repository_config_path() -> Result<PathBuf> {
//     let current_dir = env::current_dir()?;
//     Ok(current_dir.join(".xit").join("config"))
// }

// /// Checks if a repository has local user configuration
// pub fn has_local_user_config() -> bool {
//     let config_path = Path::new(".xit").join("config");
//     if !config_path.exists() {
//         return false;
//     }

//     if let Ok(Some(_)) = read_user_from_path(&config_path) {
//         return true;
//     }

//     false
// }