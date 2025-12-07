use im::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve_1(input: Vec<String>) -> String {
    (0..input.len())
        .fold(
            (start_pos_set(&input), 0),
            |(cur_set, cur_cnt), r| cur_set
                    .into_iter()
                    .filter_map(|y| input
                        .get(r)
                        .and_then(|l: &String| l.chars().nth(y)) 
                        .map(|c| if c == '^' { (vec![y - 1, y + 1], 1) } else { (vec![y], 0) })
                    ).fold(
                        (HashSet::new(), cur_cnt),
                        |(vec_acc, cnt_acc), (vec, cnt)| (vec_acc.union(vec.into()), cnt_acc + cnt)
        )).1
        .to_string()
}

pub fn solve_2(input: Vec<String>) -> String {
    (0..input.len())
        .fold(
            HashMap::from(vec![(start_pos_set(&input), 1)]),
            |cur, r| cur
                    .into_iter()
                    .map(|(pttn, cnt)| pttn
                        .into_iter()
                        .map(|p| if input[r].chars().nth(p).unwrap() == '^' {
                                vec![
                                    (p >= 1).then_some(p - 1),
                                    ((p + 1) < input[0].len()).then_some(p + 1),
                                ].into_iter().flatten().collect()
                            } else { vec![p] }
                        ).multi_cartesian_product() 
                        .map(|p| (p.into(), cnt))
                        .collect::<HashMap<HashSet<_>, _>>())
                    .fold(
                        HashMap::new(),
                        |acc, map| map
                            .into_iter()
                            .fold(
                                acc.to_owned(),
                                |acc_inner, (k, v)| acc_inner.update(k.clone(), *acc.get(&k).unwrap_or(&0) + v)))
        ).values()
        .sum::<i64>()
        .to_string()
}

fn start_pos_set(input: &Vec<String>) -> HashSet<usize> {
    HashSet::<usize>::from(vec![
        input
        .iter()
        .enumerate()
        .find_map(|(i, row)| row
            .chars()
            .position(|c| c == 'S')
            .map(|j| (i, j)))
        .inspect(|(row, _)| assert_eq!(*row, 0))
        .unwrap()
        .1])
}
