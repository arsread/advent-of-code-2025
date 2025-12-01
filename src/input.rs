use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug, Clone, Copy)]
pub enum InputType {
    Real,
    Example,
}

pub fn load(day: u32, ty: InputType) -> Vec<String> {
    let suffix = match ty {
        InputType::Real => "",
        InputType::Example => "_example",
    };
    let path = format!("inputs/day{:02}{}.txt", day, suffix);


    let file = File::open(&path)
        .unwrap_or_else(|_| panic!("Failed to open input file: {}", path));

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}
