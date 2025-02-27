use day2::Line;

fn main() -> Result<(), std::io::Error> {
    let input = util::load_data()?;

    let lines: Vec<Line> = input.split('\n').map(parse_line).collect();

    println!("there are {} lines", lines.len());

    day2::solve_part_1(&lines);
    day2::solve_part_2(&lines);

    Ok(())
}

fn parse_line(line: &str) -> Line {
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
