use pulsar::{Cell, Rule};

#[derive(Clone)]
pub struct Rule30 {
    curr_row: usize,
}

impl Rule30 {
    pub fn new() -> Rule30 {
        Rule30 { curr_row: 1 }
    }
}

impl Rule for Rule30 {
    fn apply(&mut self, grid: &Vec<Cell>) -> Vec<Cell> {
        let mut new_grid = grid.clone();

        let grid_len = (grid.len() as f64).sqrt() as usize;
        for pos in self.curr_row..self.curr_row + grid_len {
            let col = pos % grid_len;

            // get the top right, top middle, and top left cells
            // rule 30 mapping is:
            // 111	110	101	100	011	010	001	000
            // 0	0	0	1	1	1	1	0
            
            let top_left: bool = if col == 0 || self.curr_row == 0 {
                false
            } else {
                grid[grid_len * (self.curr_row - 1) + col - 1] == Cell::Alive
            };

            let top_middle: bool = if self.curr_row == 0 {
                false
            } else {
                grid[grid_len * (self.curr_row - 1) + col] == Cell::Alive
            };

            let top_right: bool = if col == grid_len - 1 || self.curr_row == 0 {
                false
            } else {
                grid[grid_len * (self.curr_row - 1) + col + 1] == Cell::Alive
            }; 

            // print coordinates where any of these are true
            if top_left || top_middle || top_right {
                println!("({}, {})", col, self.curr_row);
            }
            
            
            if top_left && top_middle && top_right {
                new_grid[pos] = Cell::Dead;
            } else if top_left && top_middle && !top_right {
                new_grid[pos] = Cell::Dead;
            } else if top_left && !top_middle && top_right {
                new_grid[pos] = Cell::Dead;
            } else if top_left && !top_middle && !top_right {
                new_grid[pos] = Cell::Alive;
            } else if !top_left && top_middle && top_right {
                new_grid[pos] = Cell::Alive;
            } else if !top_left && top_middle && !top_right {
                new_grid[pos] = Cell::Alive;
            } else if !top_left && !top_middle && top_right {
                new_grid[pos] = Cell::Alive;
            } else if !top_left && !top_middle && !top_right {
                new_grid[pos] = Cell::Dead;
            }
        }
        self.curr_row += 1;

        new_grid
    }
}
