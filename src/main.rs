use clap::{Args, Parser, Subcommand};
use log::{log, LogType};
use std::{env, fs::create_dir, path::PathBuf};

mod log;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new LaTeX project directoy with selected template
    New(NewArgs),
}

#[derive(Args)]
struct NewArgs {
    /// New project directory name
    name: String,
}

fn main() {
    let cli = Cli::parse();

    let mut cwd = PathBuf::new();
    if let Ok(path) = env::current_dir() {
        cwd = path;
    }
    log(
        LogType::Debug,
        format!("Current Working Directory: {:?}", cwd),
    );

    match &cli.command {
        Commands::New(args) => {
            cwd.push(&args.name);
            log(LogType::Info, format!("Creating new project at {:?}", cwd));

            match create_dir(cwd) {
                Ok(_) => {
                    log(
                        LogType::Success,
                        format!("Created new project {:?}", &args.name),
                    );
                }
                Err(_) => {
                    log(LogType::Error, format!("Could not create {:?}", &args.name));
                }
            }
        }
    }
}
