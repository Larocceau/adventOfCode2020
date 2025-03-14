use std::collections::HashMap;

use day7::parse;

fn main() {
    let binding = util::load_data().expect("failed to load data");

    let data: HashMap<&str, Vec<(&str, u32)>> = binding.split("\n").map(parse).collect();

    day7::solve_part_2(&data);
    day7::solve_part_2(&data)
}
