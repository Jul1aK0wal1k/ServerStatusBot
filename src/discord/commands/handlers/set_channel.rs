use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

use crate::discord::GuildController;

use super::utils::parse_channel;

pub async fn set_channel_handler(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    match command.guild_id {
        Some(id) => match parse_channel(command, 0) {
            Ok(ch) => {
                let data_read = ctx.data.read().await;
                let guild = data_read
                    .get::<GuildController>()
                    .expect("MongoDB Client not found in bot state");
                match guild
                    .set_channel(id.0.to_string().into(), ch.id.to_string())
                    .await
                {
                    Err(err) => err.to_string(),
                    Ok(_) => "Successfully set channel!".to_string(),
                }
            }
            Err(err) => err,
        },
        None => "Something went really bad and we couldn't get your servers id".to_string(),
    }
}
