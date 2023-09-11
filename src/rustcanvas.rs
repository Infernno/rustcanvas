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

    pub fn xMax(&self) -> usize {
        self.width - 1
    }

    pub fn yMax(&self) -> usize {
        self.height - 1
    }

    pub fn centerX(&self) -> usize { self.xMax() / 2 }

    pub fn centerY(&self) -> usize { self.yMax() / 2 }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = y * self.width + x;

        if index < self.pixels.len() {
            self.pixels[index] = color;
        } else {
            panic!("Index {} is out of bounds: x = {}, y = {}, width = {}, height = {}", index, x, y, self.width, self.height)
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        let index = y * self.width + x;

        if index < self.pixels.len() {
            return Some(self.pixels[index]);
        }

        return None;
    }

    pub fn save_ppm(&self, path: &str) -> io::Result<()> {
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

    pub fn fill_rect(
        &mut self,
        mut x1: usize,
        mut x2: usize,
        mut y1: usize,
        mut y2: usize,
        color: u32,
    ) {
        self.check_in_range(x1, y1);
        self.check_in_range(x2, y2);

        if x1 > x2 { swap(&mut x1, &mut x2) }
        if y1 > y2 { swap(&mut y1, &mut y2) }

        for x in x1..x2 {
            for y in y1..y2 {
                self.set_pixel(x, y, color)
            }
        }
    }

    pub fn fill_circle(
        &mut self,
        x: usize,
        y: usize,
        radius: usize,
        color: u32,
    ) {
        let offset = radius / 2;

        let x1 = x - offset;
        let x2 = x + offset;

        let y1 = y - offset;
        let y2 = y + offset;

        for a in x1..x2 {
            for b in y1..y2 {
                let distance = (a as i32 - x as i32) + (b as i32 - y as i32);
                let r = (radius * radius) as i32;

                if distance <= r {
                    self.set_pixel(a, b, color)
                }
            }
        }
    }

    pub fn line_to(&mut self,
                   mut x1: usize,
                   mut y1: usize,
                   mut x2: usize,
                   mut y2: usize,
                   color: u32) {
        self.check_in_range(x1, y1);
        self.check_in_range(x2, y2);

        // y = k * x + c

        // y1 = k * x1 + c
        // y2 = k * x2 + c

        // c = y1 - k * x1
        // k = (y2 - y1) / (x2 - x1)

        let dx = (x2 as i32 - x1 as i32);
        let dy = (y2 as i32 - y1 as i32);

        if dx == 0 {
            if y1 > y2 { swap(&mut y1, &mut y2) }

            for y in y1..y2 {
                self.set_pixel(x1, y, color)
            }
        } else {
            let k = dy as f32 / dx as f32;
            let c = y1 as f32 - k * x1 as f32;

            if x1 > x2 { swap(&mut x1, &mut x2) }

            for x in x1..x2 {
                let y = (k * x as f32 + c);

                self.set_pixel(x, y as usize, color)
            }
        }
    }

    fn check_in_range(&self, x: usize, y: usize) {
        if x >= self.width {
            panic!("x is out of range: {} >= {}", x, self.width)
        }

        if y >= self.height {
            panic!("y is out of range: {} >= {}", y, self.height)
        }
    }
}