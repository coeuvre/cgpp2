use std::fs::File;
use std::io::BufReader;

use cgpp2::triangle::*;
use cgpp2::types::*;

use image::{DynamicImage, GenericImageView};

use obj::TexturedVertex;

pub mod support;

use support::canvas::*;

pub trait Varying {
    fn as_vec(&self) -> Vec<f32>;
    fn from_slice(slice: &[f32]) -> Self;
}

trait VertexShader<A, U> {
    type V: Varying;

    fn process(&self, attribute: &A, uniform: &U) -> VertexShaderOutput<Self::V>;
}

pub struct VertexShaderOutput<V: Varying> {
    pub pos: Vec4,
    pub varying: V,
}

pub struct FragmentShaderOutput {
    pub color: Option<Vec4>,
}

trait FragmentShader<V: Varying, U> {
    fn process(&self, varying: &V, uniform: &U) -> FragmentShaderOutput;
}

fn ndc_to_screen(p: Vec3, width: i32, height: i32) -> Vec3 {
    Vec3::new(
        (p.e[0] + 1.0) * (width as f32) / 2.0,
        (p.e[1] + 1.0) * (height as f32) / 2.0,
        (-p.e[2] + 1.0) / 2.0,
    )
}

fn render<A, V: Varying, U>(
    vertices: Vec<A>,
    uniform: &U,
    vs: &dyn VertexShader<A, U, V = V>,
    fs: &dyn FragmentShader<V, U>,
    width: i32,
    height: i32,
    canvas: &mut Canvas,
) {
    let mut zbuffer = vec![std::f32::MIN; (width * height) as usize];
    let processed_vertices = vertices
        .iter()
        .map(|vertex| vs.process(vertex, uniform))
        .map(|vo| {
            let ndc = vo.pos.perspective_division();
            let screen_pos = ndc_to_screen(ndc, width, height);
            (screen_pos, vo.varying)
        })
        .collect::<Vec<(Vec3, V)>>();

    for triangle in processed_vertices.chunks(3) {
        let (a, av) = &triangle[0];
        let (b, bv) = &triangle[1];
        let (c, cv) = &triangle[2];

        for p in fill_triangle_iter(
            a.e[0],
            a.e[1],
            b.e[0],
            b.e[1],
            c.e[0],
            c.e[1],
            0,
            0,
            width - 1,
            height - 1,
        ) {
            let w = Vec3::new(p.b0, p.b1, p.b2);
            let x = p.x;
            let y = height - 1 - p.y;
            let z = Vec3::new(a.e[2], b.e[2], c.e[2]) * w;
            if z > zbuffer[(y * width + x) as usize] {
                zbuffer[(y * width + x) as usize] = z;
                let wv = av
                    .as_vec()
                    .iter()
                    .zip(bv.as_vec().iter())
                    .zip(cv.as_vec().iter())
                    .map(|((a, b), c)| Vec3::new(*a, *b, *c) * w)
                    .collect::<Vec<f32>>();
                let v = V::from_slice(&wv);
                let fo = fs.process(&v, uniform);
                if let Some(color) = fo.color {
                    let color = p.aa * color;
                    canvas.set_pixel(x, y, color.e[0], color.e[1], color.e[2], color.e[3]);
                }
            }
        }
    }

    // dump z-buffer
    //    for y in 0..height {
    //        for x in 0..width {
    //            let z = zbuffer[(y * width + x) as usize];
    //            canvas.set_pixel(x, y, z, z, z, 1.0);
    //        }
    //    }
}

struct MyAttribute {
    pub v: TexturedVertex,
}

struct MyVarying {
    u: f32,
    v: f32,
    intensity: f32,
}

impl Varying for MyVarying {
    fn as_vec(&self) -> Vec<f32> {
        vec![self.u, self.v, self.intensity]
    }

    fn from_slice(slice: &[f32]) -> Self {
        MyVarying {
            u: slice[0],
            v: slice[1],
            intensity: slice[2],
        }
    }
}

struct MyUniform<'a> {
    mvp: Mat4,
    mvp_normal: Mat4,
    light_dir_transformed: Vec3,
    texture: &'a DynamicImage,
}

struct MyVertexShader {}

fn calc_intensity(n: Vec3, light_dir: Vec3) -> f32 {
    n * (light_dir - 2.0 * (light_dir * n) * n)
}

impl<'a> VertexShader<MyAttribute, MyUniform<'a>> for MyVertexShader {
    type V = MyVarying;

    fn process(&self, attribute: &MyAttribute, uniform: &MyUniform) -> VertexShaderOutput<Self::V> {
        let v = attribute.v;
        let p = Vec4::from_vec3(Vec3::with_elements(v.position), 1.0);
        let c = uniform.mvp * p;
        let intensity = calc_intensity(
            (uniform.mvp_normal * Vec4::from_vec3(Vec3::with_elements(v.normal), 0.0))
                .xyz()
                .normalized(),
            uniform.light_dir_transformed,
        );
        VertexShaderOutput {
            pos: c,
            varying: MyVarying {
                u: v.texture[0],
                v: v.texture[1],
                intensity,
            },
        }
    }
}

struct MyFragmentShader {}

