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

pub struct Vec4 {
    pub e: [f32; 4],
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { e: [x, y, z, w] }
    }

    pub fn from_vec3(vec3: Vec3, w: f32) -> Vec4 {
        Vec4 {
            e: [vec3.e[0], vec3.e[1], vec3.e[2], w],
        }
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.e[0], self.e[1], self.e[2])
    }
}

impl Vec4 {
    pub fn perspective_division(&self) -> Vec3 {
        Vec3::new(
            self.e[0] / self.e[3],
            self.e[1] / self.e[3],
            self.e[2] / self.e[3],
        )
    }
}

#[derive(Copy, Clone)]
pub struct Mat4 {
    // Row-major order
    pub e: [f32; 16],
}

impl Mat4 {
    pub fn with_elements(e: [f32; 16]) -> Mat4 {
        Mat4 { e }
    }

    pub fn identity() -> Mat4 {
        let mut e = [0.0; 16];
        e[0] = 1.0;
        e[5] = 1.0;
        e[10] = 1.0;
        e[15] = 1.0;
        Mat4 { e }
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        Mat4::with_elements([
            self.e[0] * rhs.e[0]
                + self.e[1] * rhs.e[4]
                + self.e[2] * rhs.e[8]
                + self.e[3] * rhs.e[12],
            self.e[0] * rhs.e[1]
                + self.e[1] * rhs.e[5]
                + self.e[2] * rhs.e[9]
                + self.e[3] * rhs.e[13],
            self.e[0] * rhs.e[2]
                + self.e[1] * rhs.e[6]
                + self.e[2] * rhs.e[10]
                + self.e[3] * rhs.e[14],
            self.e[0] * rhs.e[3]
                + self.e[1] * rhs.e[7]
                + self.e[2] * rhs.e[11]
                + self.e[3] * rhs.e[15],
            self.e[4] * rhs.e[0]
                + self.e[5] * rhs.e[4]
                + self.e[6] * rhs.e[8]
                + self.e[7] * rhs.e[12],
            self.e[4] * rhs.e[1]
                + self.e[5] * rhs.e[5]
                + self.e[6] * rhs.e[9]
                + self.e[7] * rhs.e[13],
            self.e[4] * rhs.e[2]
                + self.e[5] * rhs.e[6]
                + self.e[6] * rhs.e[10]
                + self.e[7] * rhs.e[14],
            self.e[4] * rhs.e[3]
                + self.e[5] * rhs.e[7]
                + self.e[6] * rhs.e[11]
                + self.e[7] * rhs.e[15],
            self.e[8] * rhs.e[0]
                + self.e[9] * rhs.e[4]
                + self.e[10] * rhs.e[8]
                + self.e[11] * rhs.e[12],
            self.e[8] * rhs.e[1]
                + self.e[9] * rhs.e[5]
                + self.e[10] * rhs.e[9]
                + self.e[11] * rhs.e[13],
            self.e[8] * rhs.e[2]
                + self.e[9] * rhs.e[6]
                + self.e[10] * rhs.e[10]
                + self.e[11] * rhs.e[14],
            self.e[8] * rhs.e[3]
                + self.e[9] * rhs.e[7]
                + self.e[10] * rhs.e[11]
                + self.e[11] * rhs.e[15],
            self.e[12] * rhs.e[0]
                + self.e[13] * rhs.e[4]
                + self.e[14] * rhs.e[8]
                + self.e[15] * rhs.e[12],
            self.e[12] * rhs.e[1]
                + self.e[13] * rhs.e[5]
                + self.e[14] * rhs.e[9]
                + self.e[15] * rhs.e[13],
            self.e[12] * rhs.e[2]
                + self.e[13] * rhs.e[6]
                + self.e[14] * rhs.e[10]
                + self.e[15] * rhs.e[14],
            self.e[12] * rhs.e[3]
                + self.e[13] * rhs.e[7]
                + self.e[14] * rhs.e[11]
                + self.e[15] * rhs.e[15],
        ])
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.e[0] * rhs.e[0]
                + self.e[1] * rhs.e[1]
                + self.e[2] * rhs.e[2]
                + self.e[3] * rhs.e[3],
            self.e[4] * rhs.e[0]
                + self.e[5] * rhs.e[1]
                + self.e[6] * rhs.e[2]
                + self.e[7] * rhs.e[3],
            self.e[8] * rhs.e[0]
                + self.e[9] * rhs.e[1]
                + self.e[10] * rhs.e[2]
                + self.e[11] * rhs.e[3],
            self.e[12] * rhs.e[0]
                + self.e[13] * rhs.e[1]
                + self.e[14] * rhs.e[2]
                + self.e[15] * rhs.e[3],
        )
    }
}

impl Mat4 {
    pub fn translate(t: Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.e[3] = t.e[0];
        m.e[7] = t.e[1];
        m.e[11] = t.e[2];
        m
    }
    pub fn scale(s: Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.e[0] = s.e[0];
        m.e[5] = s.e[1];
        m.e[10] = s.e[2];
        m
    }

    pub fn from_basis(i: Vec3, j: Vec3, k: Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.e[0] = i.e[0];
        m.e[4] = i.e[1];
        m.e[8] = i.e[2];
        m.e[1] = j.e[0];
        m.e[5] = j.e[1];
        m.e[9] = j.e[2];
        m.e[2] = k.e[0];
        m.e[6] = k.e[1];
        m.e[10] = k.e[2];
        m
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let z = (eye - target).normalized();
        let x = up.cross(z).normalized();
        let y = z.cross(x).normalized();
        Mat4::from_basis(x, y, z) * Mat4::translate(-eye)
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4::with_elements([
            2.0 * near / (right - left),
            0.0,
            (right + left) / (right - left),
            0.0,
            0.0,
            2.0 * near / (top - bottom),
            (top + bottom) / (top - bottom),
            0.0,
            0.0,
            0.0,
            -(far + near) / (far - near),
            -(2.0 * far * near) / (far - near),
            0.0,
            0.0,
            -1.0,
            0.0,
        ])
    }
}
