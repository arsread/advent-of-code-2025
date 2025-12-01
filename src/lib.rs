pub mod input;
pub mod runner;
pub mod days;


pub struct Day {
    pub number: u32,
    pub solve_1: fn(&str) -> String,
    pub solve_2: fn(&str) -> String,
}
