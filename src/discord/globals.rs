use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::controllers;

pub struct SteamServerInfoController;

impl TypeMapKey for SteamServerInfoController {
    type Value = Arc<controllers::ServerInfoController>;
}

pub struct GuildController;

impl TypeMapKey for GuildController {
    type Value = Arc<controllers::GuildController>;
}

pub struct BackgroundJobs;

impl TypeMapKey for BackgroundJobs {
    type Value = Arc<RwLock<rs_flow::Scheduler>>;
}
