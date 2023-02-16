mod polygon;
mod raw_canvas;
mod draw;
mod point2d;
mod line;

extern crate sdl2;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{WindowSurfaceRef, Window};

use sdl2_sys::SDL_PixelFormat;
use sdl2_sys::SDL_Surface;

use raw_canvas::RawCanvas;
use raw_canvas::PixelColor;
use line::Line;
use point2d::Point2d;
use crate::draw::Draw;

struct Context {
    window: Window,
    event_pump: EventPump
}

fn create_context() -> Result<Context, String> {
    let sdl_context = sdl2::init()?;
    let event_pump = sdl_context.event_pump()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("ibuki", 800, 600)
        .position_centered()
        .build().map_err(|x| -> String {x.to_string()})?;
    Ok(Context {window, event_pump})
}

fn draw(context: &mut Context, drawables: &Vec<Box<dyn Draw>>) {
    let mut quit = false;
    while !quit {
        for event in context.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => quit = true,
                _ => {}
            }
        }
        let window_surface = context.window.surface(&context.event_pump).expect("Error");
        let raw_canvas = RawCanvas::new(&window_surface);
        raw_canvas.clean_color(&PixelColor::black());
        for drawable in drawables.iter() {
            drawable.draw(&raw_canvas);
        }
        window_surface.update_window().expect("Unable to update window");
    }
}

fn main() {
    let init_result = create_context();
    let mut drawables: Vec<Box<dyn Draw>> = Vec::new();
    //drawables.push(Box::new(Point2d {x: 5.2, y: 7.8}));
    drawables.push(Box::new(Line::new(30.0, 500.3, 40.0, 10.0)));
    drawables.push(Box::new(Line::new(30.0, 20.3, 40.0, 500.0)));
    drawables.push(Box::new(Line::new(30.0, 20.3, 500.0, 70.0)));
    drawables.push(Box::new(Line::new(30.0, 500.3, 600.0, 10.0)));
    drawables.push(Box::new(Line::new(30.0, 500.3, 600.0, 500.3)));
    drawables.push(Box::new(Line::new(700.0, 500.3, 700.0, 20.3)));
    match init_result {
        Ok(mut context) => draw(&mut context, &drawables),
        Err(err) => println!("Error occurred during context init: {}", err),
    }
}
