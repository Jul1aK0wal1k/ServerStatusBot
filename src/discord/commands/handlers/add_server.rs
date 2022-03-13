use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

use crate::{
    discord::GuildController,
    entities::{Address, GuildId},
};

use super::utils::parse_arg;

pub async fn add_server_handler(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    match parse_arg::<Address, _>(command, 0, |s| {
        format!(
            "Expected an address in the form of <host>:<port> or <host>, got {0}",
            s
        )
    }) {
        Ok(address) => {
            let data_read = ctx.data.read().await;
            let guild = data_read
                .get::<GuildController>()
                .expect("MongoDB Client not found in bot state");
            match command.guild_id {
                Some(id) => {
                    let guild_id = GuildId::new(id.0.to_string());
                    match guild.add_server(guild_id, address).await {
                        Ok(_) => "Successfully added server address!".to_string(),
                        Err(e) => format!("Couldn't add server, reason: {}", e),
                    }
                }
                None => "Something went really bad and we couldn't get your servers id".to_string(),
            }
        }
        Err(e) => e,
    }
}
