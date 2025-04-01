
fn main() {
    let data: Vec<usize> = util::load_data().unwrap().split_ascii_whitespace().map(|v| v.parse::<usize>().unwrap()).collect();

    let solution_part1 = day9::first_without_matching(&data, 25).unwrap();
    println!("the solution for part 1 is {solution_part1}");
    let solution_chunk = day9::find_chunk_that_adds_to(&data, *solution_part1).unwrap();
    let encyption_weakness = day9::to_encryption_weakness(solution_chunk).unwrap();
    println!("solution for part 2 is {}", encyption_weakness)

    // 7202279 is too low
}
