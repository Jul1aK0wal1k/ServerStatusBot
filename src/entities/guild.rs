use crate::entities::{Address, BotSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub addresses: Vec<Address>,
    pub settings: BotSettings,
}

impl Guild {
    pub fn new(id: String, name: String) -> Self {
        Guild {
            id,
            name,
            addresses: vec![],
            settings: BotSettings::default(),
        }
    }
}
