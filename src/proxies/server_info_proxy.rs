use crate::entities::{Address, ServerInfo};
use async_trait::async_trait;
use thiserror::Error;

pub type ServerInfoResult<T> = std::result::Result<T, ServerInfoError>;

#[derive(Error, Debug)]
pub enum ServerInfoError {
    #[error("Couldn't fetch address: {0}")]
    FailedToFetch(Address),
}

#[async_trait]
pub trait ServerInfoProxy {
    async fn server_info(&self, address: Address) -> ServerInfoResult<ServerInfo>;
}
