extern crate piston_window;

use piston_window::{clear, PistonWindow, WindowSettings};

pub struct Board {
    size: i32,
    grid: Vec<Cell>,
    frames: Vec<Vec<Cell>>,
    frame_index: usize,
    rule: dyn Rule,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

pub trait Rule {
    fn apply(&self, alive_neighbors: usize, current_state: &Cell) -> Cell;
}
