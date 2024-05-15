mod access;
mod my_hasher;
mod encryption;
mod memoire;

use clap::{Parser, Subcommand};
use crate::memoire::{mem_get, mem_put};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Memput {
        #[arg(short, long)]
        password: Option<String>,
        #[arg(short, long)]
        site: Option<String>,
    },
    Memget {
        #[arg(short, long)]
        site: Option<String>,
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Memput { password: mdp, site }) => {
            let mut val_mdp = String::new();
            let mut val_site = String::new();
            match mdp {
                None => {}
                Some(data) => {
                    val_mdp = data.to_string();
                }
            }
            match site {
                None => {}
                Some(data) => {
                    val_site = data.to_string();
                }
            }
            mem_put(val_mdp, val_site);
        }
        Some(Commands::Memget { site, all }) => {
            match site {
                Some(val) => mem_get(Some(val.to_string())),
                None => {
                    if *all == true {mem_get(None);}
                }
            }
        }
        None => {
            println!("No command entered, try help to get more info");
        }
    }
}
