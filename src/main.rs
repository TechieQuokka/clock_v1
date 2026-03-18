mod alarm;
mod parse;
mod stopwatch;
mod terminal;
mod time;
mod timer;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "clock", about = "iPhone-style clock CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the current time
    Time,

    /// Wait until HH:MM and ring an alarm
    Alarm {
        /// Target time in HH:MM format (24-hour)
        time: String,
    },

    /// Interactive stopwatch (Space=start/pause, L=lap, R=reset, Q=quit)
    Stopwatch,

    /// Countdown timer (e.g. 5m, 1h30m, 90s)
    Timer {
        /// Duration string (e.g. 5m, 1h30m, 90s)
        duration: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Time => time::run(),
        Command::Alarm { time } => alarm::run(&time),
        Command::Stopwatch => stopwatch::run(),
        Command::Timer { duration } => {
            parse::parse_duration(&duration)
                .and_then(|d| timer::run(d))
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