impl<'a> FragmentShader<MyVarying, MyUniform<'a>> for MyFragmentShader {
    fn process(&self, varying: &MyVarying, uniform: &MyUniform) -> FragmentShaderOutput {
        let color = if varying.intensity > 0.0 {
            let u = varying.u;
            let v = varying.v;
            assert!(u >= 0.0 && u <= 1.0);
            assert!(v >= 0.0 && v <= 1.0);

            let texture = &uniform.texture;
            let tp = texture.get_pixel(
                (u * (texture.width() - 1) as f32).round() as u32,
                ((1.0 - v) * (texture.height() - 1) as f32).round() as u32,
            );

            Some(Vec4::new(
                (tp.data[0] as f32 / 255.0) * varying.intensity,
                (tp.data[1] as f32 / 255.0) * varying.intensity,
                (tp.data[2] as f32 / 255.0) * varying.intensity,
                tp.data[3] as f32 / 255.0,
            ))
        } else {
            None
        };
        FragmentShaderOutput { color }
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

    let mut rotation = 0.0;

    setup(width, height, |_input, canvas| {
        canvas.clear();

        let camera = Mat4::look_at(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let projection = Mat4::frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 3.0);

        let model_transform = Mat4::rorate_y(rotation);
        rotation += 0.01;

        let vp = projection * camera;
        let vp_normal = vp.transpose().inverse().unwrap();

        let mvp = vp * model_transform;
        let mvp_normal = mvp.transpose().inverse().unwrap();

        let light_dir = Vec3::new(0.0, 0.0, -1.0).normalized();
        let light_dir_transformed = (vp_normal * Vec4::from_vec3(light_dir, 0.0))
            .xyz()
            .normalized();

        let vertices = model
            .indices
            .iter()
            .map(|index| MyAttribute {
                v: model.vertices[*index as usize],
            })
            .collect();

        let uniform = MyUniform {
            mvp,
            mvp_normal,
            light_dir_transformed,
            texture: &texture,
        };

        let vs = MyVertexShader {};
        let fs = MyFragmentShader {};

        render(vertices, &uniform, &vs, &fs, width, height, canvas)
    });

    /*
    setup(width, height, |_input, canvas| {
        canvas.clear();

        let camera = Mat4::look_at(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let projection = Mat4::frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 3.0);

        let model_transform = Mat4::rorate_y(rotation);
        rotation += 0.01;

        let vp = projection * camera;
        let vp_normal = vp.transpose().inverse().unwrap();

        let mvp = vp * model_transform;
        let mvp_normal = mvp.transpose().inverse().unwrap();

        let mut zbuffer = vec![std::f32::MIN; (width * height) as usize];

        let light_dir = Vec3::new(0.0, 0.0, -1.0).normalized();
        let light_dir_transformed = (vp_normal * Vec4::from_vec3(light_dir, 0.0))
            .xyz()
            .normalized();

        for face in model.indices.chunks(3) {
            let v0 = model.vertices[face[0] as usize];
            let v1 = model.vertices[face[1] as usize];
            let v2 = model.vertices[face[2] as usize];

            let p0 = Vec4::from_vec3(Vec3::with_elements(v0.position), 1.0);
            let p1 = Vec4::from_vec3(Vec3::with_elements(v1.position), 1.0);
            let p2 = Vec4::from_vec3(Vec3::with_elements(v2.position), 1.0);

            let c0 = mvp * p0;
            let c1 = mvp * p1;
            let c2 = mvp * p2;

            //            let i0 = calc_intensity(Vec3::with_elements(v0.normal), light_dir);
            //            let i1 = calc_intensity(Vec3::with_elements(v1.normal), light_dir);
            //            let i2 = calc_intensity(Vec3::with_elements(v2.normal), light_dir);
            let i0 = calc_intensity(
                (mvp_normal * Vec4::from_vec3(Vec3::with_elements(v0.normal), 0.0))
                    .xyz()
                    .normalized(),
                light_dir_transformed,
            );
            let i1 = calc_intensity(
                (mvp_normal * Vec4::from_vec3(Vec3::with_elements(v1.normal), 0.0))
                    .xyz()
                    .normalized(),
                light_dir_transformed,
            );
            let i2 = calc_intensity(
                (mvp_normal * Vec4::from_vec3(Vec3::with_elements(v2.normal), 0.0))
                    .xyz()
                    .normalized(),
                light_dir_transformed,
            );

            let (ax, ay, az) = ndc_to_screen(c0.perspective_division());
            let (bx, by, bz) = ndc_to_screen(c1.perspective_division());
            let (cx, cy, cz) = ndc_to_screen(c2.perspective_division());

            for p in fill_triangle_iter(ax, ay, bx, by, cx, cy, 0, 0, width - 1, height - 1) {
                let w = Vec3::new(p.b0, p.b1, p.b2);
                let intensity = Vec3::new(i0, i1, i2) * w;
                if intensity > 0.0 {
                    let x = p.x;
                    let y = height - 1 - p.y;
                    let z = Vec3::new(az, bz, cz) * w;

                    if z > zbuffer[(y * width + x) as usize] {
                        zbuffer[(y * width + x) as usize] = z;

                        let u = Vec3::new(v0.texture[0], v1.texture[0], v2.texture[0]) * w;
                        let v = Vec3::new(v0.texture[1], v1.texture[1], v2.texture[1]) * w;
                        assert!(u >= 0.0 && u <= 1.0);
                        assert!(v >= 0.0 && v <= 1.0);
                        let tp = texture.get_pixel(
                            (u * (texture.width() - 1) as f32).round() as u32,
                            ((1.0 - v) * (texture.height() - 1) as f32).round() as u32,
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

        // dump z-buffer
        //        for y in 0..height {
        //            for x in 0..width {
        //                let z = zbuffer[(y * width + x) as usize];
        //                canvas.set_pixel(x, y, z, z, z, 1.0);
        //            }
        //        }
    });
    */
}
