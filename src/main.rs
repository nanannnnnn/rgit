mod commands;

use commands::add::add;
use commands::init::init;

use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arg {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, PartialEq, Eq)]
enum Commands {
    Init {},
    Add {
        #[arg(required = true)]
        paths: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arg::parse();
    match args.command {
        Some(Commands::Init {}) => init()?,
        Some(Commands::Add { paths }) => {
            add(paths);
        }
        None => {
            println!("Indicate a command!!!!!!!!!!!!");
        }
    }
    Ok(())
}
