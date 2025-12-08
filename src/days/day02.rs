use itertools::Itertools;
use std::collections::HashSet;

use crate::input::InputType;


pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    assert_eq!(input.len(), 1);
    let ans = input[0]
        .split(",")
        .into_iter()
        .map(|range| {
            let (n1, n2): (i64, i64) = match range.split_once("-") {
                Some((num_str1, num_str2)) => (
                    num_str1.parse().expect(&format!("cannot parse num str {}", num_str1)),
                    num_str2.parse().expect(&format!("cannot parse num str {}", num_str2)),
                ),
                _ => panic!("{} not following format", &range),
            };
            (n1..=n2)
                .filter(|n| check(&n.to_string()))
                .sum::<i64>()
        })
        .sum::<i64>();
    ans.to_string()
}

fn check(num: &String) -> bool {
    num.len() % 2 == 0 &&
        match num.split_at(num.len()/2) {
            (first, second) => first == second,
        }
}

// Brute force; probably can be simplified by reversed number generation
pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    assert_eq!(input.len(), 1);
    let ans = input[0]
        .split(",")
        .into_iter()
        .map(|range| {
            let (n1, n2): (i64, i64) = match range.split_once("-") {
                Some((num_str1, num_str2)) => (
                    num_str1.parse().expect(&format!("cannot parse num str {}", num_str1)),
                    num_str2.parse().expect(&format!("cannot parse num str {}", num_str2)),
                ),
                _ => panic!("{} not following format", &range),
            };
            (n1..=n2)
                .filter(|n| check2(&n.to_string()))
                .sum::<i64>()
        })
        .sum::<i64>();
    ans.to_string()
}

fn check2(num: &String) -> bool {
    (1..num.len())
        .any(|l| num.len() % l == 0 &&
            num
            .chars()
            .chunks(l)
            .into_iter()
            .map(|c| c.collect::<String>())
            .collect::<HashSet<_>>()
            .len() == 1)
}
