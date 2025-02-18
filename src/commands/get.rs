use nix::unistd;
use clap::Args;

#[derive(Args, Debug)]
pub struct SubCommandGetArgs {
    #[arg(short='H', long)]
    pub hostname: bool,
    #[arg(short, long)]
    pub interfaces: bool,
}

pub fn handle_get(args: &SubCommandGetArgs) {
    if args.hostname{
        match get_host_name() {
            Ok(hostname) => println!("Hostname: {}", hostname),
            Err(e) => println!("Error: {}", e),
        }

    }
    if args.interfaces{
        get_interfaces();
    }
}

pub fn get_host_name() -> Result<String, String> {
   let hostname = unistd::gethostname().map_err(|_| "Failed to get hostname")?;
   match hostname.into_string() {
       Ok(hostname) => Ok(hostname),
       Err(_) => Err("Failed to convert hostname to String".to_string()),
   }


}

pub fn get_interfaces() {
    let interfaces = nix::net::if_::if_nameindex().unwrap();
    for iface in &interfaces {
        println!("Interface #{} is called {}", iface.index(), iface.name().to_string_lossy());
    }
}

#[cfg(test)]
mod get_tests {
    use super::*;

    #[test]
    fn hostname_should_not_be_error() {
        let hostname = get_host_name();
        assert!(hostname.is_ok());
    }
}
