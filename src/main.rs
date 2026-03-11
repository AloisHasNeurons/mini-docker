use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a command in a new isolated environment
    Run {
        /// The executable to run (e.g., /bin/bash)
        command: String,
        /// Arguments for the command
        args: Vec<String>,
    },
}

fn main() -> std::io::Result<()>{
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { command, args } => {
            println!(
                "Running [{}] with args {:?} in a new namespace...",
                command, args
            );

            // For now, we just spawn a normal child process.
            let mut child = Command::new(command)
                .args(args)
                .spawn()?;

            child.wait()?;
        }
    }
    // In case there are no errors, we return the success value with Ok()
    // Which is an empty tuple in our case : ()
    return Ok(());
}
