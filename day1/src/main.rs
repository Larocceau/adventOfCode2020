use std::io::Error;

fn main() -> Result<(), Error> {
    let input = util::load_data()?;
    let mut data: Vec<i32> = Vec::new();

    for entry in input.split("\n") {
        let as_int = entry.trim().parse::<i32>().unwrap();
        data.push(as_int);
    }
    let target: i32 = 2020;

    day1::solve_part_1(&data, &target);
    day1::solve_part_2(&data, &target);

    Ok(())
}
