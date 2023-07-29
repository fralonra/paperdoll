use serde::{Deserialize, Serialize};

use crate::common::{is_false, is_zero, Point};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Slot {
    id: u32,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub constrainted: bool,

    /// Slot's top left position in the doll.
    /// One slot can have multiple positions.
    #[serde(default = "default_positions", skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<Point>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub width: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub height: u32,

    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub anchor: Point,

    pub candidates: Vec<u32>,
}

impl Slot {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            desc: String::default(),
            required: false,
            constrainted: false,
            positions: default_positions(),
            width: 0,
            height: 0,
            anchor: Point::default(),
            candidates: vec![],
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

fn default_positions() -> Vec<Point> {
    vec![Point::default()]
}
