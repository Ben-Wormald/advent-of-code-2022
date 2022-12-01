fn process(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|elf| elf
            .split_whitespace()
            .map(|cals| cals
                .parse::<usize>()
                .unwrap()
            )
            .sum()
        )
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let elves = process(input);

    elves
        .into_iter()
        .reduce(|max, elf| std::cmp::max(max, elf))
        .unwrap()
}

pub fn solve(input: &str) -> usize {
    let mut elves = process(input);

    elves.sort_by(|a, b| b.cmp(a));
    
    elves
        .into_iter()
        .take(3)
        .sum()
}
