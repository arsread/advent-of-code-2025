use crate::input::InputType;


pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    input
        .into_iter()
        .map(|l| {
            let digits = l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
            match digits.iter().enumerate().rev().max_by_key(|&(_, val)| val) {
                Some((mx_idx, mx_val)) => if mx_idx == digits.len() - 1 {
                    digits[..mx_idx].iter().max().unwrap() * 10 + mx_val
                } else {
                    mx_val * 10 + digits[mx_idx+1..].iter().max().unwrap()
                }
                _ => panic!("max not found"),
            }
        }).sum::<u32>()
        .to_string()
}

pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    input
        .into_iter()
        .map(|l| {
            let digits = l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
            find(&digits, 0, digits.len(), 12).parse::<i64>()
        })
        .map(Result::unwrap)
        .sum::<i64>()
        .to_string()
}

fn find(digits: &Vec<u32>, l: usize, r: usize, size: usize) -> String {
    match size {
        0 => String::new(),
        sz => match digits
            .iter()
            .enumerate()
            .skip(l)
            .take(r-l)
            .rev()
            .max_by_key(|&(_, val)| val) {
                Some((mx_idx, mx_val)) => if digits.len() - mx_idx >= sz {
                    mx_val.to_string() + &find(digits, mx_idx + 1, digits.len(), sz - 1)
                } else {
                    find(digits, l, mx_idx, sz)
                }
                _ => panic!("max not found"),
            }
    }
}
