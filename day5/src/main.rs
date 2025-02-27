use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = util::load_data()?;

    day5::solve(&data).map_err(|e| e.into())
}
