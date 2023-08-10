use serde::{Deserialize, Serialize};

use crate::VERSION;

/// The meta data used in a `paperdoll` project.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    /// The name of the project.
    pub name: String,
    /// The version of paperdoll.
    /// See [crate::VERSION].
    pub version: u32,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: String::default(),
            version: VERSION,
        }
    }
}
