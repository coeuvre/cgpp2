use sdl2_sys::*;
use std::ffi::CString;
use std::ptr::{null, null_mut};

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
    pub fn clear(&mut self) {
        unsafe {
            std::ptr::write_bytes(self.pixels, 0, (self.width() * self.height() * 4) as usize);
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, r: f32, g: f32, b: f32, a: f32) {
        debug_assert!(x >= 0 && x < self.width());
        debug_assert!(y >= 0 && y < self.height());
        unsafe {
            let pixel = self.pixels.offset((self.pitch * y + x * 4) as isize);
            *pixel.offset(0) = (((a * 255.0).round() as i32) & 0xFF) as u8;
            *pixel.offset(1) = (((b * 255.0).round() as i32) & 0xFF) as u8;
            *pixel.offset(2) = (((g * 255.0).round() as i32) & 0xFF) as u8;
            *pixel.offset(3) = (((r * 255.0).round() as i32) & 0xFF) as u8;
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

pub struct Input {
    pub mouse: Mouse,
}

pub struct Mouse {
    pub x: i32,
    pub y: i32,
}

pub fn setup<F>(width: i32, height: i32, callback: F)
where
    F: FnMut(&Input, &mut Canvas),
{
    unsafe {
        run(width, height, callback);
    }
}

macro_rules! sdl_error {
    () => {{
        use std::ffi::CStr;
        CStr::from_ptr(SDL_GetError()).to_string_lossy()
    }};
}

unsafe fn run<F>(width: i32, height: i32, mut callback: F)
where
    F: FnMut(&Input, &mut Canvas),
{
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

    let mut input = Input {
        mouse: Mouse { x: 0, y: 0 },
    };

    'game: loop {
        let mut event = std::mem::MaybeUninit::uninit();
        while SDL_PollEvent(event.as_mut_ptr()) != 0 {
            let event = event.assume_init();
            let event_type: SDL_EventType = std::mem::transmute(event.type_);
            match event_type {
                SDL_EventType::SDL_QUIT => break 'game,
                _ => {}
            }
        }

        SDL_GetMouseState(&mut input.mouse.x, &mut input.mouse.y);

        {
            let mut canvas = readonly_canvas.lock();
            callback(&input, &mut canvas);
        }

        SDL_RenderCopy(renderer, texture, null(), null());
        SDL_RenderPresent(renderer);
    }

    SDL_DestroyWindow(window);

    SDL_Quit();
}
