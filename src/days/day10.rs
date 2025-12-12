use hamming_iter::hamming_iter;
use good_lp::*;
use regex::Regex;
use std::ops::BitXor;
use std::sync::LazyLock;


use crate::input::InputType;


static LINE_PARSER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[(.*)\] (.*) \{(.*)\}").unwrap()
});

pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    input
        .into_iter()
        .map(|l| {
            let caps = LINE_PARSER.captures(&l).unwrap();
            let desired = caps[1].chars().rev().fold(0, |num, c| (num << 1) + (if c == '#' { 1 } else { 0 }));
            let changes: Vec<u32> = caps[2]
                .split_whitespace()
                .map(|t| t[1..t.len()-1]
                    .split(",")
                    .map(str::parse::<usize>)
                    .map(Result::unwrap)
                    .fold(0, |num, bit| num | (1 << bit))
                ).collect();
            hamming_iter(changes.len() as u32)
                .find(|mask| changes
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(idx, change)| (((mask >> idx) & 1) == 1).then_some(change))
                    .reduce(BitXor::bitxor)
                    .unwrap_or(0) == desired)
                .unwrap()
                .count_ones() })
        .sum::<u32>()
        .to_string()
}

pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    input
        .into_iter()
        .map(|l| {
            let caps = LINE_PARSER.captures(&l).unwrap();
            let target: Vec<usize> = caps[3]
                .split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            let vectors: Vec<Vec<usize>> = caps[2]
                .split_whitespace()
                .map(|t| { 
                    let pos = t[1..t.len()-1]
                    .split(",")
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<usize>>();
                    (0..target.len())
                        .map(|i| if pos.contains(&i) { 1 } else {0})
                        .collect()
                }).collect();
           let n = vectors.len();
           let m = target.len();


           let pv = ProblemVariables::new();

           let (pv, xs) = (0..n).fold((pv, Vec::new()), |(mut pv, mut xs), _| {
               xs.push(pv.add(variable().integer().min(0)));
               (pv, xs)
           });

           let objective = xs.iter().cloned().sum::<Expression>();

           let model = (0..m).fold(
               pv.minimise(objective).using(default_solver),
               |model_acc, j| {
                   let expr = xs.iter()
                       .enumerate()
                       .map(|(i, x)| *x * (vectors[i][j] as f64))
                       .sum::<Expression>();
                   model_acc.with(constraint!(expr == target[j] as f64))
               },
           );

            let solution = model.solve().unwrap();
            xs.iter().map(|x| solution.value(*x).round() as usize).sum::<usize>()
        })
    .sum::<usize>()
    .to_string()
}


// Attempt with an a* -- still slow. Will try to improve the heuristic when I have time before
// Christmas...
//
// use im::{HashSet, Vector};
// use itertools::Itertools;
// use itertools::iproduct;
// pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
//     input
//         .into_iter()
//         .map(|l| {
//             let caps = LINE_PARSER.captures(&l).unwrap();
//             let target: Vec<usize> = caps[3]
//                 .split(",")
//                 .map(str::parse)
//                 .map(Result::unwrap)
//                 .collect();
//             let vectors: Vec<(usize, Vec<usize>)> = caps[2]
//                 .split_whitespace()
//                 .map(|t| { 
//                     t[1..t.len()-1]
//                     .split(",")
//                     .map(str::parse)
//                     .map(Result::unwrap)
//                     .collect()})
//                 .enumerate()
//                 .collect();
//             let init_status = target
//                 .into_iter()
//                 .enumerate()
//                 .map(|(idx, t)| (
//                         vectors
//                             .iter()
//                             .filter_map(|(v_idx, vct)| vct.contains(&idx).then_some(*v_idx))
//                             .collect::<HashSet<_>>(),
//                         t))
//                 .collect();
//             sub_solve(Vector::from(vec![None; vectors.len()]), init_status)
//                 .unwrap()
//                 .into_iter()
//                 .sum::<usize>()
//         })
//     .sum::<usize>()
//     .to_string()
// }

// fn sub_solve(ans: Vector<Option<usize>>, status: HashSet<(HashSet<usize>, usize)>) -> Option<Vector<usize>> {
//     if status.is_empty() { Some(ans.into_iter().map(Option::unwrap).collect()) }
//     else {
//         let (most_used, min_val) = ans
//             .iter()
//             .enumerate()
//             .filter_map(|(idx, e)| (e.is_none()).then_some(idx))
//             .map(|idx| {
//                 let eq_vals = status.iter().filter_map(|(s, v)| (s.contains(&idx)).then_some(v)).collect::<Vector<_>>();
//                 (eq_vals.len(), eq_vals.into_iter().min().unwrap(), idx) })
//             .max()
//             .unwrap();
//         (0..=*min_val)
//             .rev()
//             .find_map(|target| {
//                 (status.iter().all(|(s, rhs)| !s.contains(&most_used) || *rhs >= target) ||
//                     ans[most_used].is_none_or(|v| v == target))
//                 .then(|| {
//                     let nxt_ans = ans.update(most_used, Some(target));
//                     let (empty_eqs, nxt_status): (Vector<_>, Vector<_>) = status
//                         .clone()
//                         .into_iter()
//                         .map(|(s, v)| (s.without(&most_used), if s.contains(&most_used) { v - target } else { v }))
//                         .partition(|(s, _)| s.is_empty());
//                     (empty_eqs
//                         .into_iter()
//                         .map(|(_, v)| v)
//                         .collect::<HashSet<usize>>()
//                         .len() <= 1)
//                         .then(|| sub_solve(nxt_ans, nxt_status.into_iter().collect()))
//                         .flatten()
//                 })
//                 .flatten()
//             })
//     }
// }

