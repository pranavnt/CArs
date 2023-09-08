extern crate image;

use image::{ImageBuffer, Rgb, RgbImage};
use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

pub struct Board {
    pub grid: HashSet<(usize, usize)>,
    pub frames: Vec<HashSet<(usize, usize)>>,
    pub rule: Box<dyn Rule>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

pub trait Rule {
    fn apply(&mut self, state: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)>;
}


impl Board {
    pub fn new(rule: Box<dyn Rule>) -> Board {
        Board {
            grid: HashSet::new(),
            frames: Vec::new(),
            rule,
        }
    }

    pub fn snapshot(&mut self) {
        self.frames.push(self.grid.clone());
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        match value {
            Cell::Alive => {
                self.grid.insert((x, y));
            }
            Cell::Dead => {
                self.grid.remove(&(x, y));
            }
        }
    }

    pub fn tick(&mut self) {
        self.grid = self.rule.apply(&mut self.grid);
        self.snapshot();
    }

    pub fn export(&self, dir_name: &str) {
        let mut i = 0;
        for frame in &self.frames {
            let img_size = (10 * 100) as u32;
            let mut img: RgbImage = ImageBuffer::new(img_size, img_size);

            // Initialize the image as black (all cells dead)
            for (_, _, pixel) in img.enumerate_pixels_mut() {
                *pixel = Rgb([0, 0, 0]);
            }

            // Set alive cells to white
            // Set alive cells to white
            for &(x, y) in frame.iter() {
                for dx in 0..10 {
                    for dy in 0..10 {
                        let pixel_x = (x * 10 + dx) as u32;
                        let pixel_y = (y * 10 + dy) as u32;
                        if pixel_x < img_size && pixel_y < img_size {
                            let pixel = img.get_pixel_mut(pixel_x, pixel_y);
                            *pixel = Rgb([255, 255, 255]);
                        }
                    }
                }
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
            .arg("10".to_string())
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
