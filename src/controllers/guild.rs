use crate::{
    entities::{Address, GuildId},
    errors::GuildResult,
    proxies::GuildProxy,
};

type GuildProxyType = dyn GuildProxy + Send + Sync;

pub struct GuildController {
    proxy: Box<GuildProxyType>,
}

impl GuildController {
    pub fn new(proxy: Box<GuildProxyType>) -> Self {
        GuildController { proxy }
    }

    pub async fn status_channel_id(&self, guild: GuildId) -> GuildResult<String> {
        self.proxy.status_channel_id(guild).await
    }

    pub async fn list_addresses(&self, guild: GuildId) -> GuildResult<Vec<Address>> {
        self.proxy.list_addresses(guild).await
    }

    pub async fn set_channel(&self, guild: GuildId, channel: String) -> GuildResult<()> {
        self.proxy.set_channel(guild, channel).await
    }

    pub async fn add_server(&self, guild: GuildId, address: Address) -> GuildResult<()> {
        self.proxy.add_address(guild, address).await
    }

    pub async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()> {
        self.proxy.create_guild(guild, name).await
    }

    pub async fn delete_guild(&self, guild: GuildId) -> GuildResult<()> {
        self.proxy.delete_guild(guild).await
    }

    pub async fn remove_address(&self, guild_id: GuildId, address: Address) -> GuildResult<()> {
        self.proxy.remove_address(guild_id, address).await
    }
}
