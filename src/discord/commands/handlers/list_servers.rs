use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

use crate::{discord::GuildController, entities::GuildId};

pub async fn list_servers_handler(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> String {
    let data_read = ctx.data.read().await;
    let guild = data_read
        .get::<GuildController>()
        .expect("MongoDB Client not found in bot state");
    match command.guild_id {
        Some(id) => {
            let guild_id = GuildId::new(id.0.to_string());
            match guild.list_addresses(guild_id).await {
                Ok(adrs) => format!("Those are the addresses already added: {:?}!", adrs),
                Err(e) => format!("Something went wrong: {}", e),
            }
        }
        None => "Something went really bad and we couldn't get your servers id".to_string(),
    }
}
