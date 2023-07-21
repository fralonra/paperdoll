use crate::{common::Point, image::ImageData};

pub struct RenderMaterial {
    pub width: u32,
    pub height: u32,
    pub doll: Option<RenderPiece>,
    pub slots: Vec<RenderPiece>,
}

pub struct RenderPiece {
    pub id: u32,
    pub position: Point,
    pub image: ImageData,
}
