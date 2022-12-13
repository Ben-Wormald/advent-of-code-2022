use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Clone, Debug, Eq)]
enum Token {
    List(Vec<Token>),
    Num(u8),
}

impl Token {
    fn from_str(s: &str) -> (Token, usize) {
        let chars: Vec<char> = s.chars().collect();

        let mut current_token = vec!();
        let mut digits = String::new();
        let mut i = 0;

        loop {
            match chars.get(i) {
                Some(c) => {
                    match c {
                        '[' => {
                            let (sub_token, consumed) = Token::from_str(&s[(i + 1)..]);
                            current_token.push(sub_token);
                            i += consumed + 1;
                        },
                        ',' => {
                            if !digits.is_empty() {
                                current_token.push(Token::Num(digits.parse().unwrap()));
                                digits = String::new();
                            }
                        },
                        ']' => break,
                        c => digits.push(*c),
                    }
                },
                None => break,
            }

            i += 1;
        }

        if !digits.is_empty() {
            current_token.push(Token::Num(digits.parse().unwrap()));
        }

        (Token::List(current_token), i)
    }

    fn divider(num: u8) -> Token {
        Token::List(vec!(Token::List(vec!(Token::Num(num)))))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Token) -> Ordering {
        match (self, other) {
            (Token::Num(left), Token::Num(right)) => left.cmp(right),
            (Token::Num(_), Token::List(_)) => Token::List(vec!(self.clone())).cmp(other),
            (Token::List(_), Token::Num(_)) => self.cmp(&Token::List(vec!(other.clone()))),
            (Token::List(list_left), Token::List(list_right)) => {
                let mut list_ordering = Ordering::Equal;

                for (i, left) in list_left.iter().enumerate() {
                    if let Some(right) = list_right.get(i) {
                        list_ordering = left.cmp(right);

                        if list_ordering != Ordering::Equal {
                            break;
                        }
                    }
                }

                if list_ordering == Ordering::Equal {
                    list_ordering = list_left.len().cmp(&list_right.len());
                }

                list_ordering
            },
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn process_pairs(input: &str) -> Vec<(Token, Token)> {
    input
        .split("\n\n")
        .map(|pair| pair
            .lines()
            .map(|line| Token::from_str(line).0)
            .collect_tuple()
            .unwrap()
        )
        .collect()
}

fn process_packets(input: &str) -> Vec<Token> {
    input
        .split_whitespace()
        .map(|line| Token::from_str(line).0)
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let pairs = process_pairs(input);
    
    pairs
        .iter()
        .enumerate()
        .fold(0, |sum, (i, pair)| {
            match pair.0.cmp(&pair.1) {
                Ordering::Greater => sum,
                _ => sum + i + 1,
            }
        })
}

pub fn solve(input: &str) -> usize {
    let mut packets = process_packets(input);

    let mut dividers = vec!(Token::divider(2), Token::divider(6));
    packets.append(&mut dividers);

    packets.sort();
    
    let div_idx_2 = packets.iter()
        .find_position(|packet| **packet == Token::divider(2)).unwrap().0 + 1;
    
    let div_idx_6 = packets.iter()
        .find_position(|packet| **packet == Token::divider(6)).unwrap().0 + 1;

    div_idx_2 * div_idx_6
}
