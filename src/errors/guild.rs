use thiserror::Error;

use crate::entities::Address;

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
    #[error("Couldn't remove address {0}")]
    FailedAddressRemoval(Address),
}
