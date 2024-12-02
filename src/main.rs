use advent_of_code_2024::{day::Day, Inputs};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();

    dbg!(&args);

    let mut inputs = Inputs::new();

    if let Some(day) = args.day {
        let input = inputs.get(Day::new(day));

        let output = String::from_utf8(input.unwrap()).unwrap();
        dbg!(output);
    }
}
