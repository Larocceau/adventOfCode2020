use std::io::Error;

fn main() -> Result<(), Error> {
    let binding = util::load_data()?;
    let data: Vec<&str> = binding.split("\n\n").collect();
    println!("passports: {data:?}");

    day4::part1::solve(&data);
    day4::part2::solve(&data);

    Ok(())
}
