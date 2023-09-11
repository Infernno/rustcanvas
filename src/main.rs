mod rustcanvas;

use rustcanvas::RustCanvas;

fn main() {
    let mut canvas = RustCanvas::new(200, 200);

    canvas.clear(0x123456);

    // Red cross
    canvas.line_to(
        0, 0,
        canvas.xMax(), canvas.yMax(),
        0xFF0000,
    );

    canvas.line_to(
        canvas.xMax(), 0,
        0, canvas.yMax(),
        0xFF0000,
    );

    // Green horizontal & vertical lines
    canvas.line_to(
        canvas.centerX(), 0,
        canvas.centerX(), canvas.yMax(),
        0x00FF00,
    );

    canvas.line_to(
        0, canvas.centerY(),
        canvas.xMax(), canvas.centerY(),
        0x00FF00,
    );

    canvas.fill_circle(
        canvas.centerX(),
        canvas.centerY(),
        10,
        0xFFFFFF
    );

    canvas.save_ppm("image.ppm").unwrap();
}
