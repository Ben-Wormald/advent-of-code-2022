use std::collections::VecDeque;

const START: char = 'S';
const END: char = 'E';

type Coord = (usize, usize);

#[derive(Debug)]
struct Square {
    height: usize,
    visited: bool,
    parent: Option<Coord>,
    is_start: bool,
    is_end: bool,
}

trait ToValue {
    fn to_value(&self) -> u32;
}

impl ToValue for char {
    fn to_value(&self) -> u32 {
        if self.is_lowercase() {
            self.to_digit(36).unwrap() - 9
        } else {
            self.to_digit(36).unwrap() - 9 + 26
        }
    }
}

trait Grid {
    fn get_square(&self, square: Coord) -> &Square;
    fn get_square_mut(&mut self, square: Coord) -> &mut Square;
    fn find(&self, target: char) -> Coord;
    fn get_adjacent(&self, square: Coord, is_reverse: bool) -> Vec<Coord>;
}

impl Grid for Vec<Vec<Square>> {
    fn get_square(&self, coord: Coord) -> &Square {
        self.get(coord.0).unwrap().get(coord.1).unwrap()
    }

    fn get_square_mut(&mut self, coord: Coord) -> &mut Square {
        self.get_mut(coord.0).unwrap().get_mut(coord.1).unwrap()
    }

    fn find(&self, target: char) -> Coord {
        let (mut row_idx, mut col_idx) = (0, 0);

        for (current_row, row) in self.iter().enumerate() {
            for (current_col, square) in row.iter().enumerate() {
                let is_start = target == START && square.is_start;
                let is_end = target == END && square.is_end;

                if is_start || is_end {
                    (row_idx, col_idx) = (current_row, current_col);
                }
            }
        }

        (row_idx, col_idx)
    }

    fn get_adjacent(&self, coord: Coord, is_reverse: bool) -> Vec<Coord> {
        let mut adjacent = vec!();
        let current_square = self.get_square(coord);
    
        let coord = (coord.0 as isize, coord.1 as isize);
    
        let potentials: [(isize, isize); 4] = [
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
        ];
    
        for potential in potentials {
            let valid_row = potential.0 >= 0 && potential.0 < self.len() as isize;
            let valid_col = potential.1 >= 0
                && potential.1 < self.get(coord.0 as usize).unwrap().len() as isize;
    
            if valid_row && valid_col {
                let adjacent_square = self.get_square((potential.0 as usize, potential.1 as usize));
    
                let is_unvisited = !adjacent_square.visited;

                let is_valid_move = if !is_reverse {
                    adjacent_square.height <= current_square.height + 1
                } else {
                    current_square.height <= adjacent_square.height + 1
                };
                
    
                if is_unvisited && is_valid_move {
                    adjacent.push((potential.0 as usize, potential.1 as usize));
                }
            }
        }
    
        adjacent
    }
}

fn process(input: &str) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|char| {
                let height = match char {
                    START => 1,
                    END => 26,
                    _ => char.to_value() as usize,
                };

                Square {
                    height,
                    visited: false,
                    parent: None,
                    is_start: char == START,
                    is_end: char == END,
                }
            })
            .collect()
        )
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let mut squares = process(input);

    let start = squares.find(START);
    let mut queue: VecDeque<Coord> = VecDeque::new();

    squares.get_square_mut(start).visited = true;
    queue.push_back(start);

    while let Some(coord) = queue.pop_front() {
        if squares.get_square(coord).is_end {
            queue.clear();
        } else {
            for adjacent in squares.get_adjacent(coord, false).iter() {
                let adjacent_square = squares.get_square_mut(*adjacent);

                adjacent_square.visited = true;
                adjacent_square.parent = Some(coord);

                queue.push_back(*adjacent);
            }
        }
    }

    let mut steps = 0;
    let mut found = false;
    let mut current = squares.find(END);

    while !found {
        let current_square = squares.get_square(current);

        if current_square.is_start {
            found = true;
        } else {
            current = current_square.parent.unwrap();
            steps += 1;
        }
    }

    steps
}

pub fn solve(input: &str) -> usize {
    let mut squares = process(input);

    let start = squares.find(END);
    let mut end_coord = (0, 0);
    let mut queue: VecDeque<Coord> = VecDeque::new();

    squares.get_square_mut(start).visited = true;
    queue.push_back(start);

    while let Some(coord) = queue.pop_front() {
        if squares.get_square(coord).height == 1 {
            end_coord = coord;
            queue.clear();
        } else {
            for adjacent in squares.get_adjacent(coord, true).iter() {
                let adjacent_square = squares.get_square_mut(*adjacent);

                adjacent_square.visited = true;
                adjacent_square.parent = Some(coord);

                queue.push_back(*adjacent);
            }
        }
    }

    let mut steps = 0;
    let mut found = false;
    let mut current = end_coord;

    while !found {
        let current_square = squares.get_square(current);

        if current_square.is_end {
            found = true;
        } else {
            current = current_square.parent.unwrap();
            steps += 1;
        }
    }

    steps
}
