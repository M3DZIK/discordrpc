mod cli;
mod execute;

use clap::{IntoApp, StructOpt};
use clap_complete::generate;
use colored::Colorize;
use std::{io, process::exit, thread::sleep, time::Duration};

use crate::cli::Cli;

fn main() {
    better_panic::install();

    ctrlc::set_handler(move || {
        println!("{}", "Bye!".red());
        exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();

    // * print_completions
    if let Some(shell) = args.print_completions {
        let mut clap_app = Cli::command();
        let app_name = clap_app.get_name().to_string();

        generate(shell, &mut clap_app, app_name, &mut io::stdout());
        exit(0)
    }

    // * start discord rpc
    execute::run(args);

    println!(
        "{} {}",
        "Connected!".green(),
        "Press Ctrl+C to exit!".magenta()
    );

    loop {
        // * empty `loop {}` wastes CPU cycles
        sleep(Duration::new(9999999, 9999999));
    }
}
