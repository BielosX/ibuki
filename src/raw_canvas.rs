use sdl2::video::WindowSurfaceRef;
use sdl2_sys::{SDL_PixelFormat, SDL_Surface};

pub struct RawCanvas {
    width: u32,
    height: u32,
    pixels: *mut u32,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
    alpha_shift: u8
}

pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl PixelColor {
    pub fn red() -> PixelColor {
        PixelColor { red: 255, green: 0, blue: 0, alpha: 0 }
    }

    pub fn black() -> PixelColor {
        PixelColor { red: 0, green: 0, blue: 0, alpha: 0 }
    }
}

impl RawCanvas {
    pub fn new(window_surface: &WindowSurfaceRef) -> RawCanvas {
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

    pub fn calculate_color_value(&self, color: &PixelColor) -> u32 {
        (color.red as u32) << (self.red_shift as u32) |
            (color.blue as u32) << (self.blue_shift as u32) |
            (color.green as u32) << (self.green_shift as u32) |
            (color.alpha as u32) << (self.alpha_shift as u32)
    }

    pub fn clean_color(&self, color: &PixelColor) {
        for n in 0..(self.width * self.height) {
            unsafe {
                (*self.pixels.offset(n as isize)) = self.calculate_color_value(&color);
            }
        }
    }

    pub fn put_pixel(&self, x: u32, y: u32, color: &PixelColor) {
        let offset: isize = ((self.height - y - 1) * self.width + x) as isize;
        unsafe {
            (*self.pixels.offset(offset)) = self.calculate_color_value(color);
        }
    }
}