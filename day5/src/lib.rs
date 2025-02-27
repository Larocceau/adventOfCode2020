enum Instruction {
    Upper,
    Lower,
}

fn traverse<T, E, I>(iter: I) -> Result<Vec<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    let mut collected = Vec::new();

    for item in iter {
        match item {
            Ok(value) => collected.push(value),
            Err(e) => return Err(e),
        }
    }

    Ok(collected)
}

pub fn row_postion(instructions: &str) -> Result<i32, String> {
    let instructions = traverse(instructions.chars().map(|c| match c {
        'F' => Ok(Instruction::Lower),
        'B' => Ok(Instruction::Upper),
        other => Err(format!("Unknown instruction {}", other)),
    }))?;

    Ok(location(instructions, &0, &127))
}

pub fn column_postion(instructions: &str) -> Result<i32, String> {
    let instructions = traverse(instructions.chars().map(|c| match c {
        'L' => Ok(Instruction::Lower),
        'R' => Ok(Instruction::Upper),
        other => Err(format!("Unknown instruction {}", other)),
    }))?;

    Ok(location(instructions, &0, &7))
}

pub fn seat_position(instructions: &str) -> Result<i32, String> {
    let (row_instructions, column_instructions) = instructions.split_at(7);

    let column = column_postion(column_instructions)?;
    let row = row_postion(row_instructions)?;

    Ok(row * 8 + column)
}

pub fn solve(data: &str) -> Result<(), String> {
    let mut seat_ids = traverse(data.split("\n").map(seat_position))?;

    let max = seat_ids.iter().max().ok_or("No value found")?;
    println!("The max seat id is {max}");

    seat_ids.sort();

    let mut my_seat = None;

    for (index, seat_id) in seat_ids.iter().enumerate() {
        let next = seat_ids.get(index + 1);

        if util::option::contains(&next, &(seat_id + 2)) {
            my_seat = Some(seat_id + 1);
            break;
        }
    }

    let my_seat = my_seat.ok_or("Can't find my seat!")?;

    println!("My seat is {my_seat}");

    return Ok(());
}

fn location(instructions: Vec<Instruction>, min: &i32, max: &i32) -> i32 {
    let mut min = min.clone();
    let mut max = max.clone();

    for instruction in instructions {
        let discarding = ((max - min) / 2) + 1;

        match instruction {
            Instruction::Upper => min += discarding,
            Instruction::Lower => max -= discarding,
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_row_44() {
        let instructions = vec![
            Instruction::Lower,
            Instruction::Upper,
            Instruction::Lower,
            Instruction::Upper,
            Instruction::Upper,
            Instruction::Lower,
            Instruction::Lower,
        ];

        assert_eq!(location(instructions, &0, &127), 44);
    }
}

pub fn id(sequence: &str) {
    let (row, col) = sequence.split_at(7);
}
