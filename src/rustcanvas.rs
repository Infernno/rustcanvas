use std::fs::File;
use std::io::{self, Write};
use std::mem::swap;

pub struct RustCanvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl RustCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height];
        return RustCanvas {
            width,
            height,
            pixels,
        };
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        assert!(x < self.width && y < self.height, "{x}, {y} out of bounds!");

        let index = y * self.width + x;
        self.pixels[index] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            return Some(self.pixels[index]);
        } else {
            None
        }
    }

    pub fn save_to_ppm(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        // Write PPM header
        write!(file, "P6\n{} {}\n255\n", self.width, self.height)?;

        // Write pixel data as binary
        for pixel in &self.pixels {
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            file.write_all(&[r, g, b])?;
        }

        file.flush()?;

        Ok(())
    }
}

impl RustCanvas {
    pub fn clear(&mut self, color: u32) {
        for pixel in &mut self.pixels {
            *pixel = color
        }
    }

    pub fn fill_rect(&mut self, top: usize, left: usize, bottom: usize, right: usize, color: u32) {
        for x in left..right {
            for y in top..bottom {
                self.set_pixel(x, y, color)
            }
        }
    }

    pub fn line_to(&mut self,
                   mut x1: usize,
                   mut y1: usize,
                   mut x2: usize,
                   mut y2: usize,
                   color: u32) {
        // y = k * x + c

        // y1 = k * x1 + c
        // y2 = k * x2 + c

        // c = k * x1 - y1
        // k = (y1 + y2) / (x2 + x1)

        if y1 > y2 { swap(&mut y1, &mut y2); }
        if x1 > x2 { swap(&mut x1, &mut x2); }

        // vertical line
        if x1 + x2 == 0 {
            for y in y1..y2 {
                self.set_pixel(x1, y, color);
            }
        } else {
            let k = (y1 + y2) as f32 / (x1 + x2) as f32;
            let c = k * x1 as f32 - y1 as f32;

            for x in x1..x2 {
                let y = k * x as f32 + c;
                self.set_pixel(x, y as usize, color)
            }
        }
    }

    fn swap(a: &mut usize, b: &mut usize) {
        if b > a {
            let t = a;

            //   a = b;
            //    b = t;
        }
    }
}