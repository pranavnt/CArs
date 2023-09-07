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
    pub fn new(size: usize, rule: Box<dyn Rule>) -> Board {
        Board {
            size,
            grid: vec![Cell::Dead; size * size],
            frames: Vec::<Vec<Cell>>::new(),
            rule,
        }
    }

    pub fn snapshot(&mut self) {
        self.frames.push(self.grid.clone());
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.grid[x * self.size + y] = value;
    }

    pub fn tick(&mut self) {
        self.grid = self.rule.apply(&self.grid);
        self.frames.push(self.grid.clone());
    }

    pub fn render(&self, dir_name: &str) {
        let mut i = 0;
        for frame in &self.frames {
            let mut img: RgbImage = ImageBuffer::new((10 * self.size) as u32,  (10 * self.size) as u32);

            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let cell = frame[(x / 10 * self.size as u32 + y / 10) as usize];
                let color = match cell {
                    Cell::Alive => Rgb([255, 255, 255]),
                    Cell::Dead => Rgb([0, 0, 0]),
                };
                *pixel = color;
            }

            let path = Path::new(dir_name)
                .join(&i.to_string())
                .with_extension("png");

            img.save(path).unwrap();

            i += 1;
        }

        let output_path = Path::new(dir_name).join("output.mp4");
        let _ = Command::new("ffmpeg")
            .arg("-framerate")
            .arg("30".to_string())
            .arg("-i")
            .arg(dir_name.to_owned() + "/%d.png")
            .arg("-c:v")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-crf")
            .arg("0")
            .arg("-preset")
            .arg("ultrafast")
            .arg("-qp")
            .arg("0")
            .arg("-pix_fmt")
            .arg("yuv444p")
            .arg(output_path)
            .output()
            .unwrap();
    }
}
