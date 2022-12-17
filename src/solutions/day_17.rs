use std::{ops::Add, cmp::max, collections::{HashSet, HashMap}};

const COUNT_ONE: usize = 2022;
const COUNT: usize = 1000000000000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
    fn moved(&self, dir: &Dir) -> Cell {
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

type Permutation = (String, usize, usize);

trait CellSet {
    fn to_string(&self, height: isize) -> String;
}

impl CellSet for HashSet<Cell> {
    fn to_string(&self, height: isize) -> String {
        let mut strings = self
            .iter()
            .map(|cell| format!("{},{}", cell.x, cell.y - height))
            .collect::<Vec<String>>();

        strings.sort();

        strings.join(",")
    }
}

pub fn solve(input: &str) -> isize {
    let mut height = 0;
    let mut occupied = HashSet::<Cell>::new();

    let rock_types = [RockType::A, RockType::B, RockType::C, RockType::D, RockType::E];
    let mut rock_type = 0;

    let jets = parse(input);
    let mut jet = 0;

    let target_count = COUNT;

    let mut permutations = HashMap::<Permutation, (usize, isize)>::new();
    let mut cycle_found = false;
    let mut cycle_start_height = 0;
    let mut cycle_height = 0;
    let mut cycle_start = 0;
    let mut cycle_len = 0;
    let mut remaining_count = 0;

    for count in 0..target_count {
        let mut rock = Rock {
            rock_type: rock_types[rock_type],
            pos: Cell { x: 2, y: height + 4 },
        };

        rock_type += 1;
        if rock_type == rock_types.len() {
            rock_type = 0;
        }

        loop {
            let new_pos = rock.pos.moved(jets.get(jet).unwrap());

            jet += 1;
            if jet == jets.len() {
                jet = 0;
            }

            let within_walls = new_pos.x >= 0 && new_pos.x + rock.width() <= 7;
            let can_move = within_walls && !is_collision(&occupied, &new_pos, &rock);

            if can_move {
                rock.pos = new_pos;
            }

            let new_pos = rock.pos.moved(&Dir::Down);

            let is_floor = new_pos.y == 0;
            let can_move = !is_floor && !is_collision(&occupied, &new_pos, &rock);

            if can_move {
                rock.pos = new_pos;
            } else {
                settle(&mut occupied, &rock);
                height = max(height, rock.pos.y + rock.height() - 1);
                occupied = remove_out_of_reach(occupied, height);
                
                if !cycle_found {
                    let permutation = (occupied.to_string(height), rock_type, jet);

                    if let Some(prev_permutation) = permutations.get(&permutation) {
                        cycle_found = true;
    
                        cycle_start = prev_permutation.0;
                        cycle_len = count - prev_permutation.0;
    
                        cycle_start_height = prev_permutation.1;
                        cycle_height = height - prev_permutation.1;
    
                        remaining_count = (target_count - cycle_start) % cycle_len;
                    } else {
                        permutations.insert(permutation, (count, height));
                    }
                }

                break;
            }
        }

        if cycle_found {
            remaining_count -= 1;
            if remaining_count == 0 {
                break;
            }
        }
    }

    if cycle_found {
        let n_cycles = ((target_count - cycle_start) / cycle_len) as isize;
        let remaining_height = height - cycle_start_height - cycle_height;

        cycle_start_height + n_cycles * cycle_height + remaining_height
    } else {
        height
    }
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

fn is_collision(occupied: &HashSet<Cell>, pos: &Cell, rock: &Rock) -> bool {
    rock
        .cells()
        .iter()
        .any(|cell| {
            let cell = *cell + *pos;
            occupied.contains(&cell)
        })
}

fn settle(occupied: &mut HashSet<Cell>, rock: &Rock) {
    rock
        .cells()
        .iter()
        .for_each(|cell| {
            let cell = *cell + rock.pos;
            occupied.insert(cell);
        });
}

fn remove_out_of_reach(occupied: HashSet<Cell>, height: isize) -> HashSet<Cell> {
    let mut empty = HashSet::<Cell>::new();
    let mut occupied_edge = HashSet::<Cell>::new();
    
    empty.insert(Cell { x: 0, y: height + 1 });
    let mut explored_count = 1;
    let mut done = false;

    while !done {
        for cell in empty.clone().iter() {
            let mut neighbours = Vec::<Cell>::new();

            if cell.x > 0 {
                neighbours.push(cell.moved(&Dir::Left));
            }
            if cell.x < 6 {
                neighbours.push(cell.moved(&Dir::Right));
            }
            if cell.y > 1 {
                neighbours.push(cell.moved(&Dir::Down));
            }

            for neighbour in neighbours.iter() {
                if occupied.contains(neighbour) {
                    occupied_edge.insert(*neighbour);
                } else {
                    empty.insert(*neighbour);
                }
            }
        }

        if explored_count == empty.len() + occupied_edge.len() {
            done = true;
        }
        explored_count = empty.len() + occupied_edge.len();
    }

    occupied_edge
}
