use cars::{Board, Cell, Rule};
use std::collections::HashSet;

pub struct ConwayRule {}

impl Rule for ConwayRule {
    fn apply(&mut self, grid: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let mut new_grid = HashSet::new();
        let mut candidates = HashSet::new();

        // Build the set of candidates for state change (all alive cells and their neighbors)
        for &(x, y) in grid.iter() {
            candidates.insert((x, y));
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && ny >= 0 {
                        candidates.insert((nx as usize, ny as usize));
                    }
                }
            }
        }

        // Evaluate each candidate for the next state
        for &(x, y) in candidates.iter() {
            let mut alive_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && ny >= 0 {
                        if grid.contains(&(nx as usize, ny as usize)) {
                            alive_neighbors += 1;
                        }
                    }
                }
            }

            if grid.contains(&(x, y)) {
                if alive_neighbors == 2 || alive_neighbors == 3 {
                    new_grid.insert((x, y));
                }
            } else {
                if alive_neighbors == 3 {
                    new_grid.insert((x, y));
                }
            }
        }

        new_grid
    }
}
