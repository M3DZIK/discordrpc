use crate::cli::parse::Cli;

use clap::StructOpt;

pub fn state() -> String {
    let args = Cli::parse();

    if args.state != "none" {
        args.state
    } else {
        "state".to_string()
    }
}

pub fn details() -> String {
    let args = Cli::parse();

    if args.details != "none" {
        args.details
    } else {
        "details".to_string()
    }
}
