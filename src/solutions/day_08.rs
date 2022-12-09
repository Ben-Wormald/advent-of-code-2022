use std::cmp::Ordering;

#[derive(Eq, Debug)]
struct Tree {
    height: isize,
    is_visible: bool,
    score: usize,
}

impl Tree {
    fn new(height: char) -> Tree {
        let height = height.to_string().parse().unwrap();
        Tree {
            height,
            is_visible: false,
            score: 0,
        }
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

trait Trees {
    fn set_visibility(&mut self, row: usize, col: usize, highest: &mut isize);

    fn set_score(&mut self, row: usize, col: usize);

    fn update_score(
        &mut self,
        score: &mut usize,
        blocked: &mut bool,
        row: usize,
        col: usize,
        other_row: usize,
        other_col: usize,
    );
}

impl Trees for Vec<Vec<Tree>> {
    fn set_visibility(&mut self, row: usize, col: usize, highest: &mut isize) {
        let tree = self.get_mut(row).unwrap().get_mut(col).unwrap();
        if tree.height > *highest {
            tree.is_visible = true;
            *highest = tree.height;
        }
    }

    fn update_score(
        &mut self,
        score: &mut usize,
        blocked: &mut bool,
        row: usize,
        col: usize,
        other_row: usize,
        other_col: usize,
    ) {
        if !*blocked {
            *score += 1;
        }

        let tree = self.get(row).unwrap().get(col).unwrap();
        let other_tree = self.get(other_row).unwrap().get(other_col).unwrap();

        if other_tree.height >= tree.height {
            *blocked = true;
        }
    }

    fn set_score(&mut self, row: usize, col: usize) {
        let rows = self.len();
        let cols = self.first().unwrap().len();

        let mut scores = [0, 0, 0, 0];

        let mut blocked = false;
        for other_row in (0..row).rev() {
            self.update_score(&mut scores[0], &mut blocked, row, col, other_row, col);
        }

        blocked = false;
        for other_row in (row + 1)..rows {
            self.update_score(&mut scores[1], &mut blocked, row, col, other_row, col);
        }

        blocked = false;
        for other_col in (0..col).rev() {
            self.update_score(&mut scores[2], &mut blocked, row, col, row, other_col);
        }

        blocked = false;
        for other_col in (col + 1)..cols {
            self.update_score(&mut scores[3], &mut blocked, row, col, row, other_col);
        }

        let tree = self.get_mut(row).unwrap().get_mut(col).unwrap();
        tree.score = scores.iter().product();
    }
}

fn process(input: &str) -> (Vec<Vec<Tree>>, usize, usize) {
    let trees: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| line
            .chars()
            .map(|height| Tree::new(height))
            .collect()
        )
        .collect();

    let rows = trees.len();
    let cols = trees.first().unwrap().len();

    (trees, rows, cols)
}

pub fn solve_part_one(input: &str) -> usize {
    let (mut trees, rows, cols) = process(input);

    for row in 0..rows {
        let mut highest = -1;
        for col in 0..cols {
            trees.set_visibility(row, col, &mut highest);
        }

        highest = -1;
        for col in (0..cols).rev() {
            trees.set_visibility(row, col, &mut highest);
        }
    }

    for col in 0..cols {
        let mut highest = -1;
        for row in 0..rows {
            trees.set_visibility(row, col, &mut highest);
        }

        highest = -1;
        for row in (0..rows).rev() {
            trees.set_visibility(row, col, &mut highest);
        }
    }
    
    trees
        .iter()
        .fold(0, |sum, row|
            sum + row
                .iter()
                .fold(0, |row_sum, tree| {
                    if tree.is_visible {
                        row_sum + 1
                    } else {
                        row_sum
                    }
                })
        )
}

pub fn solve(input: &str) -> usize {
    let (mut trees, rows, cols) = process(input);

    for row in 0..rows {
        for col in 0..cols {
            trees.set_score(row, col);
        }
    }
    
    trees
        .iter()
        .max_by(|row_a, row_b| {
            let max_a = row_a.iter().max().unwrap();
            let max_b = row_b.iter().max().unwrap();
            max_a.cmp(max_b)
        })
        .unwrap()
        .iter()
        .max()
        .unwrap()
        .score
}
