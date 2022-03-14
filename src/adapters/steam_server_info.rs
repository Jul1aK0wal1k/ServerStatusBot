use async_trait::async_trait;
use std::sync::Arc;

use crate::entities::{Address, ServerInfo};
use crate::errors::{ServerInfoError, ServerInfoResult};
use a2s::A2SClient;

use super::ServerInfoAdapter;

pub struct SteamServerInfoAdapter {
    client: Arc<A2SClient>,
}

impl SteamServerInfoAdapter {
    pub async fn new() -> ServerInfoResult<Self> {
        match A2SClient::new().await {
            Ok(client) => Ok(SteamServerInfoAdapter {
                client: Arc::new(client),
            }),
            Err(reason) => Err(ServerInfoError::FailedToCreateProxy(reason.to_string())),
        }
    }
}

#[async_trait]
impl ServerInfoAdapter for SteamServerInfoAdapter {
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
