use cgpp2::triangle::*;
use image::GenericImageView;
use obj::TexturedVertex;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Mul, Neg, Sub};

pub mod support;

use support::canvas::*;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn with_elements(e: [f32; 3]) -> Vec3 {
        Vec3 { e }
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.len();
        Vec3::new(self.e[0] / len, self.e[1] / len, self.e[2] / len)
    }

    pub fn len(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
        //        self.e.iter().zip(rhs.e.iter()).map(|(a, b)| a * b).sum()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

fn main() {
    let width = 800;
    let height = 800;

    let model_input =
        BufReader::new(File::open("data/african_head.obj").expect("Failed to find obj file"));
    let model: obj::Obj<TexturedVertex> =
        obj::load_obj(model_input).expect("Failed to load obj file");
    let texture =
        image::open("data/african_head_diffuse.tga").expect("Failed to open texture file");

    setup(width, height, |input, canvas| {
        let mut zbuffer = vec![std::f32::MIN; (width * height) as usize];

        let light_dir = Vec3::new(
            (input.mouse.x as f32 / width as f32) * 2.0 - 1.0,
            (input.mouse.y as f32 / height as f32) * 2.0 - 1.0,
            -1.0,
        )
        .normalized();

        let model_to_screen_pos = |p: Vec3| {
            (
                (p.e[0] + 1.0) * (width as f32) / 2.0,
                (p.e[1] + 1.0) * (height as f32) / 2.0,
            )
        };

        for face in model.indices.chunks(3) {
            let v0 = model.vertices[face[0] as usize];
            let v1 = model.vertices[face[1] as usize];
            let v2 = model.vertices[face[2] as usize];

            let p0 = Vec3::with_elements(v0.position);
            let p1 = Vec3::with_elements(v1.position);
            let p2 = Vec3::with_elements(v2.position);

            let n = (p1 - p0).cross(p2 - p0).normalized();
            let intensity = n * -light_dir;

            if intensity > 0.0 {
                let (ax, ay) = model_to_screen_pos(p0);
                let (bx, by) = model_to_screen_pos(p1);
                let (cx, cy) = model_to_screen_pos(p2);

                for p in fill_triangle_iter(ax, ay, bx, by, cx, cy, 0, 0, width - 1, height - 1) {
                    let x = p.x;
                    let y = height - 1 - p.y;
                    let w = Vec3::new(p.b0, p.b1, p.b2);
                    let z = Vec3::new(p0.e[2], p1.e[2], p2.e[2]) * w;

                    if z > zbuffer[(y * width + x) as usize] {
                        zbuffer[(y * width + x) as usize] = z;

                        let u = Vec3::new(v0.texture[0], v1.texture[0], v2.texture[0]) * w;
                        let v = Vec3::new(v0.texture[1], v1.texture[1], v2.texture[1]) * w;
                        let tp = texture.get_pixel(
                            ((u % 1.0) * (texture.width() - 1) as f32).round() as u32,
                            texture.height()
                                - ((v % 1.0) * (texture.height() - 1) as f32).round() as u32,
                        );

                        canvas.set_pixel(
                            x,
                            y,
                            (tp.data[0] as f32 / 255.0) * intensity * p.aa,
                            (tp.data[1] as f32 / 255.0) * intensity * p.aa,
                            (tp.data[2] as f32 / 255.0) * intensity * p.aa,
                            (tp.data[3] as f32 / 255.0) * p.aa,
                        );

                        //                        canvas.set_pixel(
                        //                            x,
                        //                            y,
                        //                            intensity * p.aa,
                        //                            intensity * p.aa,
                        //                            intensity * p.aa,
                        //                            1.0,
                        //                        );
                    }
                }
            }
        }
    });
}
