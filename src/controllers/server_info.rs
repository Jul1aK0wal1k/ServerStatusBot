use crate::entities::{Address, Result, ServerInfo};
use crate::proxies::ServerInfoProxy;
use futures::future::join_all;

pub struct ServerInfoController<Proxy: ServerInfoProxy + Send + Sync> {
    proxy: Proxy,
}

impl<Proxy: ServerInfoProxy + Send + Sync> ServerInfoController<Proxy> {
    pub fn new(proxy: Proxy) -> Self {
        ServerInfoController { proxy }
    }

    pub async fn info_for(&self, addresses: Vec<Address>) -> Result<Vec<ServerInfo>> {
        if addresses.is_empty() {
            todo!()
        }
        let mut futures = Vec::with_capacity(addresses.len());
        for a in addresses {
            let fut = self.proxy.server_info(a);
            futures.push(fut);
        }
        let results = join_all(futures).await;
        Ok(Vec::new())
    }
}
