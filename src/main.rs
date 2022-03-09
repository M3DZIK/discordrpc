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

    // * print completions
    if let Some(shell) = args.print_completions {
        let mut clap_app = Cli::command();
        let app_name = clap_app.get_name().to_string();

        generate(shell, &mut clap_app, app_name, &mut io::stdout());
        exit(0)
    }

    // * print manpage
    if args.manpage {
        let clap_app = Cli::command();
        let man = clap_mangen::Man::new(clap_app);

        man.render(&mut io::stdout()).expect("generate manpage");
        exit(0)
    }

    // * start discord rpc
    execute::run(args.clone());

    println!(
        "{} {}",
        "Connected!".green(),
        "Press Ctrl+C to exit!".magenta()
    );

    if args.timeout != 0 {
        sleep(Duration::from_secs(args.timeout));

        println!("{}", "Stopping due to timeout...".blue());

        exit(0)
    } else {
        loop {
            // * empty `loop {}` wastes CPU cycles
            sleep(Duration::from_secs(9999999));
        }
    }
}
