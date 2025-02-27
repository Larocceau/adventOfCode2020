use std::io::Error;

fn main() -> Result<(), Error> {
    let data = util::load_data()?;

    let map: Vec<Vec<char>> = data
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    print!("the map is {map:?}");

    day3::solve_part_1(&map, &3);

    Ok(())
}
