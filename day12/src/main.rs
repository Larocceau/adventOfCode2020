use day12::Bearing;
use day12::Boat;

fn main() {
    let data = day12::Command::parse_multiple(&util::load_data().unwrap()).unwrap();
    let mut boat = Boat {
        bearing: Bearing::East,
        x_pos: 0,
        y_pos: 0,
    };

    boat.handle_commands(&data);

    let distance = boat.x_pos.abs() + boat.y_pos.abs();

    println!("the boat travels {distance}")
}
