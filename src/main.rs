mod rustcanvas;

use rustcanvas::RustCanvas;

fn main() {
    let mut canvas = RustCanvas::new(200, 200);

    canvas.clear(0x123456);

    /*
    canvas.fill_rect(
        0, 0, 10, canvas.width(),
        0xFFFFFF,
    );

    canvas.fill_rect(
        10, 0, 20, canvas.width(),
        0x0000FF,
    );

    canvas.fill_rect(
        20, 0, 30, canvas.width(),
        0xFF0000,
    );
     */

    canvas.line_to(
        0, 0,
        canvas.width(), canvas.height(),
        0xFF0000
    );

    canvas.line_to(
        canvas.width() / 2, 0,
        50, 50,
        0xFFFFFF
    );


    canvas.line_to(
        canvas.width() / 2, 0,
        canvas.width() / 2, canvas.height(),
        0x00FF00
    );

    canvas.save_to_ppm("image.ppm").unwrap();
}
