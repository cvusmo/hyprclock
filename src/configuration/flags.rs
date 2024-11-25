// cvusmo/hyprclock/src/configuration/flags.rs

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    version = "0.1.1-103-p",
    about = "Hyprclock - a clock widget for Time Wizards"
)]

pub struct Args {
    // Run Application in debug mode
    #[arg(long, action = ArgAction::SetTrue)]
    pub debug: bool,

    // Run the application in log mode
    #[arg(long, action = ArgAction::SetTrue)]
    pub log: bool,

    // Specify config
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,
}

impl Args {
    pub fn parse_and_validate() -> Self {
        let args = Args::parse();

        // Validate mutually exclusive options
        if args.debug && args.log {
            eprintln!("Cannot run in both debug and log mode. Please choose one.");
            std::process::exit(1);
        }

        args
    }
}
