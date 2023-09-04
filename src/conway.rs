use crate::lib::{Rule};

#[derive(Clone)]
struct ConwayRule;

impl Rule for ConwayRule {
    fn apply(&self, alive_neighbors: usize, current_state: &Cell) -> Cell {
        // Your rule logic here
        match current_state {
            Cell::Alive => {
                if alive_neighbors == 2 || alive_neighbors == 3 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            },
            Cell::Dead => {
                if alive_neighbors == 3 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            },
        }
    }
}