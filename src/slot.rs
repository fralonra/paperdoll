use std::collections::BTreeSet;

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
    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub position: Point,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub width: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub height: u32,

    #[serde(default, skip_serializing_if = "Point::is_zero")]
    pub anchor: Point,

    pub candidates: BTreeSet<u32>,
}

impl Slot {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            desc: String::default(),
            required: false,
            constrainted: false,
            position: Point::default(),
            width: 0,
            height: 0,
            anchor: Point::default(),
            candidates: BTreeSet::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
