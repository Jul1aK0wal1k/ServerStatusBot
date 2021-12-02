use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildId {
    #[serde(rename = "_id")]
    pub id: String,
}

impl GuildId {
    pub fn new(id: String) -> Self {
        GuildId { id }
    }
}

impl Display for GuildId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}