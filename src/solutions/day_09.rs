use std::collections::HashSet;
use itertools::Itertools;

const KNOTS: usize = 10;

#[derive(Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn get_delta(&self) -> Pos {
        match self {
            Dir::Left => Pos { x: -1, y: 0 },
            Dir::Right => Pos { x: 1, y: 0 },
            Dir::Up => Pos { x: 0, y: 1 },
            Dir::Down => Pos { x: 0, y: -1 },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn follow(&mut self, head: &Pos) {
        let mut d = Pos::new();

        if head.x >= self.x + 2 {
            d.x = 1;

            if head.y > self.y {
                d.y = 1;
            } else if head.y < self.y {
                d.y = -1;
            }
        } else if head.x <= self.x - 2 {
            d.x = -1;

            if head.y > self.y {
                d.y = 1;
            } else if head.y < self.y {
                d.y = -1;
            }
        }

        if head.y >= self.y + 2 {
            d.y = 1;

            if head.x > self.x {
                d.x = 1;
            } else if head.x < self.x {
                d.x = -1;
            }
        } else if head.y <= self.y - 2 {
            d.y = -1;

            if head.x > self.x {
                d.x = 1;
            } else if head.x < self.x {
                d.x = -1;
            }
        }

        *self += d;
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, other: Pos) {
        *self = Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn process(input: &str) -> Vec<Dir> {
    input
        .lines()
        .flat_map(|line| {
            let (dir, n) = line.split_whitespace().collect_tuple().unwrap();

            let dir = match dir {
                "L" => Dir::Left,
                "R" => Dir::Right,
                "U" => Dir::Up,
                "D" => Dir::Down,
                _ => panic!("bad direction!"),
            };

            let n = n.parse().unwrap();

            vec![dir; n]
        })
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let motions = process(input);

    let mut head = Pos::new();
    let mut tail = Pos::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    visited.insert(tail.clone());

    motions
        .iter()
        .for_each(|motion| {
            let d = motion.get_delta();
            head += d;
            tail.follow(&head);
            visited.insert(tail.clone());
        });

    visited.len()
}

pub fn solve(input: &str) -> usize {
    let motions = process(input);

    let mut knots = vec![Pos::new(); KNOTS];
    let mut visited: HashSet<Pos> = HashSet::new();

    visited.insert(knots.last().unwrap().clone());

    motions
        .iter()
        .for_each(|motion| {
            let d = motion.get_delta();

            *knots.first_mut().unwrap() += d;

            for i in 1..KNOTS {
                let head = knots.get(i - 1).unwrap().clone();
                knots.get_mut(i).unwrap().follow(&head);
            }

            visited.insert(knots.last().unwrap().clone());
        });

    visited.len()
}
