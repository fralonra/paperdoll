use serde::{Deserialize, Serialize};

use crate::{common::Point, fragment::FragmentId};

pub type SlotId = u32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Slot {
    pub id: SlotId,

    pub desc: Option<String>,
    pub required: bool,
    pub anchor: Point,

    pub candidates: Vec<FragmentId>,
}
