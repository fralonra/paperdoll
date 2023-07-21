#[derive(Clone, Copy, Debug, Default)]
pub enum ColorType {
    Rgb,
    #[default]
    Rgba,
}

#[derive(Clone, Debug, Default)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,

    pub color_type: ColorType,
    pub pixels: Vec<u8>,
}

impl ImageData {
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
}
