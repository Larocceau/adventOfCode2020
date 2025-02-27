use std::{
    env,
    fs::{self, read_to_string},
    io::Read,
};

use day2::Line;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let default_file_name = "input.txt".to_string();

    let file_name = args.get(1).unwrap_or(&default_file_name);

    let mut input = String::new();
    fs::File::open(file_name)?.read_to_string(&mut input)?;

    let lines: Vec<Line> = input.split('\n').map(parseLine).collect();

    println!("there are {} lines", lines.len());

    day2::solve_part_1(&lines);
    day2::solve_part_2(&lines);

    Ok(())
}

fn parseLine(line: &str) -> Line {
    let (lb_str, rest) = line.split_once('-').expect("no - found");

    let (ub_str, rest) = rest.split_once(' ').expect("No space found!");
    let (character_str, password) = rest.split_once(':').expect("No : found");

    let lb = lb_str
        .parse::<usize>()
        .expect("lower bound is not a number");
    let ub = ub_str
        .parse::<usize>()
        .expect("upper bound is not a number");
    let character = character_str
        .parse::<char>()
        .expect("character is not a char");

    Line {
        lb,
        ub,
        character,
        password: password.trim(),
    }
}
