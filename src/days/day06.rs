use itertools::izip;

use crate::input::InputType;


pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    let rotated = rotate(
        input
            .iter()
            .take(input.len()-1)
            .map(|l| l.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
    calc(input.iter().last().unwrap(), rotated).to_string()
}

pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    let rotated: Vec<String> = rotate(input
            .iter()
            .take(input.len()-1)
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>());
    let numgrps = rotated
        .split(|l| l.chars().all(char::is_whitespace))
        .map(|grp| grp.iter().map(|l| l.trim().parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect();
    calc(input.iter().last().unwrap(), numgrps).to_string()
}

fn rotate<T: Clone, U:FromIterator<T>>(mtx: Vec<Vec<T>>) -> Vec<U> {
    (0..mtx[0].len())
        .map(|j| mtx.iter().map(|row| row[j].to_owned()).collect::<U>())
        .collect::<Vec<U>>()
}

fn calc(ops_line: &String, numgrps: Vec<Vec<i64>>) -> i64 {
    let ops: Vec<_> = ops_line.split_whitespace().collect();
    izip!(ops, numgrps)
        .map(|(op, nums)| match op {
            "+" => nums.iter().sum::<i64>(),
            _ => nums.iter().product(),
        })
        .sum::<i64>()
}
