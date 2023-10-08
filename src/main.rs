mod rustcanvas;

use std::arch::aarch64::vrsqrte_f32;
use std::f32::consts::PI;
use rustcanvas::RustCanvas;

fn main() {
    let mut canvas = RustCanvas::new(800, 800);

    canvas.fill(0x12345678);

    let cx = canvas.center_x() as i32;
    let cy = canvas.center_y() as i32;

    let x = cx;
    let y = cy - 200;

    canvas.fill_circle(
        cx, cy,
        15,
        0x0000FF00,
    );

    canvas.fill_circle(
        x, y,
        30,
        0x00FF0000,
    );

    let distance: f32 = f32::sqrt((x as f32 - cx as f32).powf(2.0) + (y as f32 - cy as f32).powf(2.0));
    let angle: f32 = (f32::atan2(x as f32, y as f32)) + 0.0;

    let nx: i32 = (distance.round() * f32::cos(angle).round()) as i32 + cx;
    let ny: i32 = (distance.round() * f32::sin(angle).round()) as i32 + cy;

    canvas.fill_circle(
        nx, ny,
        30,
        0x00FF0000,
    );

    canvas.save_ppm("result.ppm").unwrap();
}

fn main2() {
    let mut canvas = RustCanvas::new(800, 800);

    canvas.fill(0x12345678);

    canvas.line_to(0, 0, canvas.max_x_i32(), canvas.max_y_i32(), 0xFFFFFFFF);
    canvas.line_to(canvas.max_x_i32(), 0, 0, canvas.max_y_i32(), 0xFFFFFFFF);
    canvas.line_to(canvas.center_x_i32(), 0, canvas.center_x_i32(), canvas.max_y_i32(), 0xFFFFFFFF);
    canvas.line_to(0, canvas.center_y_i32(), canvas.max_x_i32(), canvas.center_y_i32(), 0xFFFFFFFF);

    canvas.line_to(0, 0, canvas.max_x_i32() / 4, canvas.max_y_i32(), 0x00FF0000);
    canvas.line_to(0, canvas.max_y_i32(), canvas.max_x_i32() / 4, 0, 0x00FF0000);

    canvas.line_to(canvas.max_x_i32(), 0, canvas.max_x_i32() * 3 / 4, canvas.max_y_i32(), 0x00FF0000);
    canvas.line_to(canvas.max_x_i32(), canvas.max_y_i32(), canvas.max_x_i32() * 3 / 4, 0, 0x00FF0000);

    canvas.save_ppm("result.ppm").unwrap();
}
