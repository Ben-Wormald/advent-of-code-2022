type Rucksack<'a> = (&'a str, &'a str);

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

fn process(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|rucksack| rucksack
            .split_at(rucksack.len() / 2)
        )
        .collect()
}

pub fn solve(input: &str) -> u32 {
    let rucksacks = process(input);

    rucksacks
        .into_iter()
        .fold(0, |sum, rucksack| {
            let (compartment_a, compartment_b) = rucksack;
            sum + compartment_a
                .chars()
                .find(|item| compartment_b
                    .contains(*item)
                )
                .unwrap()
                .to_value()
        })
}
