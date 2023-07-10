use std::collections::{hash_map::Iter, HashMap};

use crate::{
    builder::PaperdollBuilder,
    doll::{Doll, DollId},
    fragment::{Fragment, FragmentId},
    manifest::Manifest,
    meta::Meta,
    slot::{Slot, SlotId},
};

pub struct PaperdollFactory {
    meta: Meta,

    dolls: HashMap<DollId, Doll>,
    slots: HashMap<SlotId, Slot>,
    fragments: HashMap<FragmentId, Fragment>,
}

impl PaperdollFactory {
    pub fn from_manifest(manifest: Manifest) -> Self {
        Self::new(
            manifest.meta,
            manifest.dolls,
            manifest.slots,
            manifest.fragments,
        )
    }

    pub fn new(
        meta: Meta,
        doll_list: Vec<Doll>,
        slot_list: Vec<Slot>,
        fragment_list: Vec<Fragment>,
    ) -> Self {
        let mut dolls = HashMap::new();

        for doll in doll_list {
            dolls.insert(doll.id, doll);
        }

        let mut slots = HashMap::new();

        for slot in slot_list {
            slots.insert(slot.id, slot);
        }

        let mut fragments = HashMap::new();

        for fragment in fragment_list {
            fragments.insert(fragment.id, fragment);
        }

        Self {
            meta,
            dolls,
            slots,
            fragments,
        }
    }

    pub fn builder(&self) -> PaperdollBuilder {
        PaperdollBuilder::new(&self.dolls, &self.slots, &self.fragments)
    }

    pub fn dolls(&self) -> Iter<DollId, Doll> {
        self.dolls.iter()
    }

    pub fn fragments(&self) -> Iter<FragmentId, Fragment> {
        self.fragments.iter()
    }

    pub fn get_doll(&self, id: DollId) -> Option<&Doll> {
        self.dolls.get(&id)
    }

    pub fn get_fragment(&self, id: FragmentId) -> Option<&Fragment> {
        self.fragments.get(&id)
    }

    pub fn get_slots(&self, id: SlotId) -> Option<&Slot> {
        self.slots.get(&id)
    }

    pub fn slots(&self) -> Iter<SlotId, Slot> {
        self.slots.iter()
    }
}
