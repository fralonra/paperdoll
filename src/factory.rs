use std::collections::{btree_map::Iter, BTreeMap, HashMap};

use anyhow::{anyhow, bail, Result};

use crate::{
    builder::PaperdollBuilder,
    doll::Doll,
    fragment::Fragment,
    id_factory::IdFactory,
    image::ImageData,
    manifest::Manifest,
    meta::Meta,
    paperdoll::Paperdoll,
    render_material::{RenderMaterial, RenderPiece},
    slot::Slot,
};

pub struct PaperdollFactory {
    pub meta: Meta,

    doll_id_factory: IdFactory,
    slot_id_factory: IdFactory,
    fragment_id_factory: IdFactory,

    dolls: BTreeMap<u32, Doll>,
    slots: BTreeMap<u32, Slot>,
    fragments: BTreeMap<u32, Fragment>,
}

impl Default for PaperdollFactory {
    fn default() -> Self {
        Self::new(Meta::default(), vec![], vec![], vec![]).unwrap()
    }
}

impl PaperdollFactory {
    pub fn from_manifest(manifest: Manifest) -> Result<Self> {
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
    ) -> Result<Self> {
        let mut dolls = BTreeMap::new();
        let mut doll_id_factory = IdFactory::new();

        for doll in doll_list {
            doll_id_factory
                .take_up(doll.id())
                .map_err(|e| anyhow!("Add doll with id {} failed: {}", doll.id(), e))?;

            dolls.insert(doll.id(), doll);
        }

        if dolls.len() == 0 {
            let doll = Doll::new(doll_id_factory.get_next()?);

            dolls.insert(doll.id(), doll);
        }

        let mut slots = BTreeMap::new();
        let mut slot_id_factory = IdFactory::new();

        for slot in slot_list {
            slot_id_factory
                .take_up(slot.id())
                .map_err(|e| anyhow!("Add slot with id {} failed: {}", slot.id(), e))?;

            slots.insert(slot.id(), slot);
        }

        let mut fragments = BTreeMap::new();
        let mut fragment_id_factory = IdFactory::new();

        for fragment in fragment_list {
            fragment_id_factory
                .take_up(fragment.id())
                .map_err(|e| anyhow!("Add fragment with id {} failed: {}", fragment.id(), e))?;

            fragments.insert(fragment.id(), fragment);
        }

        Ok(Self {
            meta,

            doll_id_factory,
            slot_id_factory,
            fragment_id_factory,

            dolls,
            slots,
            fragments,
        })
    }

    pub fn add_doll(&mut self) -> Result<u32> {
        let doll = Doll::new(
            self.doll_id_factory
                .get_next()
                .map_err(|e| anyhow!("Add new doll failed: {}", e))?,
        );

        let id = doll.id();

        self.dolls.insert(id, doll);

        Ok(id)
    }

    pub fn add_fragment(&mut self) -> Result<u32> {
        let fragment = Fragment::new(
            self.fragment_id_factory
                .get_next()
                .map_err(|e| anyhow!("Add new fragment failed: {}", e))?,
        );

        let id = fragment.id();

        self.fragments.insert(id, fragment);

        Ok(id)
    }

    pub fn add_slot(&mut self) -> Result<u32> {
        let slot = Slot::new(
            self.slot_id_factory
                .get_next()
                .map_err(|e| anyhow!("Add new slot failed: {}", e))?,
        );

        let id = slot.id();

        self.slots.insert(id, slot);

        Ok(id)
    }

    pub fn builder(&self) -> PaperdollBuilder {
        PaperdollBuilder::new(&self.dolls, &self.slots, &self.fragments)
    }

    pub fn dolls(&self) -> Iter<u32, Doll> {
        self.dolls.iter()
    }

    pub fn fragments(&self) -> Iter<u32, Fragment> {
        self.fragments.iter()
    }

    pub fn get_doll(&self, id: u32) -> Option<&Doll> {
        self.dolls.get(&id)
    }

    pub fn get_doll_mut(&mut self, id: u32) -> Option<&mut Doll> {
        self.dolls.get_mut(&id)
    }

    pub fn get_fragment(&self, id: u32) -> Option<&Fragment> {
        self.fragments.get(&id)
    }

    pub fn get_fragment_mut(&mut self, id: u32) -> Option<&mut Fragment> {
        self.fragments.get_mut(&id)
    }

