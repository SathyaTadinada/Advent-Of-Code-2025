use clap::Parser;
use year2025::{run_all, run_day, Part};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(
        default_value = "inputs",
        help = "Folder with puzzle input files."
    )]
    input_dir: String,

    #[arg(
        short,
        long,
        help = "Which day to execute (1..=25). Runs all implemented days if not specified."
    )]
    day: Option<u32>,

    #[arg(
        short,
        long,
        help = "Which part to execute (1 or 2). Runs both parts if not specified."
    )]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let part = match args.part {
        None => Part::Both,
        Some(n) => Part::from_u8(n).unwrap_or_else(|| {
            eprintln!("Invalid part {n}; must be 1 or 2.");
            std::process::exit(1);
        }),
    };

    let result = match args.day {
        Some(day) => run_day(&args.input_dir, day, part),
        None => {
            if args.part.is_some() {
                eprintln!("--part is ignored when no --day is given; running all days (both parts).");
            }
            run_all(&args.input_dir);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
