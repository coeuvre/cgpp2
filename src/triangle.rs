use crate::types::*;

// Half-Space Rasterization
// TODO: Optimization
pub struct FillTriangleIter {
    v0: Point,
    v1: Point,
    v2: Point,
    minx: i32,
    miny: i32,
    maxx: i32,
    maxy: i32,
    ix: i32,
    iy: i32,
}

impl FillTriangleIter {
    pub fn new(v0: Point, v1: Point, v2: Point, clip: Rect) -> FillTriangleIter {
        // Only support counter-clockwise winding order
        debug_assert!(signed_area(v0, v1, v2) >= 0.0);

        let minx = v0.x.min(v1.x).min(v2.x).floor().max(clip.min.x) as i32;
        let miny = v0.y.min(v1.y).min(v2.y).floor().max(clip.min.y) as i32;
        let maxx = v0.x.max(v1.x).max(v2.x).ceil().min(clip.max.x) as i32;
        let maxy = v0.y.max(v1.y).max(v2.y).ceil().min(clip.max.y) as i32;

        FillTriangleIter {
            v0,
            v1,
            v2,
            minx,
            miny,
            maxx,
            maxy,
            ix: minx,
            iy: miny,
        }
    }
}

impl Iterator for FillTriangleIter {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ix >= self.maxx {
                self.ix = self.minx;
                self.iy += 1;

                if self.iy >= self.maxy {
                    return None;
                }
            }

            let ix = self.ix;
            let iy = self.iy;

            self.ix += 1;

            debug_assert!(ix >= self.minx && ix < self.maxx);
            debug_assert!(iy >= self.miny && iy < self.maxy);

            // Pixel center is at (0.5, 0.5)
            let p = Point::new(ix as f32 + 0.5, iy as f32 + 0.5);
            let w0 = signed_area(self.v1, self.v2, p);
            let w1 = signed_area(self.v2, self.v0, p);
            let w2 = signed_area(self.v0, self.v1, p);
            let b0 = is_top_left(self.v1, self.v2);
            let b1 = is_top_left(self.v2, self.v0);
            let b2 = is_top_left(self.v0, self.v1);

            if (w0 > 0.0 || b0 && w0 == 0.0)
                && (w1 > 0.0 || b1 && w1 == 0.0)
                && (w2 > 0.0 || b2 && w2 == 0.0)
            {
                return Some(Pixel {
                    x: ix,
                    y: iy,
                    aa: 1.0,
                });
            }
        }
    }
}

pub fn fill_triangle_iter(
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    cx: f32,
    cy: f32,
    minx: i32,
    miny: i32,
    maxx: i32,
    maxy: i32,
) -> FillTriangleIter {
    FillTriangleIter::new(
        Point::new(ax, ay),
        Point::new(bx, by),
        Point::new(cx, cy),
        Rect::new(
            Point::new(minx as f32, miny as f32),
            Point::new(maxx as f32, maxy as f32),
        ),
    )
}

fn signed_area(a: Point, b: Point, c: Point) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn is_top_left(a: Point, b: Point) -> bool {
    // In a counter-clockwise triangle, a top edge is an edge that is exactly horizontal,
    // and goes towards the left, i.e. its end point is left of its start point.
    //
    // In a counter-clockwise triangle, a left edge is an edge that goes down,
    // i.e. its end point is strictly below its start point.
    //
    // See https://fgiesen.wordpress.com/2013/02/08/triangle-rasterization-in-practice/
    // See https://docs.microsoft.com/zh-cn/windows/desktop/direct3d11/d3d10-graphics-programming-guide-rasterizer-stage-rules#Triangle
    if a.y == b.y {
        b.x < a.x
    } else {
        b.y < a.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WIDTH: i32 = 1000;
    const HEIGHT: i32 = 1000;

    struct PixelCoordIter {
        pixel: Pixel,
        i: i32,
    }

    impl PixelCoordIter {
        pub fn new(pixel: Pixel) -> PixelCoordIter {
            PixelCoordIter { pixel, i: 0 }
        }
    }

    impl Iterator for PixelCoordIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let result = match self.i {
                0 => Some(self.pixel.x),
                1 => Some(self.pixel.y),
                _ => None,
            };

            self.i += 1;

            result
        }
    }

    fn assert_filled_pixels(points: [f32; 6], pixels: Vec<i32>) {
        let output = fill_triangle_iter(
            points[0], points[1], points[2], points[3], points[4], points[5], 0, 0, WIDTH, HEIGHT,
        )
        .flat_map(|p| PixelCoordIter::new(p))
        .collect::<Vec<_>>();
        assert_eq!(output, pixels);
    }

    #[test]
    fn test_fill_rule1() {
        let points = [4.5, 7.5, 4.5, 7.5, 4.5, 7.5];
        let pixels = vec![];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule2() {
        let points = [1.0, 2.0, 5.0, 2.0, 7.0, 4.0];
        let pixels = vec![2, 2, 3, 2, 4, 2, 5, 3];
        assert_filled_pixels(points, pixels);

        let points = [5.0, 2.0, 8.0, 1.0, 7.0, 4.0];
        let pixels = vec![6, 1, 7, 1, 5, 2, 6, 2, 6, 3];
        assert_filled_pixels(points, pixels);

        let points = [8.0, 1.0, 9.0, 2.5, 7.0, 4.0];
        let pixels = vec![7, 2, 8, 2, 7, 3];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule3() {
        let points = [9.5, 0.5, 9.5, -1.5, 10.5, 0.5];
        let pixels = vec![9, 0];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule4() {
        let points = [11.5, 3.5, 11.5, 1.5, 12.5, 2.5];
        let pixels = vec![11, 2];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule5() {
        let points = [12.5, 2.5, 12.5, 0.5, 14.5, 2.5];
        let pixels = vec![12, 1, 12, 2, 13, 2];
        assert_filled_pixels(points, pixels);

        let points = [14.5, 2.5, 12.5, 0.5, 14.5, 0.5];
        let pixels = vec![13, 1];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule6() {
        let points = [1.0, 7.0, 2.0, 4.0, 6.0, 6.0];
        let pixels = vec![2, 4, 1, 5, 2, 5, 3, 5, 4, 5, 1, 6, 2, 6];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule7() {
        let points = [5.2, 6.8, 6.2, 6.8, 6.2, 7.8];
        let pixels = vec![];
        assert_filled_pixels(points, pixels);

        let points = [6.5, 6.5, 7.5, 6.5, 7.5, 7.5];
        let pixels = vec![];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule8() {
        let points = [7.8, 5.5, 9.5, 2.8, 11.8, 5.5];
        let pixels = vec![9, 3, 8, 4, 9, 4, 10, 4, 8, 5, 9, 5, 10, 5, 11, 5];
        assert_filled_pixels(points, pixels);

        let points = [7.8, 5.5, 11.8, 5.5, 9.8, 7.2];
        let pixels = vec![9, 6, 10, 6];
        assert_filled_pixels(points, pixels);
    }

    #[test]
    fn test_fill_rule9() {
        let points = [13.5, 6.5, 14.5, 3.5, 14.5, 5.5];
        let pixels = vec![];
        assert_filled_pixels(points, pixels);

        let points = [13.5, 6.5, 14.5, 5.5, 15.0, 8.0];
        let pixels = vec![13, 6, 14, 6, 14, 7];
        assert_filled_pixels(points, pixels);
    }
}
