use crate::entities::{Address, GuildId};
use crate::errors::GuildResult;
use async_trait::async_trait;

#[async_trait]
pub trait GuildAdapter {
    async fn status_channel_id(&self, guild_id: GuildId) -> GuildResult<String>;

    async fn create_guild(&self, guild_id: GuildId, name: String) -> GuildResult<()>;

    async fn delete_guild(&self, guild_id: GuildId) -> GuildResult<()>;

    async fn set_channel(&self, guild_id: GuildId, channel: String) -> GuildResult<()>;

    async fn add_address(&self, guild_id: GuildId, address: Address) -> GuildResult<()>;

    async fn remove_address(&self, guild_id: GuildId, address: Address) -> GuildResult<()>;

    async fn list_addresses(&self, guild_id: GuildId) -> GuildResult<Vec<Address>>;
}
