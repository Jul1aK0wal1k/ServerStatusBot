use std::ops::Deref;

use crate::entities::{Address, ServerInfo};
use crate::errors::ServerInfoResult;
use crate::proxies::ServerInfoProxy;
use futures::future::join_all;

pub struct ServerInfoController<Proxy: ServerInfoProxy + Send + Sync> {
    proxy: Proxy,
}

impl<Proxy: ServerInfoProxy + Send + Sync> ServerInfoController<Proxy> {
    pub fn new(proxy: Proxy) -> Self {
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
            .map(|x| x.to_owned())
            .collect();
        Ok(results)
    }
}
