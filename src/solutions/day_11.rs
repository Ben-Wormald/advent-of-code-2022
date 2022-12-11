use itertools::Itertools;

enum Operand {
    Num(usize),
    Old,
}

enum Op {
    Add(Operand),
    Multiply(Operand),
}

struct Monkey {
    items: Vec<usize>,
    op: Op,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

const LABELS: [&str; 5] = [
    "  Starting items: ",
    "  Operation: new = old ",
    "  Test: divisible by ",
    "    If true: throw to monkey ",
    "    If false: throw to monkey ",
];

const ROUNDS: usize = 20;
const ROUNDS_TWO: usize = 10_000;

fn get_items(line: &str) -> Vec<usize> {
    line.split(", ").map(|item| item.parse().unwrap()).collect()
}

fn get_op(line: &str) -> Op {
    let (op, operand) = line.split_whitespace().collect_tuple().unwrap();

    let operand = match operand {
        "old" => Operand::Old,
        _ => Operand::Num(operand.parse().unwrap()),
    };

    match op {
        "+" => Op::Add(operand),
        "*" => Op::Multiply(operand),
        _ => panic!("bad operator!"),
    }
}

fn inspect(item: usize, op: &Op) -> usize {
    match op {
        Op::Add(operand) => item + match operand {
            Operand::Num(num) => *num,
            Operand::Old => item,
        },
        Op::Multiply(operand) => item * match operand {
            Operand::Num(num) => *num,
            Operand::Old => item,
        },
    }
}

fn process(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut monkey = monkey.to_string();
            for label in LABELS {
                monkey = monkey.replace(label, "");
            }

            let lines: Vec<&str> = monkey.lines().collect();

            Monkey {
                items: get_items(lines[1]),
                op: get_op(lines[2]),
                test: lines[3].parse().unwrap(),
                if_true: lines[4].parse().unwrap(),
                if_false: lines[5].parse().unwrap(),
                inspections: 0,
            }
        })
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let mut monkeys = process(input);

    for _round in 0..ROUNDS {
        for m in 0..monkeys.len() {
            let monkey = monkeys.get(m).unwrap();

            let (mut items_true, mut items_false): (Vec<usize>, Vec<usize>) = monkey.items
                .iter()
                .map(|item|
                    inspect(*item, &monkey.op) / 3
                )
                .partition(|item|
                    item % monkey.test == 0
                );

            let (if_true, if_false) = (monkey.if_true, monkey.if_false);

            let monkey = monkeys.get_mut(m).unwrap();
            monkey.inspections += monkey.items.len();
            monkey.items = vec!();

            monkeys.get_mut(if_true).unwrap().items.append(&mut items_true);
            monkeys.get_mut(if_false).unwrap().items.append(&mut items_false);
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys.get(0).unwrap().inspections * monkeys.get(1).unwrap().inspections
}

pub fn solve(input: &str) -> usize {
    let mut monkeys = process(input);

    let test_product = monkeys
        .iter()
        .fold(1, |product, monkey| product * monkey.test);

    for _round in 0..ROUNDS_TWO {
        for m in 0..monkeys.len() {
            let monkey = monkeys.get(m).unwrap();

            let (mut items_true, mut items_false): (Vec<usize>, Vec<usize>) = monkey.items
                .iter()
                .map(|item|
                    inspect(item % test_product, &monkey.op)
                )
                .partition(|item|
                    item % monkey.test == 0
                );

            let (if_true, if_false) = (monkey.if_true, monkey.if_false);

            let monkey = monkeys.get_mut(m).unwrap();
            monkey.inspections += monkey.items.len();
            monkey.items = vec!();

            monkeys.get_mut(if_true).unwrap().items.append(&mut items_true);
            monkeys.get_mut(if_false).unwrap().items.append(&mut items_false);
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys.get(0).unwrap().inspections * monkeys.get(1).unwrap().inspections
}
