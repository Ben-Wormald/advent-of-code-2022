use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Coord>,
    x_lower: usize,
    x_upper: usize,
    y_upper: usize,
}

enum Fall {
    Stay,
    Move(Coord),
    Out,
}

impl Grid {
    fn new(paths: Vec<Vec<Coord>>, x_lower: usize, x_upper: usize, y_upper: usize) -> Grid {
        let mut grid = Grid {
            cells: vec!(),
            x_lower,
            x_upper,
            y_upper,
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
                            grid.cells.push(Coord { x, y })
                        }
                    }
                })
            );

        grid
    }

    fn is_cell(&self, coord: &Coord) -> bool {
        self.cells.iter().find(|cell| *cell == coord).is_some()
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
            } else if !self.is_cell(&option) {
                return Fall::Move(option);
            }
        }

        Fall::Stay
    }

    fn is_out(&self, coord: &Coord) -> bool {
        coord.x < self.x_lower || coord.x > self.x_upper || coord.y > self.y_upper
    }

    fn add_floor(&mut self) {
        self.x_lower = 0;
        self.x_upper = 1000;
        self.y_upper += 2;

        for x in self.x_lower..=self.x_upper {
            self.cells.push(Coord { x, y: self.y_upper });
        }
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

pub fn solve_part_one(input: &str) -> usize {
    let mut grid = process(input);

    let mut sand_count = 0;
    let mut done = false;

    while !done {
        let mut sand_coord = Coord { x: 500, y: 0 };

        loop {
            match grid.fall(&sand_coord) {
                Fall::Move(coord) => sand_coord = coord,
                Fall::Stay => {
                    grid.cells.push(sand_coord);
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

pub fn solve(input: &str) -> usize {
    let mut grid = process(input);

    grid.add_floor();

    let mut sand_count = 0;
    let mut done = false;
    let origin = Coord { x: 500, y: 0 };

    while !done {
        let mut sand_coord = origin.clone();

        loop {
            match grid.fall(&sand_coord) {
                Fall::Move(coord) => sand_coord = coord,
                Fall::Stay => {
                    grid.cells.push(sand_coord.clone());
                    sand_count += 1;

                    if sand_coord == origin {
                        done = true;
                    }
                    break;
                },
                Fall::Out => panic!("sand escaped! {:?}", sand_coord),
            }
        }
    }

    sand_count
}
