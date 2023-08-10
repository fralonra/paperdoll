use serde::{Deserialize, Serialize};

use crate::{common::Point, image::ImageData};

/// The image assets that you can put into a slot as candidates.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fragment {
    id: u32,

    /// The description of the fragments.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,

    /// The coordinate of the pivot point.
    /// The top left corner of the fragment is the origin.
    ///
    /// Used in non-constrainted mode.
    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub pivot: Point,

    /// The path of the image.
    pub path: String,

    /// The data of the image.
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
