use itertools::Itertools;
use std::collections::BTreeMap;


pub fn solve_1(input: Vec<String>) -> String {
    let splt_pos = input.iter().position(String::is_empty).unwrap();
    let (ranges, elements) = input.split_at(splt_pos);

    let range_map = get_range_map(ranges);

    elements
        .into_iter()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .filter(|e| range_map 
            .range(..=e)
            .next_back()
            .map(|(_, v)| e <= v)
            .unwrap_or_default())
        .count()
        .to_string()
}

pub fn solve_2(input: Vec<String>) -> String {
    let splt_pos = input.iter().position(String::is_empty).unwrap();
    let (ranges, _) = input.split_at(splt_pos);

    get_range_map(ranges)
        .into_iter()
        .map(|(k, v)| v - k + 1)
        .sum::<i64>()
        .to_string()
}

fn get_range_map(ranges: &[String]) -> BTreeMap<i64, i64>{
    ranges
        .into_iter()
        .flat_map(|l| { 
            let (start, end) = l.split_once("-").unwrap();
            [(start.parse::<i64>().unwrap(), -1), (end.parse::<i64>().unwrap(), 1)]})
        .sorted()
            .fold((0, 0, im::vector![]),
            |(cur_start, cur_cnt, r_lst), (val, mode)| {
                let new_cnt = cur_cnt - mode;
                match new_cnt {
                    _ if cur_cnt == 0 => (val, new_cnt, r_lst),
                    0 => (0, new_cnt, r_lst + im::vector![(cur_start, val)]),
                    _ => (cur_start, new_cnt, r_lst)
                }
            }).2
        .into_iter()
        .collect()
}
