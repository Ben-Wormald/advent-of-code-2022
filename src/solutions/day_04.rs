use itertools::Itertools;

fn process(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .map(|pair| pair
            .split(',')
            .map(|elf| elf
                .split('-')
                .map(|section| section
                    .parse().unwrap()
                )
                .collect_tuple().unwrap()
            )
            .collect_tuple().unwrap()
        )
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    process(input)
        .iter()
        .filter(|(a, b)|
            contains(a, b)
        )
        .count()
}

fn contains(a: &(usize, usize), b: &(usize, usize)) -> bool {
    let a_contains_b = a.0 <= b.0 && a.1 >= b.1;
    let b_contains_a = b.0 <= a.0 && b.1 >= a.1;
    a_contains_b || b_contains_a
}

pub fn solve(input: &str) -> usize {
    process(input)
        .iter()
        .filter(|(a, b)|
            overlaps(a, b)
        )
        .count()
}

fn overlaps(a: &(usize, usize), b: &(usize, usize)) -> bool {
    let a_starts_in_b = a.0 >= b.0 && a.0 <= b.1;
    let a_ends_in_b = a.1 >= b.0 && a.0 <= b.1;
    a_starts_in_b || a_ends_in_b
}
