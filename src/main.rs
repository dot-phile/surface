mod commands {
    pub mod get;
}

use clap::{Parser, Subcommand};
use commands::get::*;


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Get(SubCommandGetArgs),
}


fn main() {
   let cli = Cli::parse();
   match &cli.command {
       Some(Commands::Get(args)) => handle_get(args),
       None => {
           println!("No args");
       }
   }
}
