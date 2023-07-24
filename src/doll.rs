use serde::{Deserialize, Serialize};

use crate::{
    common::{is_zero, Point},
    image::ImageData,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Doll {
    id: u32,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub width: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub height: u32,
    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub offset: Point,

    pub slots: Vec<u32>,

    pub path: String,
    #[serde(skip)]
    pub image: ImageData,
}

impl Doll {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            desc: String::default(),
            width: 400,
            height: 400,
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
