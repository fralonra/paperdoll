use serde::{Deserialize, Serialize};

use crate::{
    common::{is_zero, Point},
    image::ImageData,
};

/// The fundamental part of the paper doll model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Doll {
    id: u32,

    /// The description of the doll.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,

    /// The width of the doll in pixels.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub width: u32,

    /// The height of the doll in pixels.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub height: u32,

    /// The offset of the background image of the doll.
    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub offset: Point,

    /// A list of id of [slots](crate::Slot) those can be used in the doll.
    pub slots: Vec<u32>,

    /// The path of the background image.
    ///
    /// Leave empty if no background.
    pub path: String,

    /// The data of the background image.
    #[serde(skip)]
    pub image: ImageData,
}

impl Doll {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            desc: String::default(),
            width: 0,
            height: 0,
            offset: Point::default(),
            slots: vec![],
            path: String::default(),
            image: ImageData::default(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
