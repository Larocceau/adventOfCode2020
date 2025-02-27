use std::io::Error;

fn main() -> Result<(), Error> {
    let data = util::load_data()?;

    let map: Vec<Vec<char>> = data
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    print!("the map is {map:?}");

    day3::solve(&map, &vec![(3, 1)]);
    day3::solve(&map, &vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);

    Ok(())
}
