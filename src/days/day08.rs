use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use memoize::memoize;

use crate::input::InputType;
use crate::DSU;


pub fn solve_1(input: Vec<String>, ty: InputType) -> String {
    let (sorted_pairs, boxes) = preprocess(input);
    match ty {
        InputType::Real => sorted_pairs.into_iter().take(1000),
        InputType::Example => sorted_pairs.into_iter().take(10)
    }
        .fold(DSU::new(boxes.len()), |dsu, pair| dsu.union(pair.0, pair.1))
        .group_sizes()
        .0
        .values()
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    let (sorted_pairs, boxes) = preprocess(input);
    let last_pair = sorted_pairs
        .into_iter()
        .fold_while(
            (DSU::new(boxes.len()), None),
            |(dsu, _), pair| {
                let (grps, nxt_dsu) = dsu.union(pair.0, pair.1).group_sizes();
                if grps.len() == 1 {
                    Done((nxt_dsu, Some(pair)))
                } else {
                    Continue((nxt_dsu, None))
                }
            })
        .into_inner()
        .1
        .unwrap();
    (boxes[last_pair.0][0] * boxes[last_pair.1][0]).to_string()
}

fn dist(box0: &Vec<i64>, box1: &Vec<i64>) -> i64 {
    (box0[0] - box1[0]).pow(2) +  
        (box0[1] - box1[1]).pow(2) +  
        (box0[2] - box1[2]).pow(2)   
}

#[memoize]
fn preprocess(input: Vec<String>) -> (Vec<(usize, usize)>, Vec<Vec<i64>>) {
    let boxes: Vec<Vec<i64>> = input
        .into_iter()
        .map(|l| l.split(",").map(str::parse).map(Result::unwrap).collect())
        .collect();
    (
        (0..boxes.len())
            .combinations(2)
            .map(|p| (p[0], p[1]))
            .sorted_by_key(|p| dist(&boxes[p.0], &boxes[p.1]))
            .collect(),
        boxes,
    )
}
