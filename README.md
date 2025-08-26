# Xit - A Simple Git-like Tool in Rust

Xit is a command-line tool that mimics some of the basic functionalities of Git. It is written in Rust and serves as a learning project to understand the inner workings of a version control system.

## Features

*   **Repository Initialization**: Create a new `.git`-like repository structure.
*   **User Configuration**: Set up a global user name and email.
*   **File Staging**: Add files to an index (staging area).
*   **Committing**: Create commits with a message.
*   **Colored Output**: User-friendly colored output for different commands.
    *   **Viewing Repository Status**: Display changes staged for commit, unstaged changes, and untracked files.

## Commands

Here are the commands currently supported by Xit:

*   `xit init`: Initializes a new repository in the current directory. It creates a `.git` directory with the necessary subdirectories and files.

*   `xit setup`: Interactively prompts you to set up your global user name and email. This information is stored in `~/.xit/config` and used for commits.

*   `xit add <file>`: Adds a file to the staging area (the index). The file's content is stored as a blob object.

*   `xit commit -m "<message>"`: Creates a new commit with the staged files. It creates a commit object and a tree object to represent the state of the repository.

*   `xit status`: Shows the status of the working tree, index, and HEAD. It lists changes staged for commit, changes not staged for commit, and untracked files.

## Installation

You can install `xit` from `crates.io` using `cargo` (once it's published):

```sh
cargo install xit
```

This will install the `xit` binary in your Cargo bin directory (e.g., `~/.cargo/bin`), so you can run it from anywhere.

Alternatively, you can build it from source:

1.  Clone the repository:
    ```sh
    git clone https://github.com/Shivrajsoni/xit.git
    cd xit
    ```

2.  Build and run:
    ```sh
    cargo run -- <command>
    ```

## Example Workflow

Once `xit` is installed, you can use it in any directory to manage your projects.

```sh
# 1. Create a new directory for your project
mkdir my-project
cd my-project

# 2. Initialize a new Xit repository
xit init

# 3. Set up your identity (only needs to be done once)
xit setup

# 4. Create a file and add it
echo "hello world" > hello.txt
xit add hello.txt

# 5. Commit the changes
xit commit -m "Initial commit"
```
