const INIT_POS: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub fn solve_1(input: Vec<String>) -> String {
    let (_, ans) = input
        .into_iter()
        .fold((INIT_POS, 0), |(pos, cnt), ins| {
            let (dir, num) = ins.split_at(1);
            let next_pos = match dir {
                "L" => (pos - num.parse::<i32>().expect("NaN") + DIAL_SIZE) % DIAL_SIZE,
                "R" => (pos + num.parse::<i32>().expect("NaN") + DIAL_SIZE) % DIAL_SIZE,
                _ => panic!("Invalid direction!"),
            };
            (next_pos, cnt + (if next_pos == 0 { 1 } else { 0 } ))
        });
    ans.to_string()
}

pub fn solve_2(input: Vec<String>) -> String {
    let (_, ans) = input
        .into_iter()
        .fold((INIT_POS, 0), |(pos, cnt), ins| {
            let (dir, num_str) = ins.split_at(1);
            let num = num_str.parse::<i32>().expect("NaN");
            let (repeat, num_mod) = (num / DIAL_SIZE, num % DIAL_SIZE);
            let next_pos = match dir {
                "L" => pos - num_mod,
                "R" => pos + num_mod,
                _ => panic!("Invalid direction!"),
            };
            let next_pos_mod = (next_pos + DIAL_SIZE) % DIAL_SIZE;
            let cross = pos != 0 && next_pos != next_pos_mod;
            (next_pos_mod, cnt + repeat + (if next_pos == 0 || cross { 1 } else { 0 } ))
        });
    ans.to_string()
}
