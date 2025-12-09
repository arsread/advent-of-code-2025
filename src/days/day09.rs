use itertools::Itertools;
use im::vector;

use crate::input::InputType;


pub fn solve_1(input: Vec<String>, _ty: InputType) -> String {
    let coords: Vec<Vec<i64>> = input
        .into_iter()
        .map(|l| l.split(",").map(str::parse).map(Result::unwrap).collect())
        .collect();
    (0..coords.len())
        .combinations(2)
        .map(|p| ((coords[p[0]][0] - coords[p[1]][0] + 1) * (coords[p[0]][1] - coords[p[1]][1] + 1)).abs())
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_2(input: Vec<String>, _ty: InputType) -> String {
    let coords: Vec<Vec<i64>> = input
        .into_iter()
        .map(|l| l.split(",").map(str::parse).map(Result::unwrap).collect())
        .collect();
    let (x_lines, y_lines) = coords
        .iter()
        .zip(coords.iter().cycle().skip(1))
        .fold(
            (vector![], vector![]),
            |(x_acc, y_acc), point| if point.0[0] == point.1[0] {
                let u = point.0[1].min(point.1[1]);
                let d = point.0[1].max(point.1[1]);
                (x_acc + vector![(point.0[0], (u, d))], y_acc)
            } else {
                let l = point.0[0].min(point.1[0]);
                let r = point.0[0].max(point.1[0]);
                (x_acc, y_acc + vector![((l, r), point.0[1])])
            }
        );
    (0..coords.len())
        .combinations(2)
        .flat_map(|idxs|{
            let min_x = coords[idxs[0]][0].min(coords[idxs[1]][0]);
            let max_x = coords[idxs[0]][0].max(coords[idxs[1]][0]);
            let min_y = coords[idxs[0]][1].min(coords[idxs[1]][1]);
            let max_y = coords[idxs[0]][1].max(coords[idxs[1]][1]);
            let x_cross = x_lines
                .iter()
                .filter(|(x, _)| *x > min_x && *x < max_x)
                .any(|(_, (u, d))| (*u <= min_y && *d > min_y) || (*u < max_y && *d >= max_y));
            let y_cross = y_lines
                .iter()
                .filter(|(_, y)| *y > min_y && *y < max_y)
                .any(|((l, r), _)| (*l <= min_x && *r > min_x) || (*l < max_x && *r >= max_x));
            (!x_cross && !y_cross).then(|| (max_x - min_x + 1) * (max_y - min_y + 1)) })
        .max()
        .unwrap()
        .to_string()
}
