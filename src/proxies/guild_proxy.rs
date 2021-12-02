use crate::entities::{Address, GuildId};
use async_trait::async_trait;
use thiserror::Error;

pub type GuildResult<T> = std::result::Result<T, GuildError>;

#[derive(Error, Debug)]
pub enum GuildError {
    #[error("Couldn't create a database record for guild")]
    FailedGuildCreation,
    #[error("Couldn't delete the database record for guild")]
    FailedGuildDeletion,
    #[error("Couldn't find guild")]
    GuildNotFound,
    #[error("Couldn't retrieve server addresses")]
    FailedRetrievingAddresses,
    #[error("Couldn't add address {0}")]
    FailedToAddAddress(Address),
    #[error("Couldn't create proxy, reason {0}")]
    FailedToCreateProxy(String),
}

#[async_trait]
pub trait GuildProxy {
    async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()>;

    async fn delete_guild(&self, guild: GuildId) -> GuildResult<()>;

    async fn add_address(&self, guild: GuildId, address: Address) -> GuildResult<()>;

    async fn server_addresses(&self, guild: GuildId) -> GuildResult<Vec<Address>>;
}
