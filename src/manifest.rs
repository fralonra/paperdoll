use serde::{Deserialize, Serialize};

use crate::{doll::Doll, fragment::Fragment, meta::Meta, slot::Slot};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub meta: Meta,

    pub dolls: Vec<Doll>,
    pub slots: Vec<Slot>,
    pub fragments: Vec<Fragment>,
}
