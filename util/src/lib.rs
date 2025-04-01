use std::{
    collections::HashMap, env, fs, hash::Hash, io::{Error, Read}
};

pub fn load_data() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    let default_file_name = "input.txt".to_string();

    let file_name = args.get(1).unwrap_or(&default_file_name);

    let mut input = String::new();
    fs::File::open(file_name)?.read_to_string(&mut input)?;
    Ok(input)
}


/// Returns a hashmap with every item, and the number of occurrences
/// 
/// # Examples
///
/// ```
/// use std::convert::identity;
/// use util::count_by;
/// let res = count_by(&["apples", "apples", "pears"], |&v| v     );
/// let apple_count = res.get(&"apples").unwrap();
/// let pear_count = res.get(&"pears").unwrap();
/// assert_eq!(*apple_count, 2);
/// assert_eq!(*pear_count, 1);
/// ```
pub fn count_by<'a, A, B> (data: &'a [A], f: impl Fn(&A) -> B) -> HashMap<B, u32>
    where B: Hash + Eq {
        let mut counts = HashMap::<B, u32>::new();

        for item in data {
            let key = f(item);
            if let Some(count) = counts.get_mut(&key) {
                *count += 1;
            } else {
                counts.insert(key, 1);
            }
        }

        counts
}


pub mod option {

    pub fn contains<T: PartialEq>(option: &Option<&T>, expected: &T) -> bool {
        match option {
            Some(v) => {
                if **v == *expected {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}


