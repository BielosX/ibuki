extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::{WindowSurfaceRef, Window};

use sdl2_sys::SDL_PixelFormat;
use sdl2_sys::SDL_PixelFormatEnum;
use sdl2_sys::SDL_Palette;
use sdl2_sys::SDL_Surface;

struct Context {
    window: Window,
    event_pump: EventPump
}

fn create_context() -> Result<Context, String> {
    let sdl_context = sdl2::init()?;
    let event_pump = sdl_context.event_pump()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("ostwind", 800, 600)
        .position_centered()
        .build().map_err(|x| -> String {x.to_string()})?;
    Ok(Context {window, event_pump})
}

fn draw(context: &mut Context) {
    let mut quit = false;
    while !quit {
        for event in context.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => quit = true,
                _ => {}
            }
        }
        let window_surface = context.window.surface(&context.event_pump).expect("Error");
        let surface: *mut SDL_Surface = window_surface.raw();
        unsafe {
            let pixel_format: *mut SDL_PixelFormat = (*surface).format;
            let pixels = (*surface).pixels as *mut u32;
            for n in 0..(800*600) {
                (*pixels.offset(n)) = 0;
            }
            for n in 0..800 {
                (*pixels.offset(n + 800*200)) = 255 << (*pixel_format).Rshift;
            }
        }
        window_surface.update_window();
    }
}

fn main() {
    let init_result = create_context();
    match init_result {
        Ok(mut context) => draw(&mut context),
        Err(err) => println!("Error occured during context init: {}", err),
    }
}
