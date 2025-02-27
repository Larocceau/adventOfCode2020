use std::{env, fs, io::{Error, Read}};

pub fn load_data() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    let default_file_name = "input.txt".to_string();

    let file_name = args.get(1).unwrap_or(&default_file_name);

    let mut input = String::new();
    fs::File::open(file_name)?.read_to_string(&mut input)?;
    Ok(input)
}
