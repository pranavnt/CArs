use cars::{Cell, Rule};
use std::collections::HashSet;

pub struct ElementaryRule {
    rule_number: u8,
    current_row: usize,
}

impl ElementaryRule {
    pub fn new(rule_number: u8) -> ElementaryRule {
        ElementaryRule {
            rule_number,
            current_row: 0,
        }
    }
}

impl Rule for ElementaryRule {
    fn apply(&mut self, grid: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let mut new_grid = grid.clone();
        let mut candidates = HashSet::new();

        for &(x, y) in grid.iter().filter(|&&(_, y)| y == self.current_row) {
            candidates.insert(x);
            if x > 0 {
                candidates.insert(x - 1);
            }
            candidates.insert(x + 1);
        }

        for &x in candidates.iter() {
            let left = if x > 0 { grid.contains(&(x - 1, self.current_row)) } else { false };
            let center = grid.contains(&(x, self.current_row));
            let right = grid.contains(&(x + 1, self.current_row));

            if elementary_rule_next_state(self.rule_number, left, center, right) {
                new_grid.insert((x, self.current_row + 1));
            }
        }

        self.current_row += 1;

        new_grid
    }
}

fn elementary_rule_next_state(rule_number: u8, left: bool, center: bool, right: bool) -> bool {
    let index = (left as u8) << 2 | (center as u8) << 1 | (right as u8);
    (rule_number >> index) & 1 == 1
}
