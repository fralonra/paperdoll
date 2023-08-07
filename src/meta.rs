use serde::{Deserialize, Serialize};

use crate::VERSION;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub name: String,
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
