mod cli;

use std::{process::exit, vec, time::{UNIX_EPOCH, SystemTime}};
use clap::StructOpt;
use colored::Colorize;
use discord_rich_presence::{
    activity::{self, Activity},
    new_client, DiscordIpc,
};

use crate::cli::Cli;

fn main() {
    better_panic::install();

    ctrlc::set_handler(move || {
        println!("{}", "Bye!".red());
        exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();

    let state = args.state;
    let details = args.details;
    let large_image = args.large_image;
    let large_text = args.large_text;
    let small_image = args.small_image;
    let small_text = args.small_text;
    let button_1_text = args.button_1_text;
    let button_1_url = args.button_1_url;
    let button_2_text = args.button_2_text;
    let button_2_url = args.button_2_url;
    let enable_timer = args.enable_time;

    let mut client = new_client(&args.client_id).expect("failed to create client");

    let activity = Activity::new();

    client
        .connect()
        .expect("failed to connect to Discord, please try again or relaunch Discord app");

    println!("{} {}", "details :".cyan(), details.yellow());

    let mut activity: Activity = activity.details(&details);

    if state != "none" {
        println!("{} {}", "state :".cyan(), state.yellow());

        activity = activity.state(&state);
    }

    let mut assets = activity::Assets::new();

    if large_image != "none" {
        println!("{} {}", "large image :".cyan(), large_image.yellow());

        assets = assets.large_image(&large_image);
    }

    if large_text != "none" {
        println!("{} {}", "large image text :".cyan(), large_text.yellow());

        assets = assets.large_text(&large_text);
    }

    if small_image != "none" {
        println!("{} {}", "small image :".cyan(), small_image.yellow());

        assets = assets.small_image(&small_image);
    }

    if small_text != "none" {
        println!("{} {}", "small image text :".cyan(), small_text.yellow());

        assets = assets.small_text(&small_text);
    }

    activity = activity.assets(assets);

    if button_1_text != "none" && button_1_url != "none" {
        println!("{} {}", "button 1 text :".cyan(), button_1_text.yellow());
        println!("{} {}", "button 1 url :".cyan(), button_1_url.yellow());

        activity = activity.buttons(vec![activity::Button::new(&button_1_text, &button_1_url)]);
    }

    if button_1_text != "none" && button_1_url != "none" && button_2_text != "none" && button_2_url != "none" {
        println!("{} {}", "button 2 text :".cyan(), button_2_text.yellow());
        println!("{} {}", "button 2 url :".cyan(), button_2_url.yellow());

        activity = activity.buttons(vec![activity::Button::new(&button_1_text, &button_1_url), activity::Button::new(&button_2_text, &button_2_url)]);
    }

    if enable_timer {
        let time_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        activity = activity.timestamps(activity::Timestamps::new().start(time_unix))
    }

    client.set_activity(activity).expect("client set activity");

    println!(
        "{} {}",
        "Connected!".green(),
        "Press Ctrl+C to exit!".magenta()
    );

    loop {}
}
