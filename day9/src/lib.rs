use std::{collections::HashSet, usize};



fn has_matching_numbers(source: &[usize], target: usize) -> bool {
    let map: HashSet<&usize> = source.iter().collect();

    for item in source {
        if let Some (complementary) = target.checked_sub(*item) {
            if map.contains(&complementary) {
    
                if complementary == *item {
                    if source.iter().filter(|v| *v == item).count() != 1 {
                        return  true;
                    }
                }
                else {return true;}
            }
        }
    }
    return false;

}

/// skipping the first n values, returns the first value where there are no two values in the preceding value that add up to the number in the array
/// 
/// 
///
/// # Examples
///
/// ```
/// use day9::first_without_matching;
///
/// let source = &[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
/// let window = 5;
/// assert_eq!(first_without_matching(source, window), Some(&127));
/// ```
pub fn first_without_matching(source: &[usize],window:usize) -> Option<&usize> {
    source.windows(window + 1).filter_map( |v|{ 
            let (target, source) = v.split_last().unwrap();

            if has_matching_numbers(source, *target) {
                None
            } else {
                Some(target)
            }

        }).last()
}

/// finds the first continuous range of values that adds up to target
/// 
/// 
///
/// # Examples
///
/// ```
/// 
/// use day9::find_chunk_that_adds_to;
/// let source = &[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
/// let target = 127;
/// let result = find_chunk_that_adds_to(source, target);
/// assert!(result.is_some());
/// let result_vec: Vec<usize> = result.unwrap().to_vec();
/// assert_eq!(result_vec, vec![15, 25, 47, 40]);       
///  ```
pub fn find_chunk_that_adds_to(source:&[usize], target: usize) -> Option<&[usize]> {

    for i in 2..source.len() {
        if let Some(res) = source.windows(i).find(|w| w.iter().sum::<usize>() == target){
            return Some(res)
        }
    }

    return None
}

pub fn to_encryption_weakness(source: &[usize]) -> Option<usize> {
    let min = source.iter().min()?;
    let max = source.iter().max()?;

    Some (min + max)
}

#[cfg(test)]
mod test {

    mod has_matching_numbers {
        use crate::has_matching_numbers;


        #[test]
        fn without_matching_numbers() {
            let res = has_matching_numbers(&[12, 2, 15], 3);

            assert!(!(res))
        }

        #[test]
        fn with_matching_numbers() {
            let res = has_matching_numbers(&[12,14,13], 27);

            assert!(res)
        }

        #[test]
        fn with_half_value() {
            let res = has_matching_numbers(&[12,14,15], 30);
        }

        #[test]
        fn with_half_value_twice() {
            let res = has_matching_numbers(&[12,15,15], 30);
        }



    }
}