use std::collections::{BTreeMap, HashMap};

use crate::{doll::Doll, fragment::Fragment, paperdoll::Paperdoll, slot::Slot};

pub struct PaperdollBuilder<'a> {
    doll: u32,

    slot_map: HashMap<u32, u32>,

    dolls: &'a BTreeMap<u32, Doll>,
    slots: &'a BTreeMap<u32, Slot>,
    fragments: &'a BTreeMap<u32, Fragment>,
}

impl<'a> PaperdollBuilder<'a> {
    pub fn new(
        dolls: &'a BTreeMap<u32, Doll>,
        slots: &'a BTreeMap<u32, Slot>,
        fragments: &'a BTreeMap<u32, Fragment>,
    ) -> Self {
        Self {
            doll: u32::default(),
            slot_map: HashMap::new(),
            dolls,
            slots,
            fragments,
        }
    }

    pub fn doll(mut self, id: u32) -> Self {
        if !self.dolls.contains_key(&id) {
            panic!("Invalid key for doll: {}", id);
        }

        self.doll = id;
        self
    }

    pub fn set_slot(mut self, slot_id: u32, fragment_id: u32) -> Self {
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
