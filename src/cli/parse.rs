use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "discordrpc")]
pub struct Cli {
    #[clap(
        short = 'c',
        long = "client-id",
        help = "Discord application Client ID",
        required = true,
        display_order = 1
    )]
    pub client_id: String,

    #[clap(
        short = 'd',
        long = "details",
        help = "Details",
        required = true,
        display_order = 2
    )]
    pub details: String,

    #[clap(
        short = 's',
        long = "state",
        help = "State",
        required = false,
        default_value = "none",
        display_order = 3
    )]
    pub state: String,

    #[clap(
        short = 'N',
        long = "large-image",
        help = "Large Image name",
        default_value = "none",
        required = false,
        display_order = 4
    )]
    pub large_image: String,

    #[clap(
        short = 'I',
        long = "large-image-text",
        help = "Large Image text",
        default_value = "none",
        required = false,
        display_order = 5
    )]
    pub large_text: String,

    #[clap(
        short = 'n',
        long = "small-image",
        help = "Small Image name",
        default_value = "none",
        required = false,
        display_order = 6
    )]
    pub small_image: String,

    #[clap(
        short = 'i',
        long = "small-image-text",
        help = "Small Image text",
        default_value = "none",
        required = false,
        display_order = 7
    )]
    pub small_text: String,

    #[clap(
        short = 'B',
        long = "button-1-text",
        help = "Button 1 Text",
        default_value = "none",
        required = false,
        display_order = 8
    )]
    pub button_1_text: String,

    #[clap(
        short = 'T',
        long = "button-1-url",
        help = "Button 1 URL address",
        default_value = "none",
        required = false,
        display_order = 9
    )]
    pub button_1_url: String,

    #[clap(
        short = 'b',
        long = "button-2-text",
        help = "Button 2 Text",
        default_value = "none",
        required = false,
        display_order = 10
    )]
    pub button_2_text: String,

    #[clap(
        short = 't',
        long = "button-2-url",
        help = "Button 2 URL address",
        default_value = "none",
        required = false,
        display_order = 11
    )]
    pub button_2_url: String,

    #[clap(
        short = 'e',
        long = "timer",
        help = "Enable timer (counted from the current time)",
        display_order = 12
    )]
    pub enable_time: bool,
}
