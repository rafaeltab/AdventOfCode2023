use application::{self, days::one::{default_number_provider::DefaultNumberProvider, text_number_provider::TextNumberProvider, number_provider::NumberProvider}};
use std::fs;

use clap::{Args, Parser, Subcommand};

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
    One(DayOneCalibrateArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
struct DayOneCalibrateArgs {
    #[arg(short = 'p')]
    text_path: String,

    #[clap(long, default_value_t = false)]
    full_output: bool,

    #[clap(long, default_value_t = 1)]
    part: i32,
}

fn main() {
    let args = AocCli::parse();

    match args.command {
        MainCommands::Day(day) => match day {
            Day::One(args) => {
                let contents =
                    fs::read_to_string(&args.text_path).expect("Unable to read text_path file");

                let number_provider: &dyn NumberProvider = match args.part {
                    1 => &DefaultNumberProvider{},
                    2 => &TextNumberProvider{},
                    _ => panic!("Unexpected part, choose 1 or 2")
                };

                let result =
                    application::days::one::calibrate(&contents, number_provider);

                if args.full_output {
                    for res in &result {
                        println!("{}", res);
                    }
                }

                println!(
                    "The result for your input is: {}",
                    result.iter().sum::<u32>()
                );
            }
        },
    }
}