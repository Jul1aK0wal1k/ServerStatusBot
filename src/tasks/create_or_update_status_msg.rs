use async_trait::async_trait;
use rs_flow::SchedulerTask;

use crate::proxies::{GuildProxy, ServerInfoProxy};

pub struct CreateOrUpdateStatusMsg<'a> {
    server_status_controller: &'a dyn ServerInfoProxy,
    guild_controller: &'a dyn GuildProxy,
}

impl<'a> CreateOrUpdateStatusMsg<'a> {
    pub fn new(
        server_status_controller: &'a dyn ServerInfoProxy,
        guild_controller: &'a dyn GuildProxy,
    ) -> Self {
        CreateOrUpdateStatusMsg {
            server_status_controller,
            guild_controller,
        }
    }
}


#[async_trait]
impl<'a> SchedulerTask for CreateOrUpdateStatusMsg<'a> {
    async fn execute(&self) -> std::io::Result<()> {
        todo!()
    }
}