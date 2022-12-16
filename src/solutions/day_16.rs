use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const MINUTES: usize = 30;
const START: &str = "AA";

struct Graph {
    nodes: HashMap<String, usize>,
    edges: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Action {
    Move(String),
    Activate(String),
}

type MemoKey = (String, String, String, usize);
type Memo = HashMap<MemoKey, (Vec<Action>, usize)>;

impl Graph {
    fn new(input: &str) -> Graph {
        let mut graph = Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        };

        input
            .lines()
            .for_each(|line| {
                let line = line
                    .replace("Valve ", "")
                    .replace(" has flow rate=", ";")
                    .replace("; tunnel leads to ", ";")
                    .replace("; tunnels lead to ", ";")
                    .replace("valve ", "")
                    .replace("valves ", "");

                let (key, rate, adj) = line
                    .split(";")
                    .collect_tuple().unwrap();

                graph.nodes.insert(key.to_string(), rate.parse().unwrap());

                adj.split(", ").for_each(|adj_key|
                    graph.add_edge(key.to_string(), adj_key.to_string())
                );
            });

        graph
    }

    fn add_edge(&mut self, key_a: String, key_b: String) {
        self.edges.entry(key_a.clone())
            .and_modify(|adj| adj.push(key_b.clone()))
            .or_insert(vec!(key_b.clone()));

        self.edges.entry(key_b)
            .and_modify(|adj| adj.push(key_a.clone()))
            .or_insert(vec!(key_a));
    }
}

pub fn solve(input: &str) -> usize {
    let graph = Graph::new(input);

    let current_node = START.to_string();
    let activated: HashSet<String> = HashSet::new();
    let mut memo: Memo = HashMap::new();

    let moves = get_best_path(&graph, current_node, activated, MINUTES, vec!(), 0, &mut memo);

    moves.1
}

fn get_best_path(
    graph: &Graph,
    current_node: String,
    activated: HashSet<String>,
    minutes: usize,
    path: Vec<Action>,
    score: usize,
    memo: &mut Memo,
) -> (Vec<Action>, usize) {
    if minutes == 0 {
        return (path, score);
    }

    let mut options: Vec<(Vec<Action>, usize)> = vec!();

    let is_inactive = !activated.contains(&current_node);
    let rate_is_nonzero = *graph.nodes.get(&current_node).unwrap() > 0;

    if is_inactive && rate_is_nonzero {
        let option = Action::Activate(current_node.clone());

        let mut option_activated = activated.clone();
        option_activated.insert(current_node.clone());

        let mut option_path = path.clone();
        option_path.push(option.clone());

        let memo_key = get_memo_key(
            current_node.clone(),
            current_node.clone(),
            &option_activated,
            minutes,
        );

        let mut option_best_path = match memo.get(&memo_key) {
            Some(result) => result.clone(),
            None => {
                let result = get_best_path(
                    graph,
                    current_node.clone(),
                    option_activated,
                    minutes - 1,
                    option_path,
                    score,
                    memo,
                );
                memo.insert(memo_key, result.clone());
                result
            },
        };

        let activation_score = (minutes - 1) * graph.nodes.get(&current_node).unwrap();
        option_best_path.1 += activation_score;

        options.push(option_best_path);
    }

    if minutes > 1 {
        for neighbour in graph.edges.get(&current_node).unwrap() {
            let option = Action::Move(neighbour.clone());

            let mut option_path = path.clone();
            option_path.push(option.clone());

            let memo_key = get_memo_key(
                current_node.clone(),
                neighbour.clone(),
                &activated,
                minutes,
            );

            let option_best_path = match memo.get(&memo_key) {
                Some(result) => result.clone(),
                None => {
                    let result = get_best_path(
                        graph,
                        neighbour.clone(),
                        activated.clone(),
                        minutes - 1,
                        option_path,
                        score,
                        memo,
                    );
                    memo.insert(memo_key, result.clone());
                    result
                },
            };

            options.push(option_best_path);
        }
    }

    if options.is_empty() {
        return (path, score);
    }

    let best_option = options
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    (best_option.0.clone(), score + best_option.1)
}

fn get_memo_key(
    current_node: String,
    target_node: String,
    activated: &HashSet<String>,
    minutes: usize,
) -> MemoKey {
    let mut activated: Vec<String> = activated.iter().cloned().collect();
    activated.sort();
    let activated = activated.iter().join("");

    (current_node, target_node, activated, minutes)
}
