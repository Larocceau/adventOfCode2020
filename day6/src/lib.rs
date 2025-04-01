use std::collections::HashMap;

fn answered_questions(input: &str) -> usize {
    let mut vec: Vec<_> = input.chars().filter(|c| c.is_alphabetic()).collect();

    vec.sort();
    vec.dedup();

    vec.len()
}

fn answered_by_all(input: &str) -> usize {
    let people = input.chars().filter(|c| *c == '\n').count() + 1;

    let mut c_count: HashMap<char, usize> = HashMap::new();

    for c in input.chars().filter(|c| c.is_alphabetic()) {
        let count = c_count.get(&c).unwrap_or(&0) + 1;

        c_count.insert(c, count);
    }

    let mut answered_by_all = 0;

    for (_, count) in c_count {
        if people == count {
            answered_by_all += 1;
        }
    }

    answered_by_all
}

pub fn solve_part_2(data: &str) {
    let answered: usize = data.split("\n\n").map(answered_by_all).sum();

    println!("Sum of answered questions: {answered}")
}

pub fn solve_part_1(data: &str) {
    let answered: usize = data.split("\n\n").map(answered_questions).sum();

    println!("Sum of answered questions: {answered}")
}
