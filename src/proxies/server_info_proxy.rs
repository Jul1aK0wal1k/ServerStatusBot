use crate::{
    entities::{Address, ServerInfo},
    errors::ServerInfoResult,
};
use async_trait::async_trait;

#[async_trait]
pub trait ServerInfoProxy {
    async fn server_info(&self, address: Address) -> ServerInfoResult<ServerInfo>;
}
