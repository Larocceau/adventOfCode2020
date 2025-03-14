use std::{collections::HashSet, ops::Sub};

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        let (op, arg) = input
            .split_once(' ')
            .expect(&format!("no space found in '{input}'"));

        let arg = arg.parse().expect("failed to parse");

        match op {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            other => panic!("unknown instruction {other}"),
        }
    }
}

pub fn solve_part_1(instructions: &Vec<Instruction>) {
    let mut handled = HashSet::<usize>::new();

    let mut acc = 0;
    let mut index: usize = 0;

    loop {
        if handled.contains(&index) {
            break;
        } else {
            handled.insert(index);
            match instructions[index] {
                Instruction::Nop => index += 1,
                Instruction::Acc(v) => {
                    acc += v;
                    index += 1;
                }
                Instruction::Jmp(v) => index = (index as i32 + v) as usize,
            }
        }
    }

    println!("the final accumulator is {acc}");
}
