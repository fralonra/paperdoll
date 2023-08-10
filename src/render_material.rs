use crate::{common::Point, image::ImageData};

/// An intermediate representation that describes the structure of a paper doll.
pub struct RenderMaterial {
    /// The width of the paper doll in pixels.
    pub width: u32,
    /// The height of the paper doll in pixels.
    pub height: u32,
    /// The `RenderPiece` for the doll to be displayed, if any.
    pub doll: Option<RenderPiece>,
    /// The `RenderPiece` for all slots in this doll.
    pub slots: Vec<RenderPiece>,
}

/// Describes a unit of work for rendering textures.
/// Currently for dolls and fragments that needs to be displayed.
pub struct RenderPiece {
    /// The id. The same as the id of the doll or the fragment.
    pub id: u32,
    /// The top left position of this texture.
    /// The top left corner of the doll is the origin.
    pub position: Point,
    // The image data of the texture.
    pub image: ImageData,
}
