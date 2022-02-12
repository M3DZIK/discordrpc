mod cli;

use crate::cli::*;

use clap::StructOpt;
use discord_rich_presence::{activity::{Activity, self}, new_client, DiscordIpc};
use colored::Colorize;

fn main() {
    better_panic::install();

    let args = Cli::parse();

    let state = state();
    let details = details();
    let large_image = args.large_image;
    let large_text = args.large_text;
    let small_image = args.small_image;
    let small_text = args.small_text;

    let mut client = new_client(&args.client_id).expect("failed to create client");

    let activity = Activity::new();

    client
        .connect()
        .expect("failed to connect to Discord, Please try again or relaunch Discord");

    let mut activity: Activity = activity.state(&state);
    activity = activity.details(&details);

    let mut assets = activity::Assets::new();

    if large_image != "none" {
        assets = assets.large_image(&large_image);
    }

    if large_text != "none" {
        assets = assets.large_text(&large_text);
    }

    if small_image != "none" {
        assets = assets.small_image(&large_image);
    }

    if small_text != "none" {
        assets = assets.small_text(&large_text);
    }

    activity = activity.assets(assets);

    client.set_activity(activity).expect("client set activity");

    println!("{} {}", "state :".cyan(), state.yellow());

    print!("{} {}\n", "Connected!".green(), "Press Ctrl+C to exit!".magenta());

    loop {}
}
