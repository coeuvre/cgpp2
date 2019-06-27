use sdl2_sys::*;
use std::ffi::CString;
use std::ptr::{null, null_mut};

mod pixel;
mod line;
mod triangle;

use crate::line::*;
use crate::triangle::*;

fn main() {
    unsafe {
        run();
    }
}

macro_rules! sdl_error {
    () => {{
        use std::ffi::CStr;
        CStr::from_ptr(SDL_GetError()).to_string_lossy()
    }};
}

unsafe fn run() {
    let width = 1280;
    let height = 720;

    if SDL_Init(SDL_INIT_VIDEO) != 0 {
        panic!("Failed to init SDL {}", sdl_error!());
    }

    let title = CString::new("cgpp2").unwrap();
    let window = SDL_CreateWindow(
        title.as_ptr(),
        SDL_WINDOWPOS_UNDEFINED_MASK as i32,
        SDL_WINDOWPOS_UNDEFINED_MASK as i32,
        width,
        height,
        SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
    );
    if window.is_null() {
        panic!("Failed to create SDL window {}", sdl_error!());
    }

    let renderer = SDL_CreateRenderer(
        window,
        -1,
        SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32
            | SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32,
    );
    if renderer.is_null() {
        panic!("Failed to create SDL renderer {}", sdl_error!());
    }

    let texture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_RGBA8888 as u32,
        SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32,
        width,
        height,
    );
    if texture.is_null() {
        panic!("Failed to create SDL texture {}", sdl_error!());
    }

    'game: loop {
        let mut event = std::mem::uninitialized::<SDL_Event>();
        while SDL_PollEvent(&mut event) != 0 {
            let event_type: SDL_EventType = std::mem::transmute(event.type_);
            match event_type {
                SDL_EventType::SDL_QUIT => break 'game,
                _ => {}
            }

            {
                let mut pixels = null_mut();
                let mut pitch = 0;
                SDL_LockTexture(texture, null_mut(), &mut pixels, &mut pitch);

                let pixels = pixels as *mut u8;

                let set_pixel = |x: i32, y: i32, r: f32, g: f32, b: f32, a: f32| {
                    let pixel = pixels.offset((pitch * y + x * 4) as isize);
                    *pixel.offset(0) = (((a * 255.0).round() as i32) & 0xFF) as u8;
                    *pixel.offset(1) = (((b * 255.0).round() as i32) & 0xFF) as u8;
                    *pixel.offset(2) = (((g * 255.0).round() as i32) & 0xFF) as u8;
                    *pixel.offset(3) = (((r * 255.0).round() as i32) & 0xFF) as u8;
                };

                for p in line_iter(100, 100, 200, 300) {
                    set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
                }

                for p in line_iter(100, 100, 200, 200) {
                    set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
                }

                for p in fill_triangle_iter(100.0, 100.0, 200.0, 100.0, 190.0, 150.0) {
                    set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
                }

                SDL_UnlockTexture(texture);
            }

            SDL_RenderClear(renderer);
            SDL_RenderCopy(renderer, texture, null(), null());
            SDL_RenderPresent(renderer);
        }
    }

    SDL_DestroyWindow(window);

    SDL_Quit();
}
