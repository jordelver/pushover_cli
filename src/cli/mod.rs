use clap::Parser;

/// Send notifications using Pushover
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Pushover user
    #[clap(short, long)]
    pub user: String,

    /// Pushover token
    #[clap(short, long)]
    pub token: String,

    /// Message to send
    #[clap(short, long)]
    pub message: String,
}
