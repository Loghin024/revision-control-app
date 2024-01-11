# revision-control-app

# Revision Control App

## Overview

This is a revision control application developed for educational purpose. The app provides a simplified version of git functionalities, allowing users to manage multiple branches, switch between branches, commit changes, merge branches, view differences between branches, and more.

## Installation

To use the revision control app, you need to build and run the executable. Follow these steps:

1. Clone the repository: `git clone https://github.com/yourusername/revision-control-app.git`
2. Navigate to the project directory: `cd revision-control-app`
3. Build the application: `cargo build --release`
4. Run the executable: `./target/release/revision-control-app`

## Usage

```plaintext
revision-control-app.exe <COMMAND>

Commands:
  init      Initialize a new repo
  diff      Highlight differences between the current branch and selected one
  status    Provide information about the current state (current branch, modified files)
  checkout  Branch to checkout
  merge     Merge the current branch and the selected one
  commit    Commit repository changes with a message
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# How It Works Behind the Scenes

The revision control app operates on a simple yet powerful structure behind the scenes, leveraging an acyclic graph to manage commits, branches, and the working copy.

## Repository Structure

1. **.log Folder:**
   - Contains the essential data for the revision control app.
     - **ignores File:**
       - Stores information about files and directories that should be ignored by the revision control system.
     - **branches Folder:**
       - Each branch is represented by a separate file in this folder.
       - Each file contains the hash of the commit that the branch points to.
     - **branch File:**
       - Represents the current branch in use.
       - Contains the name of the current branch.
     - **objects Folder:**
       - Holds all the blobs (binary large objects) representing the content of files at different commits.

## Acyclic Graph Structure

- **Commit Node:**
  - Each commit in the repository is represented as a node in an acyclic graph.
  - **Attributes:**
    - Message: Describes the changes made in the commit.
    - Hash of the Previous Commit: Identifies the commit's parent.
    - Hash to the Tree: Points to the tree representing the working copy of the commit.

This structure forms a clear and efficient representation of the project's history. Each commit is linked to its parent, creating a chronological sequence of changes.

## Committing Changes

1. **Creating a New Commit:**
   - When a user commits changes, a new commit node is created.
   - The new commit includes the commit message, a hash of the previous commit, and a hash to the tree of the working copy.

2. **Updating Branches:**
   - The branch file is updated to point to the hash of the newly created commit.
   - This reflects the latest commit in the branch.

## Checking Out Branches

- Switching between branches involves updating the branch file to point to the commit hash of the target branch. This effectively changes the working copy to the state represented by the selected branch.

## Merging Branches

- Merging branches involves creating a new commit node, representing the merge point that contains content from both branches. If a conflict occurs, the user is asked to select the version he wants to keep.

## Viewing Differences

- The app calculates differences between commits by comparing the content hashes in the tree structures, highlighting changes made between branches or commits.

This behind-the-scenes mechanism, stored within the `.log` folder, forms the foundation for the revision control app, providing a robust structure for managing project history and changes.
