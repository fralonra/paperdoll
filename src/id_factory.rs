use std::collections::HashSet;

use anyhow::{bail, Result};

pub(crate) struct IdFactory {
    cursor: u32,
    set: HashSet<u32>,
}

impl IdFactory {
    pub fn new() -> Self {
        Self {
            cursor: u32::MIN,
            set: HashSet::new(),
        }
    }

    pub fn get_next(&mut self) -> Result<u32> {
        let start = self.cursor;

        while self.set.contains(&self.cursor) {
            self.cursor = self.cursor.wrapping_add(1);

            if self.cursor == start {
                bail!("No enough id left.");
            }
        }

        self.set.insert(self.cursor);

        Ok(self.cursor)
    }

    pub fn remove(&mut self, id: u32) -> bool {
        self.set.remove(&id)
    }

    pub fn take_up(&mut self, id: u32) -> Result<()> {
        if self.set.contains(&id) {
            bail!("Id {} already exists", id);
        }

        self.set.insert(id);

        Ok(())
    }
}
