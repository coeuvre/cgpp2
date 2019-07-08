use std::fs::File;
use std::io::BufReader;

mod support;

use support::canvas::*;

fn main() {
    let input =
        BufReader::new(File::open("data/african_head.obj").expect("Failed to find obj file"));
    let model: obj::Obj = obj::load_obj(input).expect("Failed to load obj file");

    setup(|canvas| {
        let width = canvas.width();
        let height = canvas.height();
        for face in model.indices.chunks(3) {
            let v0: obj::Vertex = model.vertices[face[0] as usize];
            let v1: obj::Vertex = model.vertices[face[1] as usize];
            let v2: obj::Vertex = model.vertices[face[2] as usize];

            let mut draw_line = |a: obj::Vertex, b: obj::Vertex| {
                canvas.draw_line(
                    ((a.position[0] + 1.0) * (width as f32) / 2.0).round() as i32,
                    ((a.position[1] + 1.0) * (height as f32) / 2.0).round() as i32,
                    ((b.position[0] + 1.0) * (width as f32) / 2.0).round() as i32,
                    ((b.position[1] + 1.0) * (height as f32) / 2.0).round() as i32,
                );
            };

            draw_line(v0, v1);
            draw_line(v1, v2);
            draw_line(v2, v0);
        }
    });
}
