/// Types of the color used in `paperdoll`.
#[derive(Clone, Copy, Debug, Default)]
pub enum ColorType {
    #[default]
    Rgba,
}

/// The data used in images.
#[derive(Clone, Debug, Default)]
pub struct ImageData {
    /// The width of the image in pixels.
    pub width: u32,
    /// The height of the image in pixels.
    pub height: u32,

    /// Type of the color used in the image.
    pub color_type: ColorType,
    /// The actual pixel data of the image.
    pub pixels: Vec<u8>,
}

impl ImageData {
    /// Is this an empty image?
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
}
