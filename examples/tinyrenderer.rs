mod support;

use support::canvas::*;

fn main() {
    setup(|canvas| {
        canvas.draw_line(100, 100, 200, 300);
        canvas.draw_line(100, 100, 200, 200);

        canvas.fill_triangle(100.0, 100.0, 200.0, 100.0, 190.0, 150.0);
        canvas.fill_triangle(200.0, 100.0, 400.0, 150.0, 190.0, 150.0);
    });
}
