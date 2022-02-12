use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "discordrpc")]
pub struct Cli {
    #[clap(
        short = 'c',
        help = "Discord application Client ID",
        required = true,
        display_order = 1
    )]
    pub client_id: String,

    #[clap(short = 's', help = "State", default_value = "none", required = false)]
    pub state: String,

    #[clap(
        short = 'd',
        help = "Details",
        default_value = "none",
        required = false
    )]
    pub details: String,

    #[clap(
        short = 'N',
        help = "Large Image name",
        default_value = "none",
        required = false
    )]
    pub large_image: String,

    #[clap(
        short = 'I',
        help = "Large Image text",
        default_value = "none",
        required = false
    )]
    pub large_text: String,

    #[clap(
        short = 'n',
        help = "Small Image name",
        default_value = "none",
        required = false
    )]
    pub small_image: String,

    #[clap(
        short = 'i',
        help = "Small Image text",
        default_value = "none",
        required = false
    )]
    pub small_text: String,
}
