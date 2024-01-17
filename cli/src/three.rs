use std::fs;

use clap::Args;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DayThreePartsArgs {
    #[arg(short = 'p')]
    text_path: String,
}

pub fn command_three(args: DayThreePartsArgs) {
    let contents = fs::read_to_string(&args.text_path).expect("Unable to read text_path file");

    let result = application::days::three::find_valid_numbers(&contents);

    println!(
        "The result for your input is: {}",
        result.into_iter().sum::<u32>()
    );
}
