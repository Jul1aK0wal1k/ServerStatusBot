use std::sync::Arc;

use crate::controllers::{GuildController, ServerInfoController};
use async_trait::async_trait;
use rs_flow::SchedulerTask;
use serenity::client::Context;

pub struct CreateOrUpdateStatusMsg {
    guild_id: String,
    discord_handler: Arc<Context>,
    server_status_controller: Arc<ServerInfoController>,
    guild_controller: Arc<GuildController>,
}

impl CreateOrUpdateStatusMsg {
    pub fn new(
        guild_id: String,
        discord_handler: Arc<Context>,
        server_status_controller: Arc<ServerInfoController>,
        guild_controller: Arc<GuildController>,
    ) -> Self {
        CreateOrUpdateStatusMsg {
            guild_id,
            discord_handler,
            server_status_controller,
            guild_controller,
        }
    }
}

#[async_trait]
impl SchedulerTask for CreateOrUpdateStatusMsg {
    async fn execute(&self) -> rs_flow::TaskResult {
        let addresses = self
            .guild_controller
            .list_addresses(self.guild_id.clone().into())
            .await
            .map_err(|err| rs_flow::TaskError::FailedTaskExecution(err.to_string()))?;
        let channel_id = self
            .guild_controller
            .status_channel_id(self.guild_id.clone().into())
            .await
            .map(|x| x.parse::<u64>())
            .map_err(|err| rs_flow::TaskError::FailedTaskExecution(err.to_string()))?
            .map_err(|err| rs_flow::TaskError::FailedTaskExecution(err.to_string()))?;
        let msg = self
            .discord_handler
            .http
            .get_channel(channel_id)
            .await
            .map_err(|err| rs_flow::TaskError::FailedTaskExecution(err.to_string()))?
            .id()
            .send_message(self.discord_handler.http.clone(), |msg| {
                msg.embed(|em| {
                    em.title("Antistasi Community Server 1")
                        .description("**Teamspeak Server Address:** `38.133.154.60`")
                        .color(0x28a722 as i32)
                        .thumbnail("https://i.postimg.cc/d0Y0krSc/mainserver-1-logo.png")
                        .field("ON", ":ballot_box_with_check:", true)
                        .author(|auth| {
                            auth.name("Antistasi Community")
                                .icon_url("https://i.postimg.cc/d0Y0krSc/mainserver-1-logo.png")
                                .url("https://antistasi.de/")
                        })
                })
            })
            .await;
        todo!()
    }
}
