use serde::{Deserialize, Serialize};

use crate::common::Point;

pub type FragmentId = u32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Fragment {
    pub id: FragmentId,

    pub anchor: Point,
    pub path: String,
}
