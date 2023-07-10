use serde::{Deserialize, Serialize};

use crate::slot::SlotId;

pub type DollId = u32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Doll {
    pub id: DollId,

    pub desc: String,

    pub slots: Vec<SlotId>,
}
