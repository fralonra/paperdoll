use serde::{Deserialize, Serialize};

use crate::{common::Point, image::ImageData};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fragment {
    id: u32,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,
    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub pivot: Point,

    pub path: String,
    #[serde(skip)]
    pub image: ImageData,
}

impl Fragment {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            desc: String::default(),
            pivot: Point::default(),
            path: String::default(),
            image: ImageData::default(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
