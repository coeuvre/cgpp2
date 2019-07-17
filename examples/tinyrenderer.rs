use std::fs::File;
use std::io::BufReader;

use cgpp2::triangle::*;
use cgpp2::types::*;

use image::GenericImageView;

use obj::TexturedVertex;

pub mod support;

use support::canvas::*;

fn main() {
    let width = 800;
    let height = 800;

    let model_input =
        BufReader::new(File::open("data/african_head.obj").expect("Failed to find obj file"));
    let model: obj::Obj<TexturedVertex> =
        obj::load_obj(model_input).expect("Failed to load obj file");
    let texture =
        image::open("data/african_head_diffuse.tga").expect("Failed to open texture file");

    let ndc_to_screen = |p: Vec3| {
        (
            (p.e[0] + 1.0) * (width as f32) / 2.0,
            (p.e[1] + 1.0) * (height as f32) / 2.0,
            (-p.e[2] + 1.0) / 2.0,
        )
    };

    let calc_intensity = |n: Vec3, light_dir: Vec3| n * (light_dir - 2.0 * (light_dir * n) * n);

    setup(width, height, |_input, canvas| {
        let camera = Mat4::look_at(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let projection = Mat4::frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 3.0);

        let model_transform = Mat4::translate(Vec3::new(0.0, 0.0, 0.0));

        let model_to_camera = camera * model_transform;

        let model_to_clip = projection * model_to_camera;

        let mut zbuffer = vec![std::f32::MIN; (width * height) as usize];

        let light_dir = Vec3::new(0.0, 0.0, -1.0).normalized();

        for face in model.indices.chunks(3) {
            let v0 = model.vertices[face[0] as usize];
            let v1 = model.vertices[face[1] as usize];
            let v2 = model.vertices[face[2] as usize];

            let p0 = Vec4::from_vec3(Vec3::with_elements(v0.position), 1.0);
            let p1 = Vec4::from_vec3(Vec3::with_elements(v1.position), 1.0);
            let p2 = Vec4::from_vec3(Vec3::with_elements(v2.position), 1.0);

            let c0 = model_to_clip * p0;
            let c1 = model_to_clip * p1;
            let c2 = model_to_clip * p2;

            let i0 = calc_intensity(Vec3::with_elements(v0.normal), light_dir);
            let i1 = calc_intensity(Vec3::with_elements(v1.normal), light_dir);
            let i2 = calc_intensity(Vec3::with_elements(v2.normal), light_dir);

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
                        debug_assert!(u >= 0.0 && u <= 1.0);
                        debug_assert!(v >= 0.0 && v <= 1.0);
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
}
