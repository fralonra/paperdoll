use std::collections::HashMap;

use crate::{doll::DollId, fragment::FragmentId, slot::SlotId};

pub struct Paperdoll {
    pub doll: DollId,

    pub slot_map: HashMap<SlotId, FragmentId>,
}

impl Paperdoll {}
