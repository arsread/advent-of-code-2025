use crate::input::{load, InputType};
use crate::days::ALL_DAYS;

pub fn run(day: u32, ty: InputType) {
    let input = load(day, ty);

    if let Some((_, solve1, solve2)) = ALL_DAYS.iter().find(|(d, _, _)| *d == day) {
        println!("=== Day {} ===", day);
        println!("Part 1: {}", solve1(input.clone(), ty.clone()));
        println!("Part 2: {}", solve2(input.clone(), ty.clone()));
    } else {
        println!("Day {} not implemented.", day);
    }
}
