mod add;
mod downgrade;
mod rm;
mod upgrade;

// use std::env::args;

use add::add_function;
use clap::{Args, Parser, Subcommand};
use downgrade::downgrade_function;
use rm::remove_function;
use upgrade::upgrade_function;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]

enum Commands {
    Add(AddArgs),
    Rm(RemoveArgs),
    Upgrade(UpgradeArgs),
    Downgrade(DowngradeArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            add_function(args);
        }
        Commands::Rm(args) => {
            remove_function(args);
        }
        Commands::Upgrade(args) => {
            upgrade_function(args);
        }
        Commands::Downgrade(args) => {
            downgrade_function(args);
        }
    }
}
