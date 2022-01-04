use std::str::FromStr;

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use crate::{
    discord::GuildController,
    entities::{Address, GuildId},
};

pub fn cmd_not_found() -> String {
    "not implemented :(".to_string()
}

pub fn ping_command_handler(_command: &ApplicationCommandInteraction) -> String {
    "Pong".to_string()
}

pub async fn watch_command_handler(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let address = command
        .data
        .options
        .get(0)
        .expect("Expected an address in the form of <host>:<port> or <host>")
        .resolved
        .as_ref()
        .expect("Expected String");

    if let ApplicationCommandInteractionDataOptionValue::String(s) = address {
        if let Ok(address) = Address::from_str(s) {
            let data_read = ctx.data.read().await;
            let guild = data_read
                .get::<GuildController>()
                .expect("MongoDB Client not found in bot state");

            if let Some(id) = command.guild_id {
                let guild_id = GuildId::new(id.0.to_string());
                let result = guild.add_server(guild_id, address).await;
            }
        }
    }
    format!("")
}
