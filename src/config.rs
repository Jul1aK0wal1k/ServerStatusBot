use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GuildStorageAdapter {
    Local,
    Mongo(MongoConfig),
}

impl Default for GuildStorageAdapter {
    fn default() -> Self {
        GuildStorageAdapter::Local
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct MongoConfig {
    pub uri: String,
    pub database: String,
    pub collection: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct DiscordConfig {
    pub token: String,
    pub application_id: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct AppConfig {
    pub discord_config: DiscordConfig,
    pub guild_storage: GuildStorageAdapter,
    pub background_scheduler_heartbeat_seconds: u64,
}
