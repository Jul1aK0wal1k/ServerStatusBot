use crate::adapters::ServerInfoAdapter;
use crate::entities::{Address, ServerInfo};
use crate::errors::ServerInfoResult;
use futures::future::join_all;

type ServerInfoAdapterRef = dyn ServerInfoAdapter + Send + Sync;

pub struct ServerInfoController {
    proxy: Box<ServerInfoAdapterRef>,
}

impl ServerInfoController {
    pub fn new(proxy: Box<ServerInfoAdapterRef>) -> Self {
        ServerInfoController { proxy }
    }

    pub async fn info_for(&self, addresses: Vec<Address>) -> ServerInfoResult<Vec<ServerInfo>> {
        if addresses.is_empty() {
            todo!()
        }
        let mut futures = Vec::with_capacity(addresses.len());
        for a in addresses {
            let fut = self.proxy.server_info(a);
            futures.push(fut);
        }
        let results: Vec<ServerInfo> = join_all(futures)
            .await
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        Ok(results)
    }
}
