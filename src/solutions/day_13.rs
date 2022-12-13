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
}

impl Ord for Token {
    fn cmp(&self, other: &Token) -> Ordering {
        match (self, other) {
            (Token::Num(left), Token::Num(right)) => left.cmp(right),
            (Token::Num(_), Token::List(_)) => Token::List(vec!(self.clone())).cmp(other),
            (Token::List(_), Token::Num(_)) => self.cmp(&Token::List(vec!(other.clone()))),
            (Token::List(list_left), Token::List(list_right)) => {
                let mut list_ordering = Ordering::Less;

                for (i, left) in list_left.iter().enumerate() {
                    if let Some(right) = list_right.get(i) {
                        let ordering = left.cmp(right);

                        match ordering {
                            Ordering::Equal => (),
                            _ => {
                                list_ordering = ordering;
                                break;
                            },
                        }
                    } else {
                        list_ordering = Ordering::Greater;
                    }
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

fn process(input: &str) -> Vec<(Token, Token)> {
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

pub fn solve(input: &str) -> usize {
    let pairs = process(input);
    
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
