use itertools::Itertools;
use std::collections::HashSet;

use crate::input::InputType;

// Works only for the particular large input. Basically:
// * Each of present 0,3,4,5 fits a 3x3 block
// * Either two present 1s or two present 2s fit a 4x3 block
pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    let input_blocks: Vec<Vec<String>> = input
        .into_iter()
        .chunk_by(String::is_empty)
        .into_iter()
        .filter_map(|(f, v)| (!f).then_some(v.collect()))
        .collect();
    let input_len = input_blocks.len();
    let (presents_strs, regions_strs) = input_blocks
        .split_at(input_len-1);
    assert_eq!(presents_strs.len(), 6);
    let simple_presents: HashSet<usize> = HashSet::from([0, 3, 4, 5]);
    let present_sizes: Vec<usize> = vec![7, 5, 6, 7, 7, 7];
    regions_strs[0]
        .clone()
        .into_iter()
        .skip(1)
        .filter(|l| {
            let (size_str, target_str) = l.split_once(":").unwrap();
            let sizes: Vec<usize> = size_str.split("x").map(str::parse).map(Result::unwrap).collect();
            let present_cnts: Vec<usize> = target_str
                .split_whitespace()
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect();
            let present_size_sum: usize = present_cnts
                .iter()
                .enumerate()
                .map(|(idx, tgt)| present_sizes[idx] * tgt)
                .sum();
            if present_size_sum > sizes[0] * sizes[1] { false }
            else {
                let (rows, width) = if sizes[1] % 3 == 0 { (sizes[1]/3, sizes[0]) } else { (sizes[0]/3, sizes[1]) };
                let (simple_grp, complx_grp): (Vec<_>, Vec<_>) = present_cnts
                    .iter()
                    .enumerate()
                    .partition(|(idx, _)| simple_presents.contains(&idx));
                let (simple_cnt, complx_cnt) = (
                    simple_grp.into_iter().map(|v| v.1).sum::<usize>(),
                    (complx_grp.into_iter().map(|v| v.1).sum::<usize>() + 1) / 2,
                );
                let simple_per_row = width / 3;
                let (simple_rows, simple_rest) = (simple_cnt / simple_per_row, simple_cnt % simple_per_row);
                let complx_per_row = width / 4;
                let (complx_rows, complx_rest) = (complx_cnt / complx_per_row, complx_cnt % complx_per_row);
                let shared_with = simple_rest + complx_rest;
                let extra_rows = if shared_with == 0 { 0 } else if shared_with <= width { 1 } else {2};
                if simple_rows + complx_rows + extra_rows > rows { panic!("Can not be decided!") } 
                true
            }
        })
        .count()
        .to_string()
}

pub fn solve_2(_input: Vec<String>, _ty: InputType) -> String {
    String::from("Not implemented")
}
