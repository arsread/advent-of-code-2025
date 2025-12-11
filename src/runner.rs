use crate::input::{load, InputType};
use crate::days::ALL_DAYS;

pub fn run(day: u32, ty: InputType) {
    let input_1 = load(day, ty, 1);
    let input_2 = load(day, ty, 2);

    if let Some((_, solve1, solve2)) = ALL_DAYS.iter().find(|(d, _, _)| *d == day) {
        println!("=== Day {} ===", day);
        println!("Part 1: {}", solve1(input_1.clone(), ty.clone()));
        println!("Part 2: {}", solve2(input_2.clone(), ty.clone()));
    } else {
        println!("Day {} not implemented.", day);
    }
}
