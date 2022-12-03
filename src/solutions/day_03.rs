use itertools::Itertools;

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

fn process(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|rucksack|
            rucksack.split_at(rucksack.len() / 2)
        )
        .collect()
}

pub fn solve_part_one(input: &str) -> u32 {
    process(input)
        .into_iter()
        .fold(0, |sum, (compartment_a, compartment_b)| {
            sum + compartment_a
                .chars()
                .find(|item|
                    compartment_b.contains(*item)
                )
                .unwrap()
                .to_value()
        })
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .fold(0, |sum, (backpack_a, backpack_b, backpack_c)| {
            sum + backpack_a
                .chars()
                .find(|item|
                    backpack_b.contains(*item) && backpack_c.contains(*item)
                )
                .unwrap()
                .to_value()
        })
}
