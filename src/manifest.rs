use serde::{Deserialize, Serialize};

use crate::{doll::Doll, fragment::Fragment, meta::Meta, slot::Slot};

/// A manifest for a `paperdoll` project.
///
/// Serves as an entry point to everything used in the model.
/// Including dolls, slots, and fragments.
#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    /// The meta data of the project.
    pub meta: Meta,

    /// All the dolls in the project.
    pub dolls: Vec<Doll>,
    /// All the slots in the project.
    pub slots: Vec<Slot>,
    /// All the fragments in the project.
    pub fragments: Vec<Fragment>,
}
