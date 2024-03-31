use clap::Parser;

use cmd::Cli;
use cmd::Commands::{Get, Set};

#[path = "../cmd.rs"]
mod cmd;

fn main() {
    let cli = Cli::parse();
    match &cli.commands {
        Get { key } => println!("{}", short::get_url(key).unwrap_or(String::from(""))),
        Set { value } => match short::set_url(value) {
            Ok(short) => println!("{}", short),
            Err(err) => println!("{}", err),
        },
    }
}
