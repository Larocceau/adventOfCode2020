fn main() {
    let data = util::load_data().expect("failed to load data");

    let parsed: Vec<day8::Instruction> = data.split('\n').map(day8::Instruction::parse).collect();

    day8::solve_part_1(&parsed);
}
