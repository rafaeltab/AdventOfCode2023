use one::command_one;
use three::command_three;
use two::command_two;

use clap::{Parser, Subcommand};

mod one;
mod two;
mod three;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, name = "aoc")]
struct AocCli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Subcommand, Debug)]
enum MainCommands {
    #[command(subcommand)]
    Day(Day),
}

#[derive(Subcommand, Debug)]
enum Day {
    One(one::DayOneCalibrateArgs),
    Two(two::DayTwoCalibrateArgs),
    Three(three::DayThreePartsArgs),
}


fn main() {
    let args = AocCli::parse();

    match args.command {
        MainCommands::Day(day) => match day {
            Day::One(args) => {
                command_one(args);
            }
            Day::Two(args) => {
                command_two(args);
            }
            Day::Three(args) => {
                command_three(args);
            }
        },
    }
}
