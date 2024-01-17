use std::fs;

use application::{
    self,
    days::one::{
        default_number_provider::DefaultNumberProvider, number_provider::NumberProvider,
        text_number_provider::TextNumberProvider,
    },
};
use clap::Args;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DayOneCalibrateArgs {
    #[arg(short = 'p')]
    text_path: String,

    #[clap(long, default_value_t = false)]
    full_output: bool,

    #[clap(long, default_value_t = 1)]
    part: i32,
}

pub fn command_one(args: DayOneCalibrateArgs) {
    let contents = fs::read_to_string(&args.text_path).expect("Unable to read text_path file");

    let number_provider: &dyn NumberProvider = match args.part {
        1 => &DefaultNumberProvider {},
        2 => &TextNumberProvider {},
        _ => panic!("Unexpected part, choose 1 or 2"),
    };

    let result = application::days::one::calibrate(&contents, number_provider);

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
