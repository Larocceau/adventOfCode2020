use std::io::Error;

use day10::solve_part_1;


fn main() -> Result<(),Error>{
    let data: Vec<usize> = util::load_data()?
        .split_ascii_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let part1_solution = solve_part_1(&data);
    println!("the solution for part 1 is {part1_solution}");

    let part_2_solution = day10::solve_part_2(&data);

    println!("solution for part 2 is {part_2_solution}");


    Ok(())
}
