use crate::entities::{Address, GuildId};
use crate::errors::GuildResult;
use async_trait::async_trait;

#[async_trait]
pub trait GuildProxy {
    async fn status_channel_id(&self, guild: GuildId) -> GuildResult<String>;

    async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()>;

    async fn delete_guild(&self, guild: GuildId) -> GuildResult<()>;

    async fn set_channel(&self, guild: GuildId, channel: String) -> GuildResult<()>;

    async fn add_address(&self, guild: GuildId, address: Address) -> GuildResult<()>;

    async fn remove_address(&self, guild: GuildId, address: Address) -> GuildResult<()>;

    async fn list_addresses(&self, guild: GuildId) -> GuildResult<Vec<Address>>;
}
