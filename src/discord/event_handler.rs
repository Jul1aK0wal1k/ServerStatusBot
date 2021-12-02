use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

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
    discord::globals::{GuildController, SteamServerInfoController},
    entities,
};

#[derive(Default)]
pub struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    // Bind commands to command handlers
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Pong".to_string(),
                "watch" => {
                    let address = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected an address in the form of <host or IP>:<port>")
                        .resolved
                        .as_ref()
                        .expect("Expected String");

                    let channel = command
                        .data
                        .options
                        .get(1)
                        .expect("Expected a channel that I can see")
                        .resolved
                        .as_ref()
                        .expect("Expected channel object");

                    let data_read = ctx.data.read().await;
                    let guild = data_read
                        .get::<GuildController>()
                        .expect("MongoDB Client not found in bot state");

                    // guild.add_server();

                    format!("{:?}, {:?}", address, channel)
                }
                _ => "not implemented :(".to_string(),
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

    // Create commands with metainformation about it
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("watch")
                        .description("Add a server address to watch")
                        .create_option(|option| {
                            option
                                .name("address")
                                .description(
                                    "The address to watch in the format of <host or IP>:<port>",
                                )
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("channel")
                                .description(
                                    "Where should I put the message about the server status?",
                                )
                                .kind(ApplicationCommandOptionType::Channel)
                                .required(true)
                        })
                })
        })
        .await;

        println!(
            "I now have the following global slash commands: {:#?}",
            commands
        );
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);

        if self.is_loop_running.load(Ordering::Relaxed) {
            tokio::spawn(async move {
                loop {
                    let data_read = ctx.data.read().await;
                    let guild_controller = data_read
                        .get::<GuildController>()
                        .expect("MongoDB Client not found in bot state");

                    let server_controller = data_read
                        .get::<SteamServerInfoController>()
                        .expect("Steam Client not found in bot state");

                    let guild_ids = guilds.iter().map(|x| x.to_string()).collect();
                    match guild_controller.server_addresses(guild_ids).await {
                        Ok(settings) => {
                            // let tasks = FuturesUnordered::new();
                            for hosts in settings {
                                server_controller.info_for(hosts).await;
                            }
                        }
                        Err(_error) => todo!(),
                    }

                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });
        }

        self.is_loop_running.swap(true, Ordering::Relaxed);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let data_read = ctx.data.read().await;
        let client = data_read
            .get::<GuildController>()
            .expect("MongoDB Client not found in bot state");

        let id = entities::GuildId::new(guild.id.to_string());
        let _result = client.create_guild(id, guild.name).await;
    }

    async fn guild_delete(&self, ctx: Context, data: GuildUnavailable, _guild: Option<Guild>) {
        if data.unavailable {
        } else {
            let data_read = ctx.data.read().await;
            let client = data_read
                .get::<GuildController>()
                .expect("MongoDB Client not found in bot state");
            let delete_guild = entities::GuildId::new(data.id.to_string());
            let _result = client.delete_guild(delete_guild).await;
        }
    }
}
