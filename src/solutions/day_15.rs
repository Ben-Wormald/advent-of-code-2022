use itertools::Itertools;
use std::cmp::max;

const TARGET_TEST_Y: isize = 10;
const TARGET_Y: isize = 2000000;

const LIMIT_Y: isize = 4000000;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
}

impl Sensor {
    fn distance(&self) -> isize {
        let d_x = self.pos.x - self.beacon.x;
        let d_y = self.pos.y - self.beacon.y;
        d_x.abs() + d_y.abs()
    }
}

pub fn solve_part_one(input: &str) -> isize {
    let sensors = process(input);
    let ranges = get_ranges(&sensors, TARGET_Y);

    let mut covered = 0;
    let mut current_start = ranges.first().unwrap().0;

    for range in &ranges {
        current_start = max(current_start, range.0);
        covered += max(range.1 - current_start + 1, 0);
        current_start = max(current_start, range.1 + 1);
    }

    covered -= sensors.iter()
        .map(|sensor| &sensor.beacon)
        .unique()
        .filter(|beacon| beacon.y == TARGET_Y)
        .count() as isize;

    covered
}

pub fn solve(input: &str) -> isize {
    let sensors = process(input);
    let mut distress_beacon = None;

    for y in 0..=LIMIT_Y {
        let ranges = get_ranges(&sensors, y);
    
        if ranges.is_empty() {
            continue;
        }

        let mut current_start = ranges.first().unwrap().0;
    
        for range in &ranges {
            if range.0 > current_start {
                distress_beacon = Some((current_start, y));
                break;
            }

            current_start = max(current_start, range.0);
            current_start = max(current_start, range.1 + 1);
        }

        if distress_beacon.is_some() {
            break;
        }
    }

    let distress_beacon = distress_beacon.expect("no distress beacon found!");

    distress_beacon.0 * LIMIT_Y + distress_beacon.1
}

fn process(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("Sensor at x=", "")
                .replace(", y=", ",")
                .replace(": closest beacon is at x=", ",");

            let (s_x, s_y, b_x, b_y) = line
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect_tuple().unwrap();

            Sensor {
                pos: Coord { x: s_x, y: s_y },
                beacon: Coord { x: b_x, y: b_y },
            }
        })
        .collect()
}

fn get_ranges(sensors: &Vec<Sensor>, target: isize) -> Vec<(isize, isize)> {
    let mut ranges: Vec<(isize, isize)> = sensors
        .iter()
        .filter_map(|sensor| {
            let beacon_distance = sensor.distance();
            let target_distance = (sensor.pos.y - target).abs();

            if target_distance > beacon_distance {
                None
            } else {
                let range_start = sensor.pos.x - (beacon_distance - target_distance);
                let range_end = sensor.pos.x + (beacon_distance - target_distance);

                Some((range_start, range_end))
            }
        })
        .collect();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ranges
}
