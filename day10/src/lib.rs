use std::{cell::RefCell, collections::HashMap, rc::Rc};


fn jolt_differences(chargers: &[usize]) -> Vec<usize>{
    let mut data = chargers.to_vec();
    data.sort();

    let first = data[0];
    let mut res: Vec::<usize> = data.windows(2).map(|v| v[1] - v[0]).collect();

    res.push(first);
    res.push(3);
    res
}



/// # Examples
///
/// ```
/// use day10::solve_part_2;
/// let data = &[28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,];
/// let res = solve_part_2(data);
/// assert_eq!(res, 19208);
/// ```
pub fn solve_part_2(chargers: &[usize]) -> usize {
    no_of_routes(chargers, &0, &(chargers.iter().max().unwrap() + 3))
}



fn no_of_routes(chargers: &[usize],start_value: &usize, target_value: &usize) -> usize {
    let memo = RefCell::new(HashMap::new());

    no_of_routes_memo(chargers, start_value, target_value, &memo)
}


fn no_of_routes_memo(chargers: &[usize], start_value: &usize, target_value: &usize, memo: &RefCell<HashMap<(Vec<usize>, usize, usize), usize>>) -> usize {
    let key = (chargers.to_vec(), *start_value, *target_value);
    if let Some(res) = memo.borrow().get(&key){
        *res
    } else {
        let next: usize= 
        chargers.iter()
        .filter(|v| (**v - start_value) <= 3)
        .map(|v|  {
            
            let chargers: Vec<usize> = chargers.iter().filter(|i| *i > v).cloned().collect();
    
            no_of_routes_memo(&chargers, v, target_value, memo)
    
        }
        
        ).sum();
        let can_finish = target_value - start_value <= 3;
    
        let res = next + (if can_finish {1} else {0});

        memo.borrow_mut().insert(key, res);
        res

    }
    
}
/// # Examples
///
/// ```
/// use day10::solve_part_1;
/// let data = &[28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,];
/// let res = solve_part_1(data);
/// assert_eq!(res, 220)
/// ```
pub fn solve_part_1(data: &[usize]) -> u32 {

    let jolt_differences = jolt_differences(data);
    let frequencies = util::count_by(&jolt_differences, |&v| v);

    frequencies[&1] * frequencies[&3]
}
