use itertools::Itertools;

type Stack = Vec<char>;

#[derive(Debug)]
struct Step {
    n: usize,
    from: usize,
    to: usize,
}

trait ToStep {
    fn to_step(&self) -> Step;
}

impl ToStep for &str {
    fn to_step(&self) -> Step {
        let parts: Vec<&str> = self.split_whitespace().collect();
        Step {
            n: parts.get(1).unwrap().parse().unwrap(),
            from: parts.get(3).unwrap().parse::<usize>().unwrap() - 1,
            to: parts.get(5).unwrap().parse::<usize>().unwrap() - 1,
        }
    }
}

fn process(input: &str) -> (Vec<Stack>, Vec<Step>) {
    let (stack_rows, steps) = input
        .split("\n\n")
        .collect_tuple()
        .unwrap();

    let mut stack_rows: Vec<&str> = stack_rows.lines().collect();
    stack_rows.pop();
    stack_rows.reverse();

    let n_stacks = (stack_rows.first().unwrap().len() + 1) / 4;

    let mut stacks = vec![Vec::new(); n_stacks];

    for n in 0..n_stacks {
        let idx = (n * 4) + 1;
        stack_rows
            .iter()
            .for_each(|stack_row| {
                let item = stack_row.chars().nth(idx).unwrap();
                if item != ' ' {
                    stacks.get_mut(n).unwrap().push(item);
                }
            });
    }

    let steps = steps
        .lines()
        .map(|step_line| step_line.to_step())
        .collect();

    (stacks, steps)
}

pub fn solve_part_one(input: &str) -> String {
    let (mut stacks, steps) = process(input);

    steps
        .iter()
        .for_each(|step| {
            for _i in 0..step.n {
                let source = stacks.get_mut(step.from).unwrap();
                let item = source.pop().expect("stack is empty!");
                let dest = stacks.get_mut(step.to).unwrap();
                dest.push(item);
            }
        });
    
    top_items(stacks)
}

fn top_items(stacks: Vec<Stack>) -> String {
    stacks
        .iter()
        .map(|stack| match stack.last() {
            Some(item) => item.to_string(),
            None => "".to_string(),
        })
        .join("")
}

pub fn solve(input: &str) -> String {
    let (mut stacks, steps) = process(input);

    steps
        .iter()
        .for_each(|step| {
            let mut items = Vec::new();
            let source = stacks.get_mut(step.from).unwrap();

            for _i in 0..step.n {
                items.push(source.pop().expect("stack is empty!"));
            }
            items.reverse();

            let dest = stacks.get_mut(step.to).unwrap();

            items.iter().for_each(|item| dest.push(*item));
        });
    
    top_items(stacks)
}
