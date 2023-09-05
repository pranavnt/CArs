use crate::lib::{Rule, Cell};

#[derive(Clone)]
pub struct ConwayRule {}

impl Rule for ConwayRule {
    fn apply(&mut self, grid: &Vec<Cell>) -> Vec<Cell> {
        let mut new_grid = grid.clone();
        
        for (pos, cell) in grid.iter().enumerate() {
            let x: usize = pos % 100;
            let y: usize = pos / 100;
            let mut alive_neighbors = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 { continue; }
                    let x = (x as i32 + i) as usize;
                    let y = (y as i32 + j) as usize;
                    if x >= 100 || y >= 100 { continue; }
                    if grid[x * 100 + y] == Cell::Alive {
                        alive_neighbors += 1;
                    }
                }
            }
            match cell {
                Cell::Alive => {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        new_grid[pos] = Cell::Dead;
                    }
                },
                Cell::Dead => {
                    if alive_neighbors == 3 {
                        new_grid[pos] = Cell::Alive;
                    }
                },
            }
        }
        new_grid
    }
}