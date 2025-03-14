fn main() {
    let data = util::load_data().expect("failed to load data");

    day7::solve_part_1(&data)

}
