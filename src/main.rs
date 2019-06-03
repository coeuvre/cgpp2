#![windows_subsystem = "windows"]

use std::mem::{size_of, zeroed};
use std::ptr::null_mut;

// use winapi::shared::guiddef::LPCGUID;
use winapi::shared::minwindef::*;
//use winapi::shared::mmreg::*;
use winapi::shared::windef::*;
//use winapi::shared::winerror::{ERROR_DEVICE_NOT_CONNECTED, ERROR_SUCCESS, SUCCEEDED};
//use winapi::um::dsound::*;
use winapi::um::libloaderapi::*;
use winapi::um::memoryapi::*;
//use winapi::um::profileapi::*;
//use winapi::um::unknwnbase::LPUNKNOWN;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;

//use winapi::um::xinput::*;

static mut GLOBAL_BACK_BUFFER: *mut Win32OffScreenBuffer = 0 as *mut Win32OffScreenBuffer;

macro_rules! wcstring {
    ($s:expr) => {{
        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new($s)
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<u16>>()
    }};
}

fn main() {
    unsafe {
        let mut back_buffer = zeroed::<Win32OffScreenBuffer>();
        GLOBAL_BACK_BUFFER = &mut back_buffer;

        run();
    }
}

static mut RUNNING: bool = false;

unsafe fn run() {
    win32_resize_dib_section(&mut *GLOBAL_BACK_BUFFER, 1280, 720);

    let instance = GetModuleHandleW(null_mut());

    let mut window_class = zeroed::<WNDCLASSW>();
    let class_name = wcstring!("CGPP2_WINDOW_CLASS");
    window_class.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    window_class.lpfnWndProc = Some(win32_main_window_proc);
    window_class.hInstance = instance;
    window_class.lpszClassName = class_name.as_ptr();

    let result = RegisterClassW(&window_class);
    if result == 0 {
        panic!("Failed to register class");
    }

    let window_title = wcstring!("cgpp2");
    let window = CreateWindowExW(
        0,
        class_name.as_ptr(),
        window_title.as_ptr(),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        1280,
        720,
        0 as HWND,
        0 as HMENU,
        instance,
        null_mut(),
    );

    if window == 0 as HWND {
        panic!("Failed to create window");
    }

    ShowWindow(window, SW_SHOW);

    RUNNING = true;

    let mut message = zeroed::<MSG>();
    while RUNNING && GetMessageW(&mut message, 0 as HWND, 0, 0) != 0 {
        if message.message == WM_QUIT {
            RUNNING = false;
        }

        TranslateMessage(&message);
        DispatchMessageW(&message);
    }
}

unsafe extern "system" fn win32_main_window_proc(
    window: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_DESTROY => {
            RUNNING = false;
            println!("WM_DESTORY");
        }
        WM_CLOSE => {
            RUNNING = false;
            println!("WM_CLOSE");
        }
        WM_ACTIVATEAPP => {
            println!("WM_ACTIVATEAPP");
        }
        //        WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
        //            let vk_code = wparam as i32;
        //            let was_down = (lparam & (1 << 30)) != 0;
        //            let is_down = (lparam & (1 << 31)) != 0;
        //            if was_down != is_down {
        //                match vk_code as u8 as char {
        //                    'W' => {}
        //                    'A' => {}
        //                    'S' => {}
        //                    'D' => {}
        //                    'Q' => {}
        //                    'E' => {}
        //                    _ => match vk_code {
        //                        VK_UP => {}
        //                        VK_LEFT => {}
        //                        VK_DOWN => {}
        //                        VK_RIGHT => {}
        //                        VK_ESCAPE => {}
        //                        VK_SPACE => {}
        //                        _ => {}
        //                    },
        //                }
        //            }
        //
        //            let alt_key_was_down = lparam & (1 << 29);
        //            if is_down && (vk_code == VK_ESCAPE || alt_key_was_down != 0 && vk_code == VK_F4) {
        //                RUNNING = false;
        //            }
        //        }
        WM_PAINT => {
            render();

            let mut ps = zeroed::<PAINTSTRUCT>();
            let device_context = BeginPaint(window, &mut ps);
            // let dimension = win32_get_window_dimension(window);
            let buffer = &mut *GLOBAL_BACK_BUFFER;
            win32_display_buffer_in_window(
                device_context,
                buffer.width,
                buffer.height,
                &mut *GLOBAL_BACK_BUFFER,
            );
            EndPaint(window, &mut ps);
        }
        _ => return DefWindowProcW(window, message, wparam, lparam),
    }

    0
}

fn render() {
    for x in 0..100 {
        for y in 0..100 {
            put_pixel(x, y, 255, 0, 255);
        }
    }
}

unsafe fn win32_resize_dib_section(buffer: &mut Win32OffScreenBuffer, width: i32, height: i32) {
    if buffer.memory != null_mut() {
        VirtualFree(buffer.memory, 0 as usize, MEM_RELEASE);
    }

    buffer.width = width;
    buffer.height = height;
    let bytes_per_pixel = 4;

    buffer.info.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    buffer.info.bmiHeader.biWidth = buffer.width;
    buffer.info.bmiHeader.biHeight = -buffer.height;
    buffer.info.bmiHeader.biPlanes = 1;
    buffer.info.bmiHeader.biBitCount = 32;
    buffer.info.bmiHeader.biCompression = BI_RGB;

    let bitmap_memory_size = buffer.width * buffer.height * bytes_per_pixel;
    buffer.memory = VirtualAlloc(
        null_mut(),
        bitmap_memory_size as usize,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );
    buffer.pitch = buffer.width * bytes_per_pixel;
}

// struct Win32WindowDimension {
//     width: i32,
//     height: i32,
// }

// unsafe fn win32_get_window_dimension(window: HWND) -> Win32WindowDimension {
//     let mut client_rect = zeroed::<RECT>();
//     GetClientRect(window, &mut client_rect);
//     let width = client_rect.right - client_rect.left;
//     let height = client_rect.bottom - client_rect.top;
//     return Win32WindowDimension { width, height };
// }

fn put_pixel(x: i32, y: i32, r: u8, g: u8, b: u8) {
    let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    unsafe {
        let buffer = &mut *GLOBAL_BACK_BUFFER;
        *((buffer.memory as *mut u8).offset((y * buffer.pitch + x * 4) as isize) as *mut u32) =
            color;
    }
}

struct Win32OffScreenBuffer {
    info: BITMAPINFO,
    memory: LPVOID,
    width: i32,
    height: i32,
    pitch: i32,
}

unsafe fn win32_display_buffer_in_window(
    device_context: HDC,
    window_width: i32,
    window_height: i32,
    buffer: &Win32OffScreenBuffer,
) {
    StretchDIBits(
        device_context,
        0,
        0,
        window_width,
        window_height,
        0,
        0,
        buffer.width,
        buffer.height,
        buffer.memory,
        &buffer.info,
        DIB_RGB_COLORS,
        SRCCOPY,
    );
}
