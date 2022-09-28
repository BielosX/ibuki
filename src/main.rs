extern crate sdl2;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{WindowSurfaceRef, Window};

use sdl2_sys::SDL_PixelFormat;
use sdl2_sys::SDL_Surface;

struct Context {
    window: Window,
    event_pump: EventPump
}

struct RawCanvas {
    width: u32,
    height: u32,
    pixels: *mut u32,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
    alpha_shift: u8
}

struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl PixelColor {
    fn red() -> PixelColor {
        PixelColor { red: 255, green: 0, blue: 0, alpha: 0 }
    }

    fn black() -> PixelColor {
        PixelColor { red: 0, green: 0, blue: 0, alpha: 0 }
    }
}

impl RawCanvas {
    fn new(window_surface: &WindowSurfaceRef) -> RawCanvas {
        let surface: *mut SDL_Surface = window_surface.raw();
        let format: *mut SDL_PixelFormat = unsafe { (*surface).format };
        let pixels = unsafe { (*surface).pixels as *mut u32 };
        let red_shift = unsafe { (*format).Rshift };
        let green_shift = unsafe { (*format).Gshift };
        let blue_shift = unsafe { (*format).Bshift };
        let alpha_shift = unsafe { (*format).Ashift };
        let width = window_surface.width();
        let height = window_surface.height();
        RawCanvas {
            width,
            height,
            pixels,
            red_shift,
            green_shift,
            blue_shift,
            alpha_shift
        }
    }

    fn calculate_color_value(&self, color: &PixelColor) -> u32 {
        (color.red as u32) << (self.red_shift as u32) |
            (color.blue as u32) << (self.blue_shift as u32) |
            (color.green as u32) << (self.green_shift as u32) |
            (color.alpha as u32) << (self.alpha_shift as u32)
    }

    fn clean_color(&self, color: &PixelColor) {
        for n in 0..(self.width * self.height) {
            unsafe {
                (*self.pixels.offset(n as isize)) = self.calculate_color_value(&color);
            }
        }
    }

    fn put_pixel(&self, x: u32, y: u32, color: &PixelColor) {
        let offset: isize = ((self.height - y - 1) * self.width + x) as isize;
        unsafe {
            (*self.pixels.offset(offset)) = self.calculate_color_value(color);
        }
    }
}

trait Draw {
    fn draw(&self, canvas: &RawCanvas);
}

struct Point2d {
    x: f32,
    y: f32
}

impl Draw for Point2d {
    fn draw(&self, canvas: &RawCanvas) {
        let x = self.x as u32;
        let y = self.y as u32;
        canvas.put_pixel(x, y, &PixelColor::red());
    }
}

struct Line {
    first: Point2d,
    last: Point2d
}

impl Line {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line {
            first: Point2d {x: x1, y: y1},
            last: Point2d {x: x2, y: y2}
        }
    }
}

impl Draw for Line {
    fn draw(&self, canvas: &RawCanvas) {
        let delta_y = self.last.y - self.first.y;
        let delta_x = self.last.x - self.first.x;
        let first_x = self.first.x as u32;
        let first_y = self.first.y as u32;
        let last_x = self.last.x as u32;
        let last_y = self.last.y as u32;
        let slope = delta_y / delta_x;
        let mut x = first_x;
        let mut y = first_y;
        if slope > 1.0 {
            let mut denominator = 2.0 * delta_y - delta_x;
            for _ in 0..(last_y - first_y) {
                canvas.put_pixel(x, y, &PixelColor::red());
                if denominator >= 0.0 {
                    denominator -= 2.0 * delta_x;
                } else {
                    x+= 1;
                    denominator += 2.0 * delta_y - 2.0 * delta_x;
                }
                y += 1;
            }
        } else if slope > 0.0 {
            let mut denominator = 2.0 * delta_y - delta_x;
            for _ in 0..(last_x - first_x) {
                canvas.put_pixel(x, y, &PixelColor::red());
                if denominator >= 0.0 {
                    y += 1;
                    denominator += 2.0 * delta_y - 2.0 * delta_x;
                } else {
                    denominator += 2.0 * delta_y;
                }
                x += 1;
            }
        } else if slope < -1.0 {
            let mut denominator = delta_y + 2.0 * delta_x;
            for _ in 0..(first_y - last_y) {
                canvas.put_pixel(x, y, &PixelColor::red());
                if denominator >= 0.0 {
                    x += 1;
                    denominator += 2.0 * delta_y + 2.0 * delta_x;
                } else {
                    denominator += 2.0 * delta_x;
                }
                y -= 1;
            }
        } else {
            let mut denominator = 2.0 * delta_y +  delta_x;
            for _ in 0..(last_x - first_x) {
                canvas.put_pixel(x, y, &PixelColor::red());
                if denominator >= 0.0 {
                    denominator += 2.0 * delta_y;
                } else {
                    y -= 1;
                    denominator += 2.0 * delta_x;
                }
                x += 1;
            }
        }
    }
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
    match init_result {
        Ok(mut context) => draw(&mut context, &drawables),
        Err(err) => println!("Error occurred during context init: {}", err),
    }
}
