enum Instruction {
    Addx(isize),
    Noop,
}

fn process(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Instruction::Noop,
            _ => Instruction::Addx(
                line.replace("addx ", "").parse().unwrap()
            ),
        })
        .collect()
}

pub fn solve_part_one(input:  &str) -> isize {
    let instructions = process(input);

    let mut cycle = 1;
    let mut i_ptr = 0;
    let mut timer: Option<usize> = None;
    let mut x = 1;

    let mut sum = 0;

    while let Some(instruction) = instructions.get(i_ptr) {
        if let Instruction::Addx(value) = instruction {
            if let Some(mut time) = timer {
                time -= 1;

                if time == 0 {
                    x += value;
                    timer = None;
                    i_ptr += 1;
                }
            } else {
                timer = Some(1);
            }
        } else {
            i_ptr += 1;
        }

        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }
    }

    sum
}

pub fn solve(input:  &str) -> String {
    let instructions = process(input);

    let mut cycle = 0;
    let mut i_ptr = 0;
    let mut timer: Option<usize> = None;
    let mut x = 1;
    
    let mut screen = [false; 240];

    while let Some(instruction) = instructions.get(i_ptr) {
        if cycle >= x - 1 && cycle <= x + 1 {
            screen[(cycle) as usize] = true;
        }

        if let Instruction::Addx(value) = instruction {
            if let Some(mut time) = timer {
                time -= 1;

                if time == 0 {
                    x += value;
                    timer = None;
                    i_ptr += 1;
                }
            } else {
                timer = Some(1);
            }
        } else {
            i_ptr += 1;
        }

        cycle += 1;

        if cycle % 40 == 0 {
            x += 40;
        }
    }

    get_output(screen)
}

fn get_output(screen: [bool; 240]) -> String {
    let mut output = String::new();

    screen.iter().enumerate().for_each(|(idx, pixel)| {
        output.push(match pixel {
            true => '#',
            false => '.',
        });

        if (idx + 1) % 40 == 0 {
            output.push('\n');
        }
    });

    output
}
