mod access;
mod my_hasher;
mod encryption;
mod memoire;
mod no_cmd_app;

use clap::{Parser, Subcommand};
use crate::memoire::{mem_get, mem_put};
use crate::no_cmd_app::no_cmd_app;


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
        site: Option<String>,
    },
    Put {
        #[arg(short, long)]
        site: Option<String>,
    },
    Memget {
        #[arg(short, long)]
        site: Option<String>,
        #[arg(short, long)]
        all: bool,
    },
    Get {
        #[arg(short, long)]
        site: Option<String>,
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Memput {site } | Commands::Put {site }) => {
            let mut val_site = String::new();
            match site {
                None => {}
                Some(data) => {
                    val_site = data.to_string();
                }
            }
            mem_put(val_site);
        }
        Some(Commands::Memget { site, all } | Commands::Get { site, all }) => {
            match site {
                Some(val) => mem_get(Some(val.to_string())),
                None => {
                    if *all == true {mem_get(None);}
                }
            }
        }
        None => {
            no_cmd_app();
        }
    }
}
