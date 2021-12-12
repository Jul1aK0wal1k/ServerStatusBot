use thiserror::Error;

use crate::entities::Address;

pub type ServerInfoResult<T> = std::result::Result<T, ServerInfoError>;

#[derive(Error, Debug)]
pub enum ServerInfoError {
    #[error("Couldn't fetch address: {0}")]
    FailedToFetch(Address),
    #[error("Couldn't create proxy, reason {0}")]
    FailedToCreateProxy(String),
}
