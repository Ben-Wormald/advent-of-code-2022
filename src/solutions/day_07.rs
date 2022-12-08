use itertools::Itertools;
use std::{collections::HashMap, cmp::min};

const ROOT: &str = "ROOT";

const LIMIT: usize = 100_000;

const TOTAL: usize = 70_000_000;
const NEEDED: usize = 30_000_000;
const TARGET: usize = TOTAL - NEEDED;

trait Dirs {
    fn add_size(&mut self, size: usize, stack: &Vec<String>);
}

impl Dirs for HashMap<String, usize> {
    fn add_size(&mut self, size: usize, stack: &Vec<String>) {
        stack
            .iter()
            .fold("".to_string(), |path, dir| {
                let current_path = format!("{}/{}", path, dir);
    
                self.entry(current_path.clone())
                    .and_modify(|current_size| *current_size += size)
                    .or_insert(size);
    
                current_path
            });
    }
}

fn process(input: &str) -> HashMap<String, usize> {
    let mut stack = vec!(ROOT.to_string());

    let mut dirs: HashMap<String, usize> = HashMap::new();

    for line in input.lines().skip(1) {
        if line.starts_with("$ cd ") {
            let dir = line.replace("$ cd ", "");
            if dir == ".." {
                stack.pop();
            } else {
                stack.push(dir);
            }
        } else if line.starts_with("$ ls") {

        } else if line.starts_with("dir ") {

        } else {
            let (size, _name) = line.split_whitespace().collect_tuple().unwrap();
            let size = size.parse().unwrap();
            dirs.add_size(size, &stack);
        }
    }

    dirs
}

pub fn solve_part_one(input: &str) -> usize {
    let dirs = process(input);

    dirs.iter()
        .fold(0, |sum, (_dir, size)| {
            if size <= &LIMIT {
                sum + size
            } else {
                sum
            }
        })
}

pub fn solve(input: &str) -> usize {
    let dirs = process(input);

    let (_root, used) = dirs
        .iter()
        .find(|(dir, _size)|
            **dir == format!("/{}", ROOT)
        )
        .unwrap();

    let mut smallest: Option<usize> = None;

    dirs.iter()
        .for_each(|(_dir, size)| {
            if used - size <= TARGET {
                smallest = Some(smallest.map_or(
                    *size,
                    |current_smallest| min(*size, current_smallest),
                ));
            }
        });

    smallest.unwrap()
}
