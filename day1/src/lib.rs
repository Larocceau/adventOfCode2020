
pub fn solve_part_2(data: &Vec<i32>, target: &i32) {
    let result = get_magic_triplet(data, target).expect("Target not found!");

    println!(
        "The numbers that combine to {target} are {}, {} and {}; their product is {}",
        result.0,
        result.1,
        result.2,
        result.0 * result.1 * result.2
    );
}

pub fn solve_part_1(data: &Vec<i32>, target: &i32) {
    let result = get_magic_pair(&data, &target).expect("Target not found!");

    println!(
        "The numbers that combine to {target} are {} and {}; their product is {}",
        result.0,
        result.1,
        result.0 * result.1
    );
}

fn get_magic_pair<'a>(data: &'a Vec<i32>, target_value: &i32) -> Option<(&'a i32, &'a i32)> {
    let mut pair: Option<(&i32, &i32)> = None;

    'outer: for (i, value) in data.iter().enumerate() {
        for other_value in &data[i..] {
            if (value + other_value) == *target_value {
                pair = Some((value, other_value));
                break 'outer;
            }
        }
    }
    pair
}

fn get_magic_triplet<'a>(
    data: &'a Vec<i32>,
    target_value: &i32,
) -> Option<(&'a i32, &'a i32, &'a i32)> {
    let mut triplet: Option<(&i32, &i32, &i32)> = None;

    'outer: for (i, value) in data.iter().enumerate() {
        for (j, inner_value) in data[i..].iter().enumerate() {
            if (value + inner_value) <= *target_value {
                for inner_most in &data[j..] {
                    if (value + inner_value + inner_most) == *target_value {
                        triplet = Some((value, inner_value, inner_most));
                        break 'outer;
                    }
                }
            }
        }
    }
    return triplet;
}
