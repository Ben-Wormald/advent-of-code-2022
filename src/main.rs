#![allow(dead_code)]
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    test: bool,
}

mod solutions { pub mod day_16; }
use solutions::day_16::solve;
const INPUT: &str = "./input/16";
const TEST_INPUT: &str = "./input/test";

fn main() {
    let opt = Opt::from_args();
    let input = if opt.test { TEST_INPUT } else { INPUT };
    let input = fs::read_to_string(input).expect("oh no!");

    let result = solve(&input);
    println!("{}", result)
}
