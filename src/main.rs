use clap::{Args, Parser, Subcommand};

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

    match &cli.command {
        Commands::New(args) => {
            println!("Create new LaTex project directory {:?}", args.name);
        }
    }
}
