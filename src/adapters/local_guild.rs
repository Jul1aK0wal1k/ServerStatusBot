use std::sync::Arc;

use crate::{
    adapters::GuildAdapter,
    entities::{Address, Guild, GuildId},
    errors::{GuildError, GuildResult},
};
use async_trait::async_trait;
use tokio::sync::RwLock;

pub struct LocalGuildAdapter {
    storage: Arc<RwLock<Vec<Guild>>>,
}

#[async_trait]
impl GuildAdapter for LocalGuildAdapter {
    async fn status_channel_id(&self, guild_id: GuildId) -> GuildResult<String> {
        self.storage
            .read()
            .await
            .iter()
            .find(|item| item.id == guild_id.to_string())
            .ok_or(GuildError::GuildNotFound)?
            .channel_id
            .clone()
            .ok_or(GuildError::ChannelNotSet)
    }

    async fn create_guild(&self, guild_id: GuildId, name: String) -> GuildResult<()> {
        self.storage
            .write()
            .await
            .push(Guild::new(guild_id.to_string(), name));
        Ok(())
    }

    async fn delete_guild(&self, guild_id: GuildId) -> GuildResult<()> {
        self.storage
            .write()
            .await
            .retain(|item| item.id != guild_id.to_string());
        Ok(())
    }

    async fn set_channel(&self, guild_id: GuildId, channel: String) -> GuildResult<()> {
        self.storage
            .write()
            .await
            .iter_mut()
            .filter(|item| item.id == guild_id.to_string())
            .for_each(|item| item.set_channel(channel.clone()));
        Ok(())
    }

    async fn add_address(&self, guild_id: GuildId, address: Address) -> GuildResult<()> {
        self.storage
            .write()
            .await
            .iter_mut()
            .filter(|item| item.id == guild_id.to_string())
            .for_each(|item| item.add_address(address.clone()));
        Ok(())
    }

    async fn remove_address(&self, guild_id: GuildId, address: Address) -> GuildResult<()> {
        self.storage
            .write()
            .await
            .iter_mut()
            .filter(|item| item.id == guild_id.to_string())
            .for_each(|item| item.remove_address(address.clone()));
        Ok(())
    }

    async fn list_addresses(&self, guild_id: GuildId) -> GuildResult<Vec<Address>> {
        let addresses = self
            .storage
            .read()
            .await
            .iter()
            .find(|item| item.id == guild_id.to_string())
            .ok_or(GuildError::GuildNotFound)?
            .addresses
            .clone();
        Ok(addresses)
    }
}
