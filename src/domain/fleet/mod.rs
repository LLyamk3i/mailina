pub mod compose;
pub mod survey;

use crate::domain::Credential;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Fleet {
    pub units: Vec<Credential>,
}

impl Fleet {
    pub fn load() -> Self {
        let raw = crate::io::disk::read("fleet.json");
        serde_json::from_str(&raw).unwrap_or_default()
    }

    pub fn save(&self) {
        let raw = serde_json::to_string_pretty(self).expect("serialization failed");
        crate::io::disk::write("fleet.json", &raw);
    }
}
