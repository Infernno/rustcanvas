mod rustcanvas;

use rustcanvas::RustCanvas;

fn main() {
    let mut canvas = RustCanvas::new(800, 800);

    canvas.fill(0x12345678);

    let x1 = 400;
    let y1 = 100;

    let x2 = 400;
    let y2 = 400;

    let x3 = 700;
    let y3 = 400;

    canvas.draw_triangle(
        x1, y1,
        x2, y2,
        x3, y3,
        0xFFFFFFFF,
    );

    /*
    canvas.fill_circle(
        x1, y1,
        10,
        0x00FF0000,
    );

    canvas.fill_circle(
        x2, y2,
        10,
        0x00FF0000,
    );

    canvas.fill_circle(
        x3, y3,
        10,
        0x00FF0000,
    );

    canvas.line_to(
        x1, y1,
        x3, y3,
        0x00FF0000,
    );
    canvas.line_to(
        x1, y1,
        x2, y2,
        0x00FF0000,
    );

    canvas.line_to(
        x2, y2,
        x3, y3,
        0x00FF0000,
    );
     */

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
