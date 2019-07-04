use cgpp2::line::*;
use cgpp2::triangle::*;

mod support;

use support::canvas::*;

fn main() {
    setup(|canvas| {
        for p in line_iter(100, 100, 200, 300) {
            canvas.set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
        }

        for p in line_iter(100, 100, 200, 200) {
            canvas.set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
        }

        for p in fill_triangle_iter(
            100.0,
            100.0,
            200.0,
            100.0,
            190.0,
            150.0,
            0,
            0,
            canvas.width(),
            canvas.height(),
        ) {
            canvas.set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
        }

        for p in fill_triangle_iter(
            200.0,
            100.0,
            400.0,
            150.0,
            190.0,
            150.0,
            0,
            0,
            canvas.width(),
            canvas.height(),
        ) {
            canvas.set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
        }
    });
}
