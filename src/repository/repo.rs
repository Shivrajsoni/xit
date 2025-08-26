use std::{fs, path::Path};

// init repository
pub fn create_repository() {
    let folder_name = ".xit";

    let git_dir = Path::new(folder_name);

    if git_dir.is_dir() {
        println!("Xit Exists");
        return;
    }
    fs::create_dir_all(git_dir.join("objects")).expect("Failed to create objects directory");
    fs::create_dir_all(git_dir.join("refs/heads")).expect("Failed to create refs/heads directory");
    fs::create_dir_all(git_dir.join("refs/tags")).expect("Failed to create refs/tags directory");

    // create HEAD file
    let head_content = "ref: refs/heads/main\n";
    fs::write(git_dir.join("HEAD"), head_content).expect("Failed to write HEAD file");

    // Create config file
    let config_content =
        "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n";
    fs::write(git_dir.join("config"), config_content).expect("Failed to write config file");

    //    println!("Initialized empty Git repository in {}", git_dir.display());
}
