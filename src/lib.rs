extern crate piston_window;

use piston_window::*;

pub struct Board {
    size: usize,
    grid: Vec<Cell>,
    frames: Vec<Vec<Cell>>,
    f_index: usize,
    rule: Box<dyn Rule>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

pub trait Rule {
    fn apply(&mut self, state: &Vec<Cell>) -> Vec<Cell>;
}

impl Board {
    pub fn new(size: usize, rule: Box<dyn Rule>) -> Board{
        Board {
            size,
            grid: vec![Cell::Dead; size as usize],
            frames: Vec::<Vec<Cell>>::new(),
            f_index: 0,
            rule,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.grid[x * self.size + y] = value
    }

    pub fn tick(&mut self) {
        self.grid = self.rule.apply(&self.grid);
        self.frames.push(self.grid.clone());
    }

    pub fn render_video(&self) {
        let mut window: PistonWindow = WindowSettings::new("Game of Life", [self.size as u32, self.size as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();
        let mut frame_index = 0;
        while let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                let frame = &self.frames[frame_index];
                window.draw_2d(&event, |context: Context, graphics, _| {
                    clear([0.0, 0.0, 0.0, 1.0], graphics);
                    for (pos, cell) in self.grid.iter().enumerate() {
                        let color: [f32; 4] = match cell {
                            Cell::Alive => [1.0, 1.0, 1.0, 1.0],
                            Cell::Dead => [0.0, 0.0, 0.0, 1.0],
                        };
                        let x: usize = pos % self.size;
                        let y: usize = pos / self.size;
                        let rect: [f64; 4] = rectangle::square(x as f64, y as f64, 1.0);
                        rectangle(color, rect, context.transform, graphics);
                    }
                });
                frame_index = (frame_index + 1) % self.frames.len();
            }
        }
    }
}
