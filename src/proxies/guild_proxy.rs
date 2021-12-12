use crate::entities::{Address, GuildId};
use crate::errors::GuildResult;
use async_trait::async_trait;

#[async_trait]
pub trait GuildProxy {
    async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()>;

    async fn delete_guild(&self, guild: GuildId) -> GuildResult<()>;

    async fn add_address(&self, guild: GuildId, address: Address) -> GuildResult<()>;

    async fn remove_address(&self, guild: GuildId, address: Address) -> GuildResult<()>;

    async fn list_addresses(&self, guild: GuildId) -> GuildResult<Vec<Address>>;
}
