use crate::pixel::Pixel;
use std::mem::swap;

// Half-Space Rasterization
// TODO: Optimization
pub struct FillTriangleIter {
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
    ix: i32,
    iy: i32,
    acx: f32,
    acy: f32,
    bax: f32,
    bay: f32,
    cbx: f32,
    cby: f32,
}

impl FillTriangleIter {
    pub fn new(ax: f32, ay: f32, mut bx: f32, mut by: f32, mut cx: f32, mut cy: f32) -> FillTriangleIter {
        let minx = ax.min(bx).min(cx).floor() as i32;
        let miny = ay.min(by).min(cy).floor() as i32;
        let maxx = ax.max(bx).max(cx).ceil() as i32;
        let maxy = ay.max(by).max(cy).ceil() as i32;
        let (acx, acy) = vec(ax, ay, cx, cy);
        let (bax, bay) = vec(bx, by, ax, ay);
        let (cbx, cby) = vec(cx, cy, bx, by);
        FillTriangleIter {
            ax,
            ay,
            bx,
            by,
            cx,
            cy,
            minx,
            miny,
            maxx,
            maxy,
            ix: minx,
            iy: miny,
            acx,
            acy,
            bax,
            bay,
            cbx,
            cby,
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

            let px = ix as f32;
            let py = iy as f32;
            let (apx, apy) = vec(self.ax, self.ay, px, py);
            let (bpx, bpy) = vec(self.bx, self.by, px, py);
            let (cpx, cpy) = vec(self.cx, self.cy, px, py);
            let c1 = perp_dot(self.acx, self.acy, apx, apy);
            let c2 = perp_dot(self.bax, self.bay, bpx, bpy);
            let c3 = perp_dot(self.cbx, self.cby, cpx, cpy);

            if c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0 || c1 <= 0.0 && c2 <= 0.0 && c3 <= 0.0{
                return Some(Pixel { x: ix, y: iy, aa: 1.0 });
            }
        }
    }
}

pub fn fill_triangle_iter(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> FillTriangleIter {
    FillTriangleIter::new(ax, ay, bx, by, cx, cy)
}

fn perp_dot(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    ax * by - ay * bx
}

fn vec(ax: f32, ay: f32, bx: f32, by: f32) -> (f32, f32) {
    (bx - ax, by - ay)
}