use itertools::Itertools;

const START_OF_PACKET: usize = 4;
const START_OF_MESSAGE: usize = 14;

trait IsMarker {
    fn is_marker(&self) -> bool;
}

impl IsMarker for &&[char] {
    fn is_marker(&self) -> bool {
        self.len() == self.iter().unique().count()
    }
}

fn find_marker(input: &str, marker_len: usize) -> usize {
    input
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(marker_len)
        .find_position(|window| window.is_marker())
        .expect("no marker found!")
        .0 + marker_len
}

pub fn solve_part_one(input: &str) -> usize {
    find_marker(input, START_OF_PACKET)
}

pub fn solve(input: &str) -> usize {
    find_marker(input, START_OF_MESSAGE)
}
