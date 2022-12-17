use std::{ops::Add, cmp::max};

const COUNT: usize = 2022;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cell {
    x: isize,
    y: isize,
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, other: Cell) -> Cell {
        Cell {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Cell {
    fn move_cell(&self, dir: &Dir) -> Cell {
        match dir {
            Dir::Left => Cell {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Right => Cell {
                x: self.x + 1,
                y: self.y,
            },
            Dir::Down => Cell {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

#[derive(Clone, Copy)]
enum RockType {
    A,
    B,
    C,
    D,
    E,
}

struct Rock {
    rock_type: RockType,
    pos: Cell,
}

impl Rock {
    fn cells(&self) -> Vec<Cell> {
        match self.rock_type {
            RockType::A => vec!(
                Cell { x: 0, y: 0 },
                Cell { x: 1, y: 0 },
                Cell { x: 2, y: 0 },
                Cell { x: 3, y: 0 },
            ),
            RockType::B => vec!(
                Cell { x: 0, y: 1 },
                Cell { x: 1, y: 0 },
                Cell { x: 1, y: 1 },
                Cell { x: 1, y: 2 },
                Cell { x: 2, y: 1 },
            ),
            RockType::C => vec!(
                Cell { x: 0, y: 0 },
                Cell { x: 1, y: 0 },
                Cell { x: 2, y: 0 },
                Cell { x: 2, y: 1 },
                Cell { x: 2, y: 2 },
            ),
            RockType::D => vec!(
                Cell { x: 0, y: 0 },
                Cell { x: 0, y: 1 },
                Cell { x: 0, y: 2 },
                Cell { x: 0, y: 3 },
            ),
            RockType::E => vec!(
                Cell { x: 0, y: 0 },
                Cell { x: 0, y: 1 },
                Cell { x: 1, y: 0 },
                Cell { x: 1, y: 1 },
            ),
        }
    }

    fn width(&self) -> isize {
        match self.rock_type {
            RockType::A => 4,
            RockType::B => 3,
            RockType::C => 3,
            RockType::D => 1,
            RockType::E => 2,
        }
    }

    fn height(&self) -> isize {
        match self.rock_type {
            RockType::A => 1,
            RockType::B => 3,
            RockType::C => 3,
            RockType::D => 4,
            RockType::E => 2,
        }
    }
}

enum Dir {
    Left,
    Right,
    Down,
}

pub fn solve(input: &str) -> isize {
    let mut height = 0;
    let mut occupied = Vec::<Cell>::new();

    let jets = parse(input);
    let mut jet = 0;

    let rock_types = [RockType::A, RockType::B, RockType::C, RockType::D, RockType::E];
    let mut rock_type = 0;

    for _rock in 0..COUNT {
        let mut rock = Rock {
            rock_type: rock_types[rock_type],
            pos: Cell { x: 2, y: height + 4 },
        };

        rock_type += 1;
        if rock_type == rock_types.len() {
            rock_type = 0;
        }

        loop {
            let new_pos = rock.pos.move_cell(jets.get(jet).unwrap());

            jet += 1;
            if jet == jets.len() {
                jet = 0;
            }

            let within_walls = new_pos.x >= 0 && new_pos.x + rock.width() <= 7;
            let can_move = within_walls && !is_collision(&occupied, &new_pos, &rock);

            if can_move {
                rock.pos = new_pos;
            }

            let new_pos = rock.pos.move_cell(&Dir::Down);

            let is_floor = new_pos.y == 0;
            let can_move = !is_floor && !is_collision(&occupied, &new_pos, &rock);

            if can_move {
                rock.pos = new_pos;
            } else {
                settle(&mut occupied, &rock);
                height = max(height, rock.pos.y + rock.height() - 1);
                break;
            }
        }
    }
    
    height
}

fn parse(input: &str) -> Vec<Dir> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("bad input"),
        })
        .collect()
}

fn is_collision(occupied: &Vec<Cell>, pos: &Cell, rock: &Rock) -> bool {
    rock
        .cells()
        .iter()
        .any(|cell| {
            let cell = *cell + *pos;
            occupied.contains(&cell)
        })
}

fn settle(occupied: &mut Vec<Cell>, rock: &Rock) {
    rock
        .cells()
        .iter()
        .for_each(|cell| {
            let cell = *cell + rock.pos;
            occupied.push(cell);
        });
}
