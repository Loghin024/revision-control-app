use clap::Parser;
use std::{env::current_dir, io::stdout};
use std::{process, env};

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}
use lib::{dot_log::{self, JSON},  directory::Directory, commit::Commit};

#[derive(Parser, Debug)]
enum Command {
    #[clap(about = "Initialize a new repo")]
    Init,
    #[clap(about = "Highlight differences between current branch and selected one")]
    Diff{branch:String},
    #[clap(about = "Provide information about current state(current branch, modified files)")]
    Status,
    #[clap(about = "Branch to checkout")]
    Checkout{branch:String},
    #[clap(about = "Merge current branch and selected one")]
    Merge{branch:String},
    #[clap(about = "Commit repository changes with a message")]
    Commit{message:String},
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {:?}", current_dir);
    } else {
        eprintln!("Failed to get current working directory");
    }
    match args.command{
        Command::Init => {
            println!("you entered init");
            // dot_log::DotLog::init(current_dir().unwrap().join(".log")).unwrap();
            match dot_log::DotLog::init(current_dir().unwrap().join(".log")) {
                Ok(_) => {} ,
                Err(err) => {println!("{:?}", err);}
            }
        }
        Command::Diff { branch } => {
            println!("you entered diff with branch {}", branch);
        }
        Command::Status => {
            let current_branch:String;
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo)=>{
                    current_branch = dot_log::DotLog::get_branch(&repo).unwrap();
                    println!("Current branch: {}", current_branch);
                    repo
                }
                None =>{println!("Not a log repo!"); process::exit(0);}
                
            };
            let mut objects = dot_log.get_objects().expect("Error at getting objects!");
            let commit_hash = dot_log.get_branch_commit_hash(&current_branch).expect("Error at getting last commit hash from current branch!");
            let ignores = dot_log.ignores().expect("Error at getting files to be ignored!");
            let directory = Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("");
            let commit:Commit = objects.read_json(commit_hash).expect("Error at getting commit data");
            let commit_directory:Directory = objects.read_json(commit.directory).expect("e");
            // println!("{:?}\n{:?}", commit_directory., directory);
            serde_json::to_writer_pretty(stdout(), &commit_directory.diff(&directory)).expect("msg");
        }
        Command::Checkout { branch } => {
            println!("you entered checkout with branch {}", branch);
        }
        Command::Merge{branch} => {
            println!("you entered merge with branch {}", branch);
        }
        Command::Commit { message } => {
            let current_branch:String;
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo)=>{
                    current_branch = dot_log::DotLog::get_branch(&repo).unwrap();
                    println!("Current branch: {}", current_branch);
                    repo
                }
                None =>{println!("Not a log repo!"); process::exit(0);}
                
            };
            let mut objects = dot_log.get_objects().expect("Error at getting objects!");
            let last_commit_hash = dot_log.get_branch_commit_hash(&current_branch).expect("Error at getting last commit hash from current branch!");
            let ignores = dot_log.ignores().expect("Error at getting files to be ignored!");
            let directory = Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("");
            let new_commit_blob = objects.insert_json(&directory).expect("");
            let commit = Commit{
                directory:new_commit_blob,
                message:message,
                previous:vec![last_commit_hash].into_iter().collect(),
            };
            let new_commit_hash = objects.insert_json(&commit).expect("");
            dot_log.set_branch_commit_hash(&current_branch, new_commit_hash).expect("");
            // dot_log.se
            // println!("you entered commit with messsage {}", message);
        }
    }
}
