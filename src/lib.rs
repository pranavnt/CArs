extern crate piston_window;
extern crate image;

use image::{ImageBuffer, Rgb, RgbImage};
use std::path::Path;
use std::process::Command;

pub struct Board {
    pub size: usize,
    pub grid: Vec<Cell>,
    pub frames: Vec<Vec<Cell>>,
    pub rule: Box<dyn Rule>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            grid: vec![Cell::Dead; size * size],
            frames: Vec::<Vec<Cell>>::new(),
            rule,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.grid[x * self.size + y] = value;
        self.frames.push(self.grid.clone());
    }

    pub fn tick(&mut self) {
        self.grid = self.rule.apply(&self.grid);
        self.frames.push(self.grid.clone());
    }
   
    pub fn render(&self, dir_name: &str) {
        let mut i = 0;
        for frame in &self.frames {
            let mut img: RgbImage = ImageBuffer::new(self.size as u32, self.size as u32);
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let cell = frame[(x * self.size as u32 + y) as usize];
                let color = match cell {
                    Cell::Alive => Rgb([255, 255, 255]),
                    Cell::Dead => Rgb([0, 0, 0]),
                };
                *pixel = color;
            }
            let path = Path::new(dir_name + "/" + &i.to_string());
        }
        // let _ = Command::new("ffmpeg")
        //     .arg("-y")
        //     .arg("-i")
        //     .arg(format!("{}.png", path.to_str().unwrap()))
        //     .arg("-c:v")
        //     .arg("libx264")
        //     .arg("-pix_fmt")
        //     .arg("yuv420p")
        //     .arg("-crf")
        //     .arg("23")
        //     .arg("-preset")
        //     .arg("medium")
        //     .arg(format!("{}.mp4", path.to_str().unwrap()))
        //     .output()
        //     .unwrap();
    }
}