use std::collections::HashMap;

/// A paper doll model.
///
/// See [`crate::PaperdollFactory`] for examples.
pub struct Paperdoll {
    /// The id of [doll](crate::Doll) to use.
    pub doll: u32,

    /// A map with the id of [slot](crate::Slot) as key and the id of [fragment](crate::Fragment) which is used in this slot as value.
    pub slot_map: HashMap<u32, u32>,
}
