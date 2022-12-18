use itertools::Itertools;
use std::{collections::HashSet, ops::Add};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl Cube {
    fn neighbours(&self) -> Vec<Cube> {
        vec!(
            *self + Cube { x: -1, y: 0, z: 0 },
            *self + Cube { x: 1, y: 0, z: 0 },
            *self + Cube { x: 0, y: -1, z: 0 },
            *self + Cube { x: 0, y: 1, z: 0 },
            *self + Cube { x: 0, y: 0, z: -1 },
            *self + Cube { x: 0, y: 0, z: 1 },
        )
    }
}

impl Add for Cube {
    type Output = Cube;

    fn add(self, other: Cube) -> Cube {
        Cube {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn parse(input: &str) -> HashSet<Cube> {
    let mut cubes = HashSet::<Cube>::new();

    input
        .lines()
        .for_each(|line| {
            let (x, y, z) = line
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple().unwrap();

            cubes.insert(Cube { x, y, z });
        });

    cubes
}

pub fn solve_part_one(input: &str) -> usize {
    let cubes = parse(input);

    cubes
        .iter()
        .fold(0, |surfaces, cube| {
            let empty_neighbours = cube
                .neighbours()
                .iter()
                .filter(|neighbour| !cubes.contains(&neighbour))
                .count();

            surfaces + empty_neighbours
        })
}

pub fn solve(input: &str) -> usize {
    let cubes = parse(input);
    let outside_cubes = get_outside_cubes(&cubes);

    cubes
        .iter()
        .fold(0, |surfaces, cube| {
            let outside_neighbours = cube
                .neighbours()
                .iter()
                .filter(|neighbour|
                    outside_cubes.contains(&neighbour) && !cubes.contains(&neighbour)
                )
                .count();

            surfaces + outside_neighbours
        })
}

fn get_outside_cubes(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let mut outside = HashSet::<Cube>::new();
    let mut occupied = HashSet::<Cube>::new();

    let min_x = cubes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x - 1;
    let min_y = cubes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y - 1;
    let min_z = cubes.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap().z - 1;

    let max_x = cubes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 1;
    let max_y = cubes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 1;
    let max_z = cubes.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z + 1;

    outside.insert(Cube { x: min_x, y: min_y, z: min_z });

    let mut explored_len = 1;

    loop {
        for cube in outside.clone().iter() {
            for neighbour in cube.neighbours().iter() {
                let in_x = neighbour.x >= min_x && neighbour.x <= max_x;
                let in_y = neighbour.y >= min_y && neighbour.y <= max_y;
                let in_z = neighbour.z >= min_z && neighbour.z <= max_z;

                if in_x && in_y && in_z {
                    if cubes.contains(&neighbour) {
                        occupied.insert(*neighbour);
                    } else {
                        outside.insert(*neighbour);
                    }
                }
            }
        }

        if explored_len == outside.len() + occupied.len() {
            break
        }

        explored_len = outside.len() + occupied.len();
    }

    outside
}
