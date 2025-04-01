
fn jolt_differences(chargers: &[u32]) -> Vec<u32>{
    let mut data = chargers.to_vec();
    data.sort();

    let first = data[0];
    let mut res: Vec::<u32> = data.windows(2).map(|v| v[1] - v[0]).collect();

    res.push(first);
    res.push(3);
    res
}


/// # Examples
///
/// ```
/// use day10::solve_part_1;
/// let data = &[28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,];
/// let res = solve_part_1(data);
/// assert_eq!(res, 220)
/// ```
pub fn solve_part_1(data: &[u32]) -> u32 {

    let jolt_differences = jolt_differences(data);
    let frequencies = util::count_by(&jolt_differences, |&v| v);

    frequencies[&1] * frequencies[&3]
}
