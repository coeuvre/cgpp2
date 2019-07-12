use std::fs::File;
use std::io::BufReader;

pub mod support;

use std::ops::{Mul, Neg, Sub};
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
        self.e.iter().zip(rhs.e.iter()).map(|(a, b)| a * b).sum()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

fn main() {
    let input =
        BufReader::new(File::open("data/african_head.obj").expect("Failed to find obj file"));
    let model: obj::Obj = obj::load_obj(input).expect("Failed to load obj file");

    setup(|input, canvas| {
        let width = canvas.width();
        let height = canvas.height();

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
            let v0 = Vec3::with_elements(model.vertices[face[0] as usize].position);
            let v1 = Vec3::with_elements(model.vertices[face[1] as usize].position);
            let v2 = Vec3::with_elements(model.vertices[face[2] as usize].position);

            let n = (v1 - v0).cross(v2 - v0).normalized();
            let intensity = n * -light_dir;

            if intensity > 0.0 {
                let (ax, ay) = model_to_screen_pos(v0);
                let (bx, by) = model_to_screen_pos(v1);
                let (cx, cy) = model_to_screen_pos(v2);

                canvas.fill_triangle(ax, ay, bx, by, cx, cy, intensity, intensity, intensity, 1.0);
            }
        }
    });
}