    pub fn get_slot(&self, id: u32) -> Option<&Slot> {
        self.slots.get(&id)
    }

    pub fn get_slot_mut(&mut self, id: u32) -> Option<&mut Slot> {
        self.slots.get_mut(&id)
    }

    pub fn remove_doll(&mut self, id: u32) -> Option<Doll> {
        if let Some(doll) = self.dolls.remove(&id) {
            self.doll_id_factory.remove(id);

            for slot_id in &doll.slots {
                self.remove_slot(*slot_id);
            }

            Some(doll)
        } else {
            None
        }
    }

    pub fn remove_fragment(&mut self, id: u32) -> Option<Fragment> {
        if let Some(fragment) = self.fragments.remove(&id) {
            self.fragment_id_factory.remove(id);

            for slot in &mut self.slots.values_mut() {
                if let Some(position) = slot
                    .candidates
                    .iter()
                    .position(|fragment_id| *fragment_id == id)
                {
                    slot.candidates.remove(position);
                }
            }

            Some(fragment)
        } else {
            None
        }
    }

    pub fn remove_slot(&mut self, id: u32) -> Option<Slot> {
        if let Some(slot) = self.slots.remove(&id) {
            self.slot_id_factory.remove(id);

            for doll in &mut self.dolls.values_mut() {
                if let Some(position) = doll.slots.iter().position(|slot_id| *slot_id == id) {
                    doll.slots.remove(position);
                }
            }

            Some(slot)
        } else {
            None
        }
    }

    pub fn render(
        &self,
        doll: u32,
        slot_map: &HashMap<u32, u32>,
        only_id: bool,
    ) -> Result<RenderMaterial> {
        let doll = self
            .get_doll(doll)
            .ok_or(anyhow!("Failed to find doll with id {}", doll))?;

        let width = doll.width;
        let height = doll.height;

        let mut slots = vec![];

        for slot_id in &doll.slots {
            let slot = self
                .get_slot(*slot_id)
                .ok_or(anyhow!("Failed to find slot with id {}", slot_id))?;

            if let Some(fragment_id) = slot_map.get(slot_id) {
                let fragment = self
                    .get_fragment(*fragment_id)
                    .ok_or(anyhow!("Failed to find fragment with id {}", fragment_id))?;

                if fragment.image.is_empty() {
                    bail!(
                        "Fragment with id {} is used but it contains no image data",
                        fragment_id
                    );
                }

                for position in &slot.positions {
                    let mut image = ImageData {
                        width: fragment.image.width,
                        height: fragment.image.height,
                        color_type: fragment.image.color_type,
                        ..Default::default()
                    };

                    let position = if slot.constrainted {
                        image.width = slot.width;
                        image.height = slot.height;

                        *position
                    } else {
                        *position + slot.anchor - fragment.pivot
                    };

                    if !only_id {
                        image.pixels = fragment.image.pixels.clone();
                    }

                    slots.push(RenderPiece {
                        id: *fragment_id,
                        position,
                        image,
                    });
                }
            } else {
                if slot.required {
                    bail!(
                        "Slot with id {} requires an image to be drawn but not found",
                        slot_id
                    );
                }
            }
        }

        let doll = (!doll.image.is_empty()).then(|| {
            let image = if only_id {
                ImageData {
                    width: doll.image.width,
                    height: doll.image.height,
                    color_type: doll.image.color_type,
                    ..Default::default()
                }
            } else {
                doll.image.clone()
            };

            RenderPiece {
                id: doll.id(),
                position: doll.offset,
                image,
            }
        });

        Ok(RenderMaterial {
            width,
            height,
            doll,
            slots,
        })
    }

    pub fn render_paperdoll(&self, paperdoll: &Paperdoll, only_id: bool) -> Result<RenderMaterial> {
        self.render(paperdoll.doll, &paperdoll.slot_map, only_id)
    }

    pub fn slots(&self) -> Iter<u32, Slot> {
        self.slots.iter()
    }

    pub fn to_manifest(&self) -> Manifest {
        Manifest {
            meta: self.meta.clone(),
            dolls: self.dolls.values().cloned().collect(),
            slots: self.slots.values().cloned().collect(),
            fragments: self.fragments.values().cloned().collect(),
        }
    }
}
