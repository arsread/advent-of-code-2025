use im::HashMap;
use memoize::memoize;

use crate::input::InputType;


type State = [usize; 4];

pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    dfs(String::from("you"), read_links(input)).iter().sum::<usize>().to_string()
}

pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    dfs(String::from("svr"), read_links(input))[0].to_string()
}

fn read_links(input: Vec<String>) -> HashMap<String, Vec<String>> {
    input
        .into_iter()
        .map(|l| {
            let (i, o) = l.split_once(":").unwrap();
            (String::from(i), o.to_string().split_whitespace().map(str::trim).map(String::from).collect()) })
        .collect()
}

#[memoize]
fn dfs(cur: String, links: HashMap<String, Vec<String>>) -> State {
    let all_nxts = links
        .get(&cur)
        .unwrap_or(&vec![])
        .iter()
        .fold(
            [0; 4],
            |acc, nxt| {
                let nxt_state = dfs(nxt.clone(), links.clone());
                std::array::from_fn(|i| acc[i] + nxt_state[i])
            }
        );
    match cur.as_str() {
        "out" => [0, 0, 0, 1],
        "dac" => [all_nxts[0] + all_nxts[2], all_nxts[1] + all_nxts[3], 0, 0],
        "fft" => [all_nxts[0] + all_nxts[1], 0, all_nxts[2] + all_nxts[3], 0],
        _ => all_nxts,
    }
}
