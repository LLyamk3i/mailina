use crate::domain::profile::Profile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Catalog {
    pub profiles: HashMap<String, Profile>,
}

impl Catalog {
    pub fn load() -> Self {
        let raw = crate::io::disk::read("providers.json");
        serde_json::from_str(&raw).unwrap_or_default()
    }

    pub fn find(&self, domain: &str) -> Option<&Profile> {
        self.profiles.get(domain)
    }

    pub fn seed() {
        let mut profiles = HashMap::new();

        profiles.insert(
            String::from("google"),
            Profile {
                host: String::from("imap.gmail.com"),
                port: 993,
                secure: true,
            },
        );

        profiles.insert(
            String::from("protonmail"),
            Profile {
                host: String::from("127.0.0.1"),
                port: 1143,
                secure: false,
            },
        );

        let catalog = Self { profiles };
        let raw = serde_json::to_string_pretty(&catalog).expect("serialization failed");
        crate::io::disk::write("providers.json", &raw);
    }
}
