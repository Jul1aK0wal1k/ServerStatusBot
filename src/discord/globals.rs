use serenity::prelude::TypeMapKey;

use crate::{
    controllers,
    proxies::{MongoGuildProxy, SteamServerInfoProxy},
};

pub struct SteamServerInfoController;

impl TypeMapKey for SteamServerInfoController {
    type Value = controllers::ServerInfoController<SteamServerInfoProxy>;
}

pub struct GuildController;

impl TypeMapKey for GuildController {
    type Value = controllers::GuildController<MongoGuildProxy>;
}

pub struct BackgroundJobs;

impl TypeMapKey for BackgroundJobs {
    type Value = rs_flow::Scheduler;
}