use std::io::Write;
use itertools::Itertools;

fn main() {
    let days: Vec<u32> = std::fs::read_dir("src/days")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            let file = path.file_stem()?.to_str()?;
            if file.starts_with("day") && file.len() == 5 && path.extension()? == "rs" {
                Some(file[3..].parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sorted()
        .collect();

    let includes = days
    .iter()
    .map(|d| format!(
        "pub mod day{:02} {{ include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/days/day{:02}.rs\")); }}",
        d, d
    ))
    .collect::<Vec<_>>();


    let days_maps = days.iter().map(|d| {
        format!(
            "    ({},
             day{:02}::solve_1,
             day{:02}::solve_2),",
            d, d, d
        )
    }).collect::<Vec<_>>();


    let registrations = [
        includes,
        vec![String::from("pub const ALL_DAYS: &[(u32, fn(Vec<String>)->String, fn(Vec<String>)->String)] = &[")],
        days_maps,
        vec![String::from("];\n")],
    ].concat().join("\n");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = std::fs::File::create(format!("{}/days_generated.rs", out_dir)).unwrap();
    file.write_all(registrations.as_bytes()).unwrap();
}

