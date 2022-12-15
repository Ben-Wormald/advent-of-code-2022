use itertools::Itertools;
use std::cmp::max;

const TARGET_TEST_Y: isize = 10;
const TARGET_Y: isize = 2000000;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    coord: Coord,
    closest_beacon: Coord,
}

impl Sensor {
    fn distance(&self) -> isize {
        let d_x = self.coord.x - self.closest_beacon.x;
        let d_y = self.coord.y - self.closest_beacon.y;
        d_x.abs() + d_y.abs()
    }
}

pub fn solve(input: &str) -> isize {
    let sensors = process(input);
    
    let ranges = get_ranges(&sensors);

    let mut covered = 0;
    let mut current_start = ranges.first().unwrap().0;

    for range in &ranges {
        current_start = max(current_start, range.0);
        covered += max(range.1 - current_start + 1, 0);
        current_start = max(current_start, range.1 + 1);
    }

    covered -= sensors.iter()
        .map(|sensor| &sensor.closest_beacon)
        .unique()
        .filter(|beacon| beacon.y == TARGET_Y)
        .count() as isize;

    covered
}

fn process(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let line = line.replace("Sensor at x=", "")
                .replace(", y=", ",")
                .replace(": closest beacon is at x=", ",");

            let (s_x, s_y, b_x, b_y) = line
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect_tuple().unwrap();

            Sensor {
                coord: Coord { x: s_x, y: s_y },
                closest_beacon: Coord { x: b_x, y: b_y },
            }
        })
        .collect()
}

fn get_ranges(sensors: &Vec<Sensor>) -> Vec<(isize, isize)> {
    let mut ranges: Vec<(isize, isize)> = sensors
        .iter()
        .map(|sensor| {
            let beacon_distance = sensor.distance();
            let target_distance = (sensor.coord.y - TARGET_Y).abs();

            if target_distance > beacon_distance {
                None
            } else {
                let range_start = sensor.coord.x - (beacon_distance - target_distance);
                let range_end = sensor.coord.x + (beacon_distance - target_distance);

                Some((range_start, range_end))
            }
        })
        .filter_map(|range| range)
        .collect();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ranges
}
