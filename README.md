# Xit - A Simple Git-like Tool in Rust

Xit is a command-line tool that mimics some of the basic functionalities of Git. It is written in Rust and serves as a learning project to understand the inner workings of a version control system.

## Features

*   **Repository Initialization**: Create a new `.xit`-like repository structure.
*   **User Configuration**: Set up a global user name and email.
*   **File Staging**: Add files to an index (staging area).
*   **Committing**: Create commits with a message.
*   **Ignoring Files**: Use a `.xitignore` file to exclude files and directories from being tracked.
*   **Viewing Repository Status**: Display changes staged for commit, unstaged changes, and untracked files.
*   **Colored Output**: User-friendly colored output for status and messages.

## Commands

Here are the commands currently supported by Xit:

*   `xit init`: Initializes a new repository in the current directory. It creates a `.xit` directory with the necessary subdirectories and files.

*   `xit setup`: Interactively prompts you to set up your global user name and email. This information is stored in `~/.xit/config` and used for commits.

*   `xit add <file>`: Adds a file to the staging area (the index). The file's content is stored as a blob object.

*   `xit commit -m "<message>"`: Creates a new commit with the staged files. It creates a commit object and a tree object to represent the state of the repository.

*   `xit status`: Shows the status of the working tree with color-coded output. It lists changes staged for commit (green), changes not staged for commit (red), and untracked files (red).

## Ignoring Files (.xitignore)

You can create a `.xitignore` file in the root of your repository to tell `xit` to ignore certain files and directories. This works similarly to Git's `.gitignore`.

Each line in the `.xitignore` file specifies a pattern. `xit` will not track any files or directories matching these patterns. The implementation currently ignores any path component that matches a line in the file (e.g., `target` will ignore any directory named `target` anywhere in the project).

By default, `xit` ignores `.xit`, `.git`, and `target`.

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

# 5. Create a file to ignore
mkdir logs
echo "dev.log" > logs/dev.log

# 6. Create a .xitignore file
echo "logs" > .xitignore

# 7. Check the status (logs/ will be ignored)
xit status

# 8. Commit the changes
xit commit -m "Initial commit"
```