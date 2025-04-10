use std::{os::windows::process::ExitStatusExt, usize};

#[derive(Clone, Copy, Debug)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

impl Bearing {
    fn left(&self) -> Bearing {
        match self {
            Bearing::North => Bearing::West,
            Bearing::East => Bearing::North,
            Bearing::South => Bearing::East,
            Bearing::West => Bearing::South,
        }
    }

    fn right(&self) -> Bearing {
        self.left().left().left()
    }
}

#[derive(Debug)]
pub struct Boat {
    pub bearing: Bearing,
    pub x_pos: i32,
    pub y_pos: i32,
}

pub enum Direction {
    Left,
    Right,
}

pub enum Command {
    Turn(Direction, u32),
    Forward(u32),
    MoveToward(Bearing, u32),
}

impl Command {
    pub fn parse_multiple(data: &str) -> Option<Vec<Command>> {
        util::option::traverse(data.split_whitespace().map(Command::parse_single))
    }

    pub fn parse_single(data: &str) -> Option<Command> {
        let (first, rest) = data.split_at(1);
        let parameter = rest.parse::<u32>().ok()?;

        match first {
            "L" => Some(Self::Turn(Direction::Left, parameter)),
            "R" => Some(Self::Turn(Direction::Right, parameter)),
            "N" => Some(Self::MoveToward(Bearing::North, parameter)),
            "E" => Some(Self::MoveToward(Bearing::East, parameter)),
            "S" => Some(Self::MoveToward(Bearing::South, parameter)),
            "W" => Some(Self::MoveToward(Bearing::West, parameter)),
            "F" => Some(Self::Forward(parameter)),
            _ => None,
        }
    }
}

impl Boat {
    /// # Examples
    ///
    /// ```
    /// use day12::Boat;
    /// use day12::Bearing;
    /// use day12::Command;
    /// use day12::Direction;
    ///
    /// let mut boat = Boat {
    ///     bearing: Bearing::East,
    ///     x_pos: 0,
    ///     y_pos: 0,
    /// };
    ///
    /// boat.handle_commands(&[
    ///     Command::Forward(10),
    ///     Command::MoveToward(Bearing::North, 3),
    ///     Command::Forward(7),
    ///     Command::Turn(Direction::Right, 90),
    ///     Command::Forward(11),
    /// ]);
    ///
    /// assert_eq!(boat.x_pos, 17);
    /// assert_eq!(boat.y_pos, -8)
    /// ```
    pub fn handle_commands(&mut self, commands: &[Command]) {
        for command in commands {
            self.handle_command(command);
        }
    }

    pub fn handle_command(&mut self, command: &Command) {
        match command {
            Command::Turn(direction, degrees) => {
                let mut degrees = *degrees;
                while degrees > 0 {
                    self.bearing = match direction {
                        Direction::Left => self.bearing.left(),
                        Direction::Right => self.bearing.right(),
                    };
                    degrees -= 90;
                }
            }
            Command::MoveToward(direction, distance) => {
                let distance = *distance as i32;
                match direction {
                    Bearing::North => self.y_pos += distance,
                    Bearing::East => self.x_pos += distance,
                    Bearing::South => self.y_pos -= distance,
                    Bearing::West => self.x_pos -= distance,
                }
            }
            Command::Forward(distance) => {
                self.handle_command(&Command::MoveToward(self.bearing, *distance));
            }
        }
    }
}
