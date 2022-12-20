use itertools::Itertools;

const MINUTES: u8 = 24;

#[derive(Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn from_string(resource: &str) -> Resource {
        match resource {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            _ => panic!("unknown type!"),
        }
    }
}

#[derive(Debug)]
struct Robot {
    resource: Resource,
    cost: Vec<(Resource, u8)>,
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    robots: Vec<Robot>,
}

pub fn solve(input: &str) -> usize {
    let blueprints = parse(input);

    blueprints
        .iter()
        .fold(0, |sum, blueprint| 
            sum + get_quality(&blueprint)
        )
}

fn get_quality(blueprint: &Blueprint) -> usize {
    let mut robots = vec!(Resource::Ore);
    0
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let (id, robots) = line.split(": ").collect_tuple().unwrap();
            let id = id.replace("Blueprint ", "").parse().unwrap();

            let robots = robots
                .split(". ")
                .map(|robot| {
                    let robot = robot
                        .replace("Each ", "")
                        .replace(" robot costs ", ";")
                        .replace(".", "");

                    let (resource, cost) = robot.split(";").collect_tuple().unwrap();

                    let resource = Resource::from_string(resource);

                    let cost = cost
                        .split(" and ")
                        .map(|c| {
                            let (amount, resource) = c.split_whitespace().collect_tuple().unwrap();
                            let amount = amount.parse::<u8>().unwrap();
                            let resource = Resource::from_string(resource);
                            (resource, amount)
                        })
                        .collect();

                    Robot {
                        resource,
                        cost,
                    }
                })
                .collect();

            Blueprint {
                id,
                robots,
            }
        })
        .collect()
}
 