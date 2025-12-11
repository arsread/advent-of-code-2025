use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug, Clone, Copy)]
pub enum InputType {
    Real,
    Example,
}

pub fn load(day: u32, ty: InputType, part: usize) -> Vec<String> {
    let input_type = match ty {
        InputType::Real => "",
        InputType::Example => "_example",
    };
    let paths = (
        format!("inputs/day{:02}{}_{}.txt", day, input_type, part),
        format!("inputs/day{:02}{}.txt", day, input_type),
    );


    let file = File::open(&paths.0)
        .or(File::open(&paths.1))
        .unwrap_or_else(|_| panic!("Failed to open input file: {} or {}", paths.0, paths.1));

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}
