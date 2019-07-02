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

            let p = Point::new(ix as f32, iy as f32);
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
    use std::collections::HashMap;

    const WIDTH: i32 = 1000;
    const HEIGHT: i32 = 1000;

    fn assert_no_overlapped<I1, I2>(p1: I1, p2: I2)
    where
        I1: Iterator<Item = Pixel>,
        I2: Iterator<Item = Pixel>,
    {
        let mut counts = HashMap::new();
        for p in p1.chain(p2) {
            *counts.entry((p.x, p.y)).or_insert(0u32) += 1u32;
        }

        for (_, count) in counts {
            assert_eq!(count, 1);
        }
    }

    #[test]
    fn test_fill_rule() {
        let p1 = fill_triangle_iter(
            100.0, 100.0, 200.0, 100.0, 190.0, 150.0, 0, 0, WIDTH, HEIGHT,
        );
        let p2 = fill_triangle_iter(
            200.0, 100.0, 400.0, 150.0, 190.0, 150.0, 0, 0, WIDTH, HEIGHT,
        );
        assert_no_overlapped(p1, p2);
    }
}
