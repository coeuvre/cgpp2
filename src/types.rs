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
