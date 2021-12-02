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
