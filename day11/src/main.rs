use day11::solve_part_1;

fn main() {
    let data = util::load_data().unwrap();
    let mut parsed = day11::parse(&data);

    let res = solve_part_1(&mut parsed);

    println!("the result of part 1 is {res}")
}
