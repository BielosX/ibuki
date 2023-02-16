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