use nix::unistd;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Get(SubCommandGetArgs),
}

#[derive(Args, Debug)]
struct SubCommandGetArgs {
    #[arg(short='H', long)]
    hostname: bool,
    #[arg(short, long)]
    ip: bool,
}

fn handle_get(args: &SubCommandGetArgs) {
    if args.hostname{
        get_host_name(true);
    }
    if args.ip{
        get_ip();
    }
}

fn get_host_name(print_name: bool) -> String {
   let hostname = unistd::gethostname().expect("Failed to get hostname").into_string();
   if print_name {
       println!("{:#?}", hostname.as_ref().unwrap());
  }
   hostname.unwrap().to_string()
}

fn get_ip() {
    let interfaces = nix::net::if_::if_nameindex().unwrap();
    for iface in &interfaces {
        println!("Interface #{} is called {}", iface.index(), iface.name().to_string_lossy());
    }
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
