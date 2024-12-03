use advent_of_code_2024::{day::Day, template::create_files};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Day { day: Day },
    Setup { day: Day },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Day { day } => {
            // let input = inputs.get(day);
            //
            // let output = String::from_utf8(input.unwrap()).unwrap();
            // dbg!(output);
        }
        Commands::Setup { day } => {
            create_files(day);
        }
    }
}
