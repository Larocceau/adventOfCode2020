use std::io::Error;

use day10::solve_part_1;


fn main() -> Result<(),Error>{
    let data: Vec<u32> = util::load_data()?
        .split_ascii_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    let part1_solution = solve_part_1(&data);
    println!("the solution for part 1 is {part1_solution}");


    Ok(())
}
