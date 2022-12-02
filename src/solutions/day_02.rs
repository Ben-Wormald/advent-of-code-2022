enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_value(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

trait ToShape {
    fn to_shape(&self) -> Shape;
}

impl ToShape for char {
    fn to_shape(&self) -> Shape {
        match self {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("bad char")
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let rounds = process_part_one(input);

    rounds
        .iter()
        .fold(0, |score, (opponent, us)| {
            score + us.get_value() + get_outcome(opponent, us)
        })
}

fn process_part_one(input: &str) -> Vec<(Shape, Shape)> {
    input
        .lines()
        .map(|line| (
            line.chars().nth(0).unwrap().to_shape(),
            line.chars().nth(2).unwrap().to_shape(),
        ))
        .collect()
}

fn get_outcome(opponent: &Shape, us: &Shape) -> usize {
    match opponent {
        Shape::Rock => match us {
            Shape::Rock => 3,
            Shape::Paper => 6,
            Shape::Scissors => 0,
        },
        Shape::Paper => match us {
            Shape::Rock => 0,
            Shape::Paper => 3,
            Shape::Scissors => 6,
        },
        Shape::Scissors => match us {
            Shape::Rock => 6,
            Shape::Paper => 0,
            Shape::Scissors => 3,
        },
    }
}


enum Outcome {
    Lose,
    Draw,
    Win,
}

trait ToOutcome {
    fn to_outcome(&self) -> Outcome;
}

impl ToOutcome for char {
    fn to_outcome(&self) -> Outcome {
        match self {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("bad char")
        }
    }
}

pub fn solve(input: &str) -> usize {
    let rounds = process(input);

    rounds
        .iter()
        .fold(0, |score, (opponent, outcome)| {
            let us = get_shape(opponent, outcome);
            score + us.get_value() + get_outcome(opponent, &us)
        })
}

fn process(input: &str) -> Vec<(Shape, Outcome)> {
    input
        .lines()
        .map(|line| (
            line.chars().nth(0).unwrap().to_shape(),
            line.chars().nth(2).unwrap().to_outcome(),
        ))
        .collect()
}

fn get_shape(opponent: &Shape, outcome: &Outcome) -> Shape {
    match opponent {
        Shape::Rock => match outcome {
            Outcome::Lose => Shape::Scissors,
            Outcome::Draw => Shape::Rock,
            Outcome::Win => Shape::Paper,
        },
        Shape::Paper => match outcome {
            Outcome::Lose => Shape::Rock,
            Outcome::Draw => Shape::Paper,
            Outcome::Win => Shape::Scissors,
        },
        Shape::Scissors => match outcome {
            Outcome::Lose => Shape::Paper,
            Outcome::Draw => Shape::Scissors,
            Outcome::Win => Shape::Rock,
        },
    }
}
