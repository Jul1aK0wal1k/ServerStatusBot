use async_trait::async_trait;
use std::sync::Arc;

use crate::entities::{Address, ServerInfo};
use crate::errors::{ServerInfoError, ServerInfoResult};
use crate::proxies::ServerInfoProxy;
use a2s::A2SClient;

pub struct SteamServerInfoProxy {
    client: Arc<A2SClient>,
}

#[async_trait]
impl ServerInfoProxy for SteamServerInfoProxy {
    async fn server_info(&self, address: Address) -> ServerInfoResult<ServerInfo> {
        match self.client.info(address.to_string()).await {
            Ok(info) => Ok(ServerInfo::new(
                info.name,
                address,
                info.map,
                info.players as u16,
                info.max_players as u16,
            )),
            Err(_err) => Err(ServerInfoError::FailedToFetch(address)),
        }
    }
}

impl SteamServerInfoProxy {
    pub async fn new() -> ServerInfoResult<Self> {
        match A2SClient::new().await {
            Ok(client) => Ok(SteamServerInfoProxy {
                client: Arc::new(client),
            }),
            Err(reason) => Err(ServerInfoError::FailedToCreateProxy(reason.to_string())),
        }
    }
}
