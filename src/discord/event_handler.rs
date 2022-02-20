use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use rs_flow::FunctionTask;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        guild::{Guild, GuildUnavailable},
        id::GuildId,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandOptionType},
            Interaction, InteractionResponseType,
        },
        prelude::Ready,
    },
};

use crate::{
    discord::{cmd_handlers, cmds, BackgroundJobs, GuildController, SteamServerInfoController},
    entities,
};

#[derive(Default)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => cmd_handlers::ping_handler(&command),
                "add_server" => cmd_handlers::add_server_handler(&ctx, &command).await,
                "remove_server" => cmd_handlers::remove_server_by_address_handler(&ctx, &command).await,
                "remove_server_by_alias" => cmd_handlers::remove_server_by_address_handler(&ctx, &command).await,
                "list_servers" => cmd_handlers::list_servers_handler(&ctx, &command).await,
                _ => cmd_handlers::command_not_found_handler(),
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
                .create_application_command(cmds::ping_command)
                .create_application_command(cmds::add_server_command)
                .create_application_command(cmds::list_servers_command)
                .create_application_command(cmds::remove_server_by_address_command)
                .create_application_command(cmds::remove_server_by_alias_command)
        })
        .await;
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);

        let handle = tokio::spawn(async move {
            let data_read = ctx.data.read().await;
            let guild_controller = data_read.get::<GuildController>().expect("MongoDB Client not found in bot state");

            let server_controller = data_read
                .get::<SteamServerInfoController>()
                .expect("Steam Client not found in bot state");

            let guild_ids = guilds.iter().map(|x| x.to_string());

            let mut data_rw = ctx.data.write().await;
            let scheduler = data_rw.get_mut::<BackgroundJobs>().expect("Scheduler not in bot state");
            if let Err(err) = scheduler.start() {
                panic!("Couldn't start scheduler, reason {0}", err)
            }
            let results = guilds.iter().map(|id| async {
                let task_id = format!("task_{}", id.to_string());
                let task = Box::new(FunctionTask::new(|| {
                    
                }));
                scheduler
                    .add_task(task, task_id, Duration::from_secs(60), rs_flow::StartFrom::Now)
                    .await
            });
            // scheduler.add_task(task, &task_id, Duration::from_secs(60), rs_flow::StartFrom::Now)
        });
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let data_read = ctx.data.read().await;
        let client = data_read.get::<GuildController>().expect("MongoDB Client not found in bot state");

        let id = entities::GuildId::new(guild.id.to_string());
        let _result = client.create_guild(id, guild.name).await;
    }

    async fn guild_delete(&self, ctx: Context, data: GuildUnavailable, _guild: Option<Guild>) {
        if data.unavailable {
        } else {
            let data_read = ctx.data.read().await;
            let client = data_read.get::<GuildController>().expect("MongoDB Client not found in bot state");
            let delete_guild = entities::GuildId::new(data.id.to_string());
            let _result = client.delete_guild(delete_guild).await;
        }
    }
}
