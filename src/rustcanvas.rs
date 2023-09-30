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

    pub fn max_x(&self) -> usize {
        self.width - 1
    }

    pub fn max_y(&self) -> usize {
        self.height - 1
    }

    pub fn max_x_i32(&self) -> i32 {
        self.max_x() as i32
    }

    pub fn max_y_i32(&self) -> i32 {
        self.max_y() as i32
    }

    pub fn center_x(&self) -> usize { self.max_x() / 2 }

    pub fn center_y(&self) -> usize { self.max_y() / 2 }

    pub fn center_x_i32(&self) -> i32 { self.max_x_i32() / 2 }

    pub fn center_y_i32(&self) -> i32 { self.max_y_i32() / 2 }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        // println!("Set {x}, {y} ({}, {}) with {color}", self.width, self.height);

        let index = y * self.width + x;

        if index < self.pixels.len() {
            self.pixels[index] = color;
        } else {
            println!("x = {}, y = {} is out of range", x, y);
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        let index = y * self.width + x;

        if index < self.pixels.len() {
            return Some(self.pixels[index]);
        } else {
            println!("x = {}, y = {} is out of range", x, y);
        }

        return None;
    }

    pub fn save_ppm(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        // Write PPM header
        write!(file, "P6\n{0} {1} 255\n", self.width, self.height)?;

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
    pub fn fill(&mut self, color: u32) {
        for pixel in &mut self.pixels {
            *pixel = color
        }
    }

    pub fn fill_rect(
        &mut self,
        x1: i32,
        y1: i32,
        width: i32,
        height: i32,
        color: u32,
    ) {
        self.ensure_in_range(x1, y1);

        let x2 = x1 + width;
        let y2 = y1 + height;

        for x in x1..=x2 {
            if x >= self.width as i32 { continue; }

            for y in y1..=y2 {
                if y >= self.height as i32 { continue; }

                self.set_pixel(x as usize, y as usize, color);
            }
        }
    }

    pub fn fill_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: u32,
    ) {
        self.ensure_in_range(center_x, center_y);

        let x1 = center_x - radius;
        let y1 = center_y - radius;

        let x2 = center_x + radius;
        let y2 = center_y + radius;

        let r2 = radius * radius;

        for x in x1..=x2 {
            for y in y1..=y2 {
                let dx = x - center_x;
                let dy = y - center_y;

                if dx * dx + dy * dy <= r2 {
                    self.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }

    pub fn line_to(&mut self,
                   mut x1: i32,
                   mut y1: i32,
                   mut x2: i32,
                   mut y2: i32,
                   color: u32) {
        self.ensure_in_range(x1, y1);
        self.ensure_in_range(x2, y2);

        // y = k * x + c

        // y1 = k * x1 + c
        // y2 = k * x2 + c

        // c = y1 - k * x1
        // k = (y2 - y1) / (x2 - x1)

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0 {
            if y1 > y2 { swap(&mut y1, &mut y2) }

            for y in y1..=y2 {
                self.set_pixel(x1 as usize, y as usize, color)
            }
        } else {
            let c = y1 - (dy * x1) / dx;

            if x1 > x2 { swap(&mut x1, &mut x2) }

            for x in x1..=x2 {
                let mut py = ((dy * x) / dx + c);
                let mut ny = ((dy * (x + 1)) / dx + c);

                if py > ny { swap(&mut py, &mut ny); }

                for y in py..=ny {
                    if y >= 0 && y <= self.max_y_i32() {
                        self.set_pixel(x as usize, y as usize, color)
                    } else {
                        println!("Outside of range ({}, {}) (py = {}, ny = {}, x1 = {}, y1 = {}, x2 = {}, y2 = {})", x, y, py, ny, x1, y1, x2, y2)
                    }
                }
            }
        }
    }

    fn ensure_in_range(&self, x: i32, y: i32) {
        if x >= self.width as i32 {
            panic!("x is out of range: {} >= {}", x, self.width)
        }

        if y >= self.height as i32 {
            panic!("y is out of range: {} >= {}", y, self.height)
        }
    }
}

impl RustCanvas {
    pub fn draw_triangle(
        &mut self,
        mut x1: i32,
        mut y1: i32,
        mut x2: i32,
        mut y2: i32,
        mut x3: i32,
        mut y3: i32,
        color: u32,
    ) {
        if y1 > y2 {
            swap(&mut y1, &mut y2);
            swap(&mut x1, &mut x2);
        }

        if y2 > y3 {
            swap(&mut y2, &mut y3);
            swap(&mut x2, &mut x3);
        }

        if y1 > y3 {
            swap(&mut y1, &mut y3);
            swap(&mut x1, &mut x3);
        }

        for y in y1..=y2 {
            let mut sx1 = RustCanvas::get_x_at(y, x1, y1, x2, y2);
            let mut sx2 = RustCanvas::get_x_at(y, x1, y1, x3, y3);

            if sx1 > sx2 { swap(&mut x1, &mut x2); }

            for x in sx1..=sx2 {
                self.set_pixel(x as usize, y as usize, color)
            }
        }

        for y in y2..=y3 {
            let mut sx1 = RustCanvas::get_x_at(y, x2, y2, x3, y3);
            let mut sx2 = RustCanvas::get_x_at(y, x1, y1, x3, y3);

            if sx1 > sx2 { swap(&mut x1, &mut x2); }

            for x in sx1..=sx2 {
                self.set_pixel(x as usize, y as usize, color)
            }
        }
    }

    fn get_x_at(
        y: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> i32 {
        let dy = y2 - y1;

        if dy == 0 {
            return x1;
        }

        let dx = x2 - x1;
        let x = (y - y1) * dx / dy + x1;

        return x;
    }
}