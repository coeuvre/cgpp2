use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
    pub aa: f32,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new(min: Point, max: Point) -> Rect {
        Rect { min, max }
    }
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn one() -> Vec3 {
        Vec3 { e: [1.0, 1.0, 1.0] }
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

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
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

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}
