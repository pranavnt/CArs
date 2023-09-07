use pulsar::{Cell, Rule};
use std::collections::HashSet;

pub struct Rule30 {
    current_row: usize, // Add this field to keep track of the current row
}

impl Rule30 {
    pub fn new() -> Rule30 {
        Rule30 {
            current_row: 0, // Initialize current_row to 0
        }
    }
}

impl Rule for Rule30 {
    fn apply(&mut self, grid: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let mut new_grid = grid.clone(); // Start with the existing grid
        let mut candidates = HashSet::new();

        // Build the set of candidates for state change based on the current_row
        for &(x, y) in grid.iter().filter(|&&(_, y)| y == self.current_row) {
            candidates.insert(x);
            if x > 0 {
                candidates.insert(x - 1);
            }
            candidates.insert(x + 1);
        }

        // Evaluate each candidate for the next state
        for &x in candidates.iter() {
            let left = if x > 0 { grid.contains(&(x - 1, self.current_row)) } else { false };
            let center = grid.contains(&(x, self.current_row));
            let right = grid.contains(&(x + 1, self.current_row));

            if rule_30_next_state(left, center, right) {
                new_grid.insert((x, self.current_row + 1));
            }
        }

        self.current_row += 1; // Increment current_row for the next iteration

        new_grid
    }
}

// The same rule_30_next_state function as before
fn rule_30_next_state(left: bool, center: bool, right: bool) -> bool {
    match (left, center, right) {
        (true, true, true) => false,
        (true, true, false) => false,
        (true, false, true) => false,
        (true, false, false) => true,
        (false, true, true) => true,
        (false, true, false) => true,
        (false, false, true) => true,
        (false, false, false) => false,
    }
}

