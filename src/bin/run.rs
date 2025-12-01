use aoc2025::{input::InputType, runner};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> [example]", args[0]);
        std::process::exit(1);
    }

    let day: u32 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Invalid day number: {}", args[1]);
            std::process::exit(1);
        }
    };

    let input_type = match args.get(2).map(String::as_str) {
        Some("example") => InputType::Example,
        _ => InputType::Real
    };

    runner::run(day, input_type);
}

