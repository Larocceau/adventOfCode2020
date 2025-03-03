fn main() {
    let data = util::load_data().expect("failed to load data");
    day6::solve_part_1(&data);
    println!("part 2");
    day6::solve_part_2(&data);
}
