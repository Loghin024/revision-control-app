use clap::Parser;
use std::env::current_dir;

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}
use lib::dot_log;

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
    
    match args.command{
        Command::Init => {
            println!("you entered init");
            dot_log::DotLog::init(current_dir().unwrap().join(".log")).unwrap();
        }
        Command::Diff { branch } => {
            println!("you entered diff with branch {}", branch);
        }
        Command::Status => {
            println!("you entered status");
        }
        Command::Checkout { branch } => {
            println!("you entered checkout with branch {}", branch);
        }
        Command::Merge{branch} => {
            println!("you entered merge with branch {}", branch);
        }
        Command::Commit { message } => {
            println!("you entered commit with messsage {}", message);
        }
    }
}
