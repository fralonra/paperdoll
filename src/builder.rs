use std::collections::HashMap;

use crate::{
    doll::{Doll, DollId},
    fragment::{Fragment, FragmentId},
    paperdoll::Paperdoll,
    slot::{Slot, SlotId},
};

pub struct PaperdollBuilder<'a> {
    pub doll: DollId,

    pub slot_map: HashMap<SlotId, FragmentId>,

    dolls: &'a HashMap<DollId, Doll>,
    slots: &'a HashMap<SlotId, Slot>,
    fragments: &'a HashMap<FragmentId, Fragment>,
}

impl<'a> PaperdollBuilder<'a> {
    pub fn new(
        dolls: &'a HashMap<DollId, Doll>,
        slots: &'a HashMap<SlotId, Slot>,
        fragments: &'a HashMap<FragmentId, Fragment>,
    ) -> Self {
        Self {
            doll: DollId::default(),
            slot_map: HashMap::new(),
            dolls,
            slots,
            fragments,
        }
    }

    pub fn doll(mut self, id: DollId) -> Self {
        if !self.dolls.contains_key(&id) {
            panic!("Invalid key for doll: {}", id);
        }

        self.doll = id;
        self
    }

    pub fn set_slot(mut self, slot_id: SlotId, fragment_id: FragmentId) -> Self {
        if !self.slots.contains_key(&slot_id) {
            panic!("Invalid key for slot: {}", slot_id);
        }

        if !self.fragments.contains_key(&fragment_id) {
            panic!("Invalid key for fragment: {}", fragment_id);
        }

        self.slot_map.insert(slot_id, fragment_id);
        self
    }

    pub fn build(self) -> Paperdoll {
        Paperdoll {
            doll: self.doll,
            slot_map: self.slot_map,
        }
    }
}
