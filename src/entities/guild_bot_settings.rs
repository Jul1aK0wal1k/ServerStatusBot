use crate::entities::BotSettings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildBotSettings {
    #[serde(rename = "_id")]
    pub id: String,
    pub bot_settings: BotSettings,
}

impl GuildBotSettings {
    pub fn new(id: String) -> Self {
        GuildBotSettings {
            id,
            bot_settings: BotSettings::default(),
        }
    }
}
