use sdl2_sys::*;
use std::ffi::CString;
use std::ptr::{null, null_mut};

use cgpp2::line::*;
use cgpp2::triangle::*;

struct ReadonlyCanvas {
    width: i32,
    height: i32,
    texture: *mut SDL_Texture,
}

impl ReadonlyCanvas {
    pub fn new(width: i32, height: i32, texture: *mut SDL_Texture) -> ReadonlyCanvas {
        ReadonlyCanvas {
            width,
            height,
            texture,
        }
    }

    pub fn lock(&mut self) -> Canvas {
        let mut pixels = null_mut();
        let mut pitch = 0;
        unsafe {
            SDL_LockTexture(self.texture, null_mut(), &mut pixels, &mut pitch);
        }
        Canvas {
            data: self,
            pixels: pixels as *mut u8,
            pitch,
        }
    }
}

pub struct Canvas<'a> {
    data: &'a mut ReadonlyCanvas,
    pixels: *mut u8,
    pitch: i32,
}

impl<'a> Canvas<'a> {
    pub fn set_pixel(&mut self, x: i32, y: i32, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            let pixel = self.pixels.offset((self.pitch * y + x * 4) as isize);
            *pixel.offset(0) = (((a * 255.0).round() as i32) & 0xFF) as u8;
            *pixel.offset(1) = (((b * 255.0).round() as i32) & 0xFF) as u8;
            *pixel.offset(2) = (((g * 255.0).round() as i32) & 0xFF) as u8;
            *pixel.offset(3) = (((r * 255.0).round() as i32) & 0xFF) as u8;
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        for p in line_iter(x0, y0, x1, y1) {
            self.set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
        }
    }

    pub fn fill_triangle(&mut self, ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) {
        for p in fill_triangle_iter(ax, ay, bx, by, cx, cy, 0, 0, self.width(), self.height()) {
            self.set_pixel(p.x, p.y, 1.0 * p.aa, 1.0 * p.aa, 1.0 * p.aa, 1.0);
        }
    }

    pub fn width(&self) -> i32 {
        self.data.width
    }

    pub fn height(&self) -> i32 {
        self.data.height
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        unsafe {
            SDL_UnlockTexture(self.data.texture);
        }
    }
}

pub fn setup<F>(callback: F)
where
    F: Fn(&mut Canvas),
{
    unsafe {
        run(callback);
    }
}

macro_rules! sdl_error {
    () => {{
        use std::ffi::CStr;
        CStr::from_ptr(SDL_GetError()).to_string_lossy()
    }};
}

unsafe fn run<F>(callback: F)
where
    F: Fn(&mut Canvas),
{
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

    let mut readonly_canvas = ReadonlyCanvas::new(width, height, texture);

    'game: loop {
        let mut event = std::mem::uninitialized::<SDL_Event>();
        while SDL_PollEvent(&mut event) != 0 {
            let event_type: SDL_EventType = std::mem::transmute(event.type_);
            match event_type {
                SDL_EventType::SDL_QUIT => break 'game,
                _ => {}
            }

            {
                let mut canvas = readonly_canvas.lock();
                callback(&mut canvas);
            }

            SDL_RenderClear(renderer);
            SDL_RenderCopy(renderer, texture, null(), null());
            SDL_RenderPresent(renderer);
        }
    }

    SDL_DestroyWindow(window);

    SDL_Quit();
}
