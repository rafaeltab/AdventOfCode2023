use std::fs;

use clap::Args;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DayTwoCalibrateArgs {
    #[arg(short = 'p')]
    text_path: String,

    #[clap(long, default_value_t = 1)]
    part: i32,
}

pub fn command_two(args: DayTwoCalibrateArgs) {
    let contents = fs::read_to_string(&args.text_path).expect("Unable to read text_path file");

    if args.part == 1 {
        let result = application::days::two::extract_possible_games(&contents, 12, 14, 13);

        println!(
            "The result for your input is: {}",
            result.into_iter().map(|x| x.nr).sum::<i32>()
        );
    } else {
        let result = application::days::two::extract_least_cubes(&contents);

        println!(
            "The result for your input is: {}",
            result.into_iter().map(|x| x.power()).sum::<i32>()
        );
    }
}
