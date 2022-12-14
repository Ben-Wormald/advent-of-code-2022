use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    x_offest: usize,
}

enum Fall {
    Stay,
    Move(Coord),
    Out,
}

impl Grid {
    fn new(paths: Vec<Vec<Coord>>, x_lower: usize, x_upper: usize, y_upper: usize) -> Grid {
        let cells = vec![vec![Cell::Air; y_upper + 1].clone(); x_upper - x_lower + 1];

        let mut grid = Grid {
            cells,
            x_offest: x_lower,
        };

        paths
            .iter()
            .for_each(|path| path
                .windows(2)
                .for_each(|window| {
                    let (coord_a, coord_b) = window.iter().collect_tuple().unwrap();

                    let x_range = if coord_b.x >= coord_a.x {
                        coord_a.x..=coord_b.x
                    } else {
                        coord_b.x..=coord_a.x
                    };
                    let y_range = if coord_b.y >= coord_a.y {
                        coord_a.y..=coord_b.y
                    } else {
                        coord_b.y..=coord_a.y
                    };

                    for x in x_range {
                        for y in y_range.clone() {
                            grid.set_cell(&Coord { x, y }, Cell::Rock);
                        }
                    }
                })
            );

        grid
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        let x = coord.x - self.x_offest;
        self.cells
            .get(x).expect(&format!("bad x {}", x))
            .get(coord.y).expect(&format!("bad y {}", coord.y))
    }

    fn get_cell_mut(&mut self, coord: &Coord) -> &mut Cell {
        let x = coord.x - self.x_offest;
        self.cells
            .get_mut(x).expect(&format!("bad x {}", x))
            .get_mut(coord.y).expect(&format!("bad y {}", coord.y))
    }

    fn set_cell(&mut self, coord: &Coord, cell: Cell) {
        *self.get_cell_mut(coord) = cell;
    }

    fn fall(&self, coord: &Coord) -> Fall {
        let options = [
            Coord { x: coord.x, y: coord.y + 1 },
            Coord { x: coord.x - 1, y: coord.y + 1 },
            Coord { x: coord.x + 1, y: coord.y + 1 }
        ];

        for option in options {
            if self.is_out(&option) {
                return Fall::Out;
            } else if *self.get_cell(&option) == Cell::Air {
                return Fall::Move(option);
            }
        }

        Fall::Stay
    }

    fn is_out(&self, coord: &Coord) -> bool {
        let x = coord.x as isize - self.x_offest as isize;

        let is_x_out = x < 0 || x > self.cells.len() as isize;
        let is_y_out = coord.y >= self.cells.first().unwrap().len();

        is_x_out || is_y_out
    }
}

fn process(input: &str) -> Grid {
    let mut x_lower = usize::MAX;
    let mut x_upper = 0;
    let mut y_upper = 0;

    let paths: Vec<Vec<Coord>> = input
        .lines()
        .map(|line| line
            .split(" -> ")
            .map(|coord| {
                let (x, y) = coord
                    .split(",")
                    .map(|i| i.parse().unwrap())
                    .collect_tuple().unwrap();

                if x < x_lower { x_lower = x }
                if x > x_upper { x_upper = x }
                if y > y_upper { y_upper = y }

                Coord { x, y }
            })
            .collect()
        )
        .collect();

    let grid = Grid::new(paths, x_lower, x_upper, y_upper);

    grid
}

pub fn solve(input: &str) -> usize {
    let mut grid = process(input);

    let mut sand_count = 0;
    let mut done = false;

    while !done {
        let mut sand_coord = Coord { x: 500, y: 0 };

        loop {
            match grid.fall(&sand_coord) {
                Fall::Move(coord) => sand_coord = coord,
                Fall::Stay => {
                    grid.set_cell(&sand_coord, Cell::Sand);
                    sand_count += 1;
                    break;
                },
                Fall::Out => {
                    done = true;
                    break;
                }
            }
        }
    }

    sand_count
}
