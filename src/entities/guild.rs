use crate::entities::BotSettings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub bot_settings: BotSettings,
}

impl Guild {
    pub fn new(id: String, name: String) -> Self {
        Guild {
            id,
            name,
            bot_settings: BotSettings::default(),
        }
    }
}
