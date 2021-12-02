use crate::{
    entities::{Address, GuildId, Result},
    proxies::GuildProxy,
};

pub struct GuildController<Proxy: GuildProxy + Send + Sync> {
    proxy: Proxy,
}

impl<Proxy: GuildProxy + Send + Sync> GuildController<Proxy> {
    pub fn new(proxy: Proxy) -> Self {
        GuildController { proxy }
    }

    pub async fn server_addresses(&self, id: String) -> Result<Vec<Vec<Address>>> {
        todo!()
    }

    pub async fn add_server(&self, address: Address) -> Result<()> {
        todo!()
    }

    pub async fn create_guild(&self, id: GuildId, name: String) -> Result<()> {
        match self.proxy.create_guild(id, name).await {
            Ok(_) => Ok(()),
            Err(err) => Ok(()),
        }
    }

    pub async fn delete_guild(&self, guild: GuildId) -> Result<()> {
        todo!()
    }
}
