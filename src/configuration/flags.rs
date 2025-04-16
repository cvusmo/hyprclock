// In src/main.rs, modify Args struct in configuration/flags.rs
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    version = "0.1.2",
    about = "Hyprclock - a clock widget for Time Wizards"
)]
pub struct Args {
    #[arg(long, action = ArgAction::SetTrue)]
    pub debug: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    pub log: bool,
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,
    #[arg(long, action = ArgAction::SetTrue)]
    pub waybar: bool,
}
