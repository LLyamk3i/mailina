use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub host: String,
    pub port: u16,
    pub secure: bool,
}
