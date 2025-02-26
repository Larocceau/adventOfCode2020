use std::{
    env, fs,
    io::{Error, Read},
};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let default_file_name = "input.txt".to_string();

    let file_name = args.get(1).unwrap_or(&default_file_name);

    let mut input = String::new();
    fs::File::open(file_name)?.read_to_string(&mut input)?;

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
