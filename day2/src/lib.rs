pub struct Line<'a> {
    pub lb: usize,
    pub ub: usize,
    pub character: char,
    pub password: &'a str,
}

pub fn solve_part_1(lines: &Vec<Line>) {
    let legal_lines = lines.iter().filter(|line| is_legal(*line)).count();

    println!("There are {legal_lines} legal lines");
}

pub fn solve_part_2(lines: &Vec<Line>) {
    let legal_lines = lines
        .iter()
        .filter(|line| is_legal_corrected(*line))
        .count();

    println!("There ar actually {legal_lines} legal lines");
}

pub fn is_legal(line: &Line) -> bool {
    let count = line
        .password
        .chars()
        .filter(|c| *c == line.character)
        .count();

    count >= line.lb && count <= line.ub
}

pub fn is_legal_corrected(line: &Line) -> bool {
    let chars: Vec<char> = line.password.chars().collect();
    let at_first_index = chars[line.lb - 1];
    let at_second_index = chars[line.ub - 1];

    at_first_index != at_second_index
        && (at_first_index == line.character || at_second_index == line.character)
}
