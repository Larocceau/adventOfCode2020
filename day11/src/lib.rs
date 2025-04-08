use std::{cell::RefCell, rc::Rc};
pub trait GameElement {
    fn pre_tick(&mut self) -> ();
    fn tick(&mut self) -> ();
    fn is_frozen(&self) -> bool;
}

type Board = Vec<Rc<RefCell<Chair>>>;

pub fn print_board(board: &Board) {
    for row in 0..10 {
        for col in 0..10 {
            let chair = board
                .iter()
                .find(|c| c.borrow().row == row && c.borrow().column == col);

            match chair {
                Some(c) => {
                    if c.borrow().occupied {
                        print!("#");
                    } else {
                        print!("L");
                    }
                }
                None => print!("."),
            }
        }
        println!("");
    }
}

///
/// # Examples
///
/// ```
/// use day11::solve_part_1;
/// use day11::parse;
///
// / let mut board = parse(
// / "L.LL.LL.LL
// /LLLLLLL.LL
// /L.L.L..L..
// /LLLL.LL.LL
// /L.LL.LL.LL
// /L.LLLLL.LL
// /..L.L.....
// /LLLLLLLLLL
// /L.LLLLLL.L
// /L.LLLLL.LL" );
// / let res = solve_part_1(&mut board);
// / assert_eq!(res, 37 );
/// ```
pub fn solve_part_1(board: &mut Board) -> usize {
    print_board(board);

    while !(board.is_frozen()) {
        board.pre_tick();
        board.tick();
        print_board(board);
    }

    board.iter().filter(|v| v.borrow().occupied).count()
}

impl GameElement for Board {
    fn is_frozen(&self) -> bool {
        self.iter().all(|v| v.borrow().is_frozen())
    }

    fn pre_tick(&mut self) -> () {
        for chair in self.iter() {
            chair.borrow_mut().pre_tick();
        }
    }

    fn tick(&mut self) -> () {
        for chair in self.iter() {
            chair.borrow_mut().tick();
        }
    }
}

pub fn parse(data: &str) -> Board {
    let rows = data.split_ascii_whitespace();

    let data: Board = rows
        .enumerate()
        .flat_map(|(row, content)| {
            content.chars().enumerate().filter_map(move |(column, v)| {
                match v {
                    'L' => Some(false),
                    '#' => Some(true),
                    _ => None,
                }
                .map(|occupied| Chair {
                    occupied,
                    will_be_occupied: !occupied,
                    row,
                    column,
                    neigbours: Vec::new(),
                })
                .map(|v| Rc::new(RefCell::new(v)))
            })
        })
        .collect();

    link(&data);

    data
}

fn link(chairs: &Board) {
    for chair in chairs {
        chair.borrow_mut().neigbours = chairs
            .iter()
            .filter(|v| v.borrow().is_neighbor(&chair.borrow()))
            .map(Rc::clone)
            .collect();
    }
}

pub struct Chair {
    neigbours: Vec<Rc<RefCell<Chair>>>,
    occupied: bool,
    will_be_occupied: bool,
    row: usize,
    column: usize,
}

impl Chair {
    fn is_neighbor(&self, other: &Self) -> bool {
        // Calculate the distance between chairs
        let row_diff = if self.row >= other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };

        let col_diff = if self.column >= other.column {
            self.column - other.column
        } else {
            other.column - self.column
        };

        // Chairs are neighbors if they are at most 1 position away in any direction
        // This includes diagonally adjacent chairs
        row_diff <= 1 && col_diff <= 1 && (row_diff > 0 || col_diff > 0)
    }

    fn will_be_occupied(&self) -> bool {
        let occupied_neighbors = self
            .neigbours
            .iter()
            .filter(|&c| c.borrow().occupied)
            .count();

        match self.occupied {
            false => {
                if occupied_neighbors == 0 {
                    true
                } else {
                    false
                }
            }
            true => {
                if occupied_neighbors < 4 {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl GameElement for Chair {
    fn is_frozen(&self) -> bool {
        self.occupied == self.will_be_occupied()
    }

    fn pre_tick(&mut self) {
        self.will_be_occupied = self.will_be_occupied()
    }

    fn tick(&mut self) {
        self.occupied = self.will_be_occupied
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Chair, link};

    #[test]
    fn leaves_when_busy() {
        let mut chairs = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                chairs.push(Rc::new(RefCell::new(Chair {
                    row: i,
                    column: j,
                    neigbours: Vec::new(),
                    occupied: j == 2,
                    will_be_occupied: true,
                })));
            }
        }

        link(&chairs);

        let middle_left_will_be_occupied = chairs
            .iter()
            .find(|c| {
                let c = c.borrow();

                c.column == 0 && c.row == 1
            })
            .unwrap()
            .borrow()
            .will_be_occupied();

        let middle_right_will_stay = chairs
            .iter()
            .find(|c| {
                let c = c.borrow();

                c.column == 2 && c.row == 1
            })
            .unwrap()
            .borrow()
            .will_be_occupied();

        assert_eq!(middle_left_will_be_occupied, true);
        assert_eq!(middle_right_will_stay, true);
    }
}
