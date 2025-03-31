use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop(i32),
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
            "nop" => Instruction::Nop(arg),
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            other => panic!("unknown instruction {other}"),
        }
    }
}

fn handle_instruction(acc: &i32, index: &usize, instruction: &Instruction) -> (i32, usize) {
    match instruction {
        Instruction::Nop(_) => (*acc, *index + 1 ),
        Instruction::Acc(v) => (acc + v, index + 1),
        Instruction::Jmp(v) => (*acc, (*index as i32 + v) as usize),
    }
}

struct ComputationState {
    instruction: Instruction,
    acc: i32,
    index: usize,
    id: usize,
}

fn rewrite_last_rewriteable(stack: &mut Vec<ComputationState>, rewritten: Option<usize>) -> usize {
    let mut last = stack.pop().expect("unwound to bottom of stack");

    match rewritten {
        Some(earlier_rewritten) => {
            while last.id >= earlier_rewritten {
                last = stack.pop().expect("unwound to bottom of the stack");
            }},
        None => ()
    }

    let new_instruction =
        match last.instruction {
            Instruction::Acc(_) => None,
            Instruction::Nop(v) => Some(Instruction::Jmp(v)),
            Instruction::Jmp(v) => Some(Instruction::Nop(v)),
        };

    match new_instruction {
        None => rewrite_last_rewriteable(stack, None),
        Some(instruction) => {
            let stack_state = stack.last().expect("unwound to bottom of stack");

            let (acc, index) =
                handle_instruction(&stack_state.acc, &stack_state.index, &instruction);
            
            let id = stack_state.id + 1;

            stack.push(ComputationState {
                instruction,
                acc,
                index,
                id,
            });

            id
        }
    }
}

pub fn solve_part_2(instructions: &Vec<Instruction>) {
    let mut rewritten = None;

    let mut stack = Vec::<ComputationState>::new();
    stack.push(ComputationState {
        instruction: Instruction::Acc(0),
        acc: 0,
        index: 0,
        id: 0
    });
    let mut handled = HashSet::<usize>::new();

    'outer: loop {
        let last = stack.last().expect("unwound stack to bottom");

        if stack
            .iter()
            .find(|cs| cs.index == last.index && cs.id != last.id)
            .is_some()
        {
            rewritten = Some(rewrite_last_rewriteable(&mut stack, rewritten));
            println!("unwound to {}", stack.len());
        } else {
            handled.insert(last.index);
            match instructions.get(last.index) {
                Some(instruction) => {
                    let (acc, index) = handle_instruction(&last.acc, &last.index, instruction);
                    stack.push(ComputationState {
                        instruction: (*instruction).clone(),
                        acc,
                        index,
                        id: (last.id + 1)
                    });
                }
                None => {
                    println!("final accumulator is {}", last.acc);
                    break 'outer;
                }
            }
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
            let instruction = &instructions[index];

            let (new_acc, new_index) = handle_instruction(&acc, &index, instruction);
            acc = new_acc;
            index = new_index;
        }
    }

    println!("the final accumulator is {acc}");
}
