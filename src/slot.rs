use serde::{Deserialize, Serialize};

use crate::common::{is_false, is_zero, Point};

/// Areas where the paper doll can have alternative styles.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Slot {
    id: u32,

    /// The description of the slot.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,

    /// If false, this slot can be empty.
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,

    /// Whether to use constrainted mode for this slot.
    /// [Read more on the crate's page](crate).
    #[serde(default, skip_serializing_if = "is_false")]
    pub constrainted: bool,

    /// Slot's top left position in the [doll](crate::Doll).
    ///
    /// One slot can have multiple positions.
    #[serde(default = "default_positions", skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<Point>,

    /// The width of the slot in pixels.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub width: u32,

    /// The height of the slot in pixels.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub height: u32,

    /// The coordinate of the anchor point.
    /// The top left corner of the slot is the origin.
    ///
    /// Used in non-constrainted mode.
    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub anchor: Point,

    /// A list of id of [fragments](crate::Fragment) those work as candidates in the slot.
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
