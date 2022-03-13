use std::{sync::Arc, time::Duration};

use crate::{
    discord::{commands, BackgroundJobs, GuildController, SteamServerInfoController},
    entities,
    tasks::CreateOrUpdateStatusMsg,
};
use futures::{stream::FuturesUnordered, StreamExt};
use rs_flow::{SchedulerResult, TaskId};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        guild::{Guild, GuildUnavailable},
        id::GuildId,
        interactions::{
            application_command::ApplicationCommand, Interaction, InteractionResponseType,
        },
        prelude::Ready,
    },
};

#[derive(Default)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::handlers::ping_handler(&command),
                "set_channel" => commands::handlers::set_channel_handler(&ctx, &command).await,
                "add_server" => commands::handlers::add_server_handler(&ctx, &command).await,
                "remove_server" => {
                    commands::handlers::remove_server_by_address_handler(&ctx, &command).await
                }
                "remove_server_by_alias" => {
                    commands::handlers::remove_server_by_address_handler(&ctx, &command).await
                }
                "list_servers" => commands::handlers::list_servers_handler(&ctx, &command).await,
                _ => commands::handlers::errors::command_not_found_handler(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _ = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(commands::ping_command)
                .create_application_command(commands::set_channel_command)
                .create_application_command(commands::add_server_command)
                .create_application_command(commands::list_servers_command)
                .create_application_command(commands::remove_server_by_address_command)
                .create_application_command(commands::remove_server_by_alias_command)
        })
        .await;
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);

        let guild_controller = ctx
            .data
            .read()
            .await
            .get::<GuildController>()
            .expect("MongoDB Client not found in bot state")
            .clone();

        let server_controller = ctx
            .data
            .read()
            .await
            .get::<SteamServerInfoController>()
            .expect("Steam Client not found in bot state")
            .clone();

        let scheduler = ctx
            .data
            .read()
            .await
            .get::<BackgroundJobs>()
            .expect("Scheduler not in bot state")
            .clone();

        let results = guilds
            .iter()
            .map(|x| x.to_string())
            .map(|id| {
                let task_id = format!("task_{}", id);
                Box::new(CreateOrUpdateStatusMsg::new(
                    id,
                    ctx.clone(),
                    server_controller.clone(),
                    guild_controller.clone(),
                ))
            })
            .map(|task| async {
                scheduler
                    .read()
                    .await
                    .add_task(
                        task,
                        "".to_string(),
                        Duration::from_secs(60),
                        rs_flow::StartFrom::Now,
                    )
                    .await
            })
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<SchedulerResult<TaskId>>>()
            .await;
        // scheduler.add_task(task, &task_id, Duration::from_secs(60), rs_flow::StartFrom::Now)
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let data_read = ctx.data.read().await;
        let client = data_read
            .get::<GuildController>()
            .expect("MongoDB Client not found in bot state");

        let id = entities::GuildId::new(guild.id.0.to_string());
        let _result = client.create_guild(id, guild.name).await;
    }

    async fn guild_delete(&self, ctx: Context, data: GuildUnavailable, _guild: Option<Guild>) {
        if data.unavailable {
        } else {
            let data_read = ctx.data.read().await;
            let client = data_read
                .get::<GuildController>()
                .expect("MongoDB Client not found in bot state");
            let delete_guild = entities::GuildId::new(data.id.0.to_string());
            let _result = client.delete_guild(delete_guild).await;
        }
    }
}
