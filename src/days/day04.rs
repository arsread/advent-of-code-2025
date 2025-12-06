use itertools::Itertools;

pub fn solve_1(input: Vec<String>) -> String {
    let (mtx, m, n) = normalize(input);
    (0..m)
        .cartesian_product(0..n)
        .filter(|(i, j)| mtx[*i][*j] &&
            neighbors(m, n, *i, *j)
            .filter(|(x, y)| mtx[*x][*y])
            .count() < 4
        ).count()
        .to_string()
}
    
pub fn solve_2(input: Vec<String>) -> String {
    let (mtx, m, n) = normalize(input);
    let (init_q, rest): (Vec<_>, Vec<_>) = (0..m)
        .cartesian_product(0..n)
        .filter_map(|(i, j)| mtx[i][j]
            .then_some(((i, j), neighbors(m, n, i, j).filter(|(x, y)| mtx[*x][*y]).count()))
        ).partition(|(_, c)| *c < 4);
    std::iter::successors(
        Some((init_q.len(), im::Vector::from(init_q), im::HashMap::from(rest))),
        |(acc, rs, cnts)| {
            let (nxt_q, nxt_cnts) = rs
                .into_iter()
                .flat_map(|r| neighbors(m, n, r.0.0, r.0.1))
                .into_grouping_map_by(|v| *v)
                .fold(0, |acc, _, _| acc + 1)
                .into_iter()
                .fold::<(_, im::HashMap<_,_>), _>(
                    (im::vector![], cnts.to_owned()),
                    |(queue, cur_cnts), ((x, y), minus)| {
                        if let Some(cur_cnt) = cur_cnts.get(&(x, y)) {
                            let nxt_cnt = cur_cnt - minus;
                            if nxt_cnt < 4 {
                                (im::vector![((x, y), nxt_cnt)] + queue, cur_cnts.without(&(x, y)))
                            } else {
                                (queue, cur_cnts.update((x, y), nxt_cnt))
                            }
                        } else {
                            (queue, cur_cnts)
                        }
                    }
                );
            (!nxt_q.is_empty()).then_some((acc + nxt_q.len(), nxt_q, nxt_cnts))
        }
    ).last()
    .unwrap()
    .0
    .to_string()
}


fn normalize(input: Vec<String>) -> (Vec<Vec<bool>>, usize, usize) {
    let mtx = input
        .into_iter()
        .map(|l| l.chars().into_iter().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let m = mtx.len();
    let n = mtx[0].len();
    (mtx, m, n)
}


fn neighbors(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)>  {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter_map(move |(dx, dy)| {
            let ni = i as isize + dx;
            let nj = j as isize + dy;
            (ni >= 0 && nj >= 0 && (ni != i as isize || nj != j as isize) && ni < rows as isize && nj < cols as isize)
                .then_some((ni as usize, nj as usize))
        })
}
