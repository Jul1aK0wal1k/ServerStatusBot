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


pub enum CmdResult<T> {
    Ok(T),
    Error(String)
}

pub fn parse_arg<T: FromStr, ErrFn: Fn(&String) -> String>(command: &ApplicationCommandInteraction, index: usize, error_msg_factory: ErrFn) -> CmdResult<T> {
    match command.data.options.get(index) {
        Some(opt) => {
            match opt.resolved.as_ref() {
                Some(opt) => {
                    match opt {
                    ApplicationCommandInteractionDataOptionValue::String(s) => {
                        match T::from_str(s) {
                            Ok(res) => CmdResult::Ok(res),
                            Err(_) => CmdResult::Error(error_msg_factory(s)),
                        }
                    },
                    _ => CmdResult::Error(format!("Supplied argument was not a string."))
                }},
                None => CmdResult::Error(format!("No argument was supplied.")),
            }
        },
        None => CmdResult::Error(format!("No argument was supplied.")),
    }
}

pub fn command_not_found_handler() -> String {
    "not implemented :(".to_string()
}

pub fn ping_handler(_command: &ApplicationCommandInteraction) -> String {
    "Pong".to_string()
}


pub async fn add_server_handler(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    match parse_arg::<Address, _>(command,
        0, 
        |s| format!("Expected an address in the form of <host>:<port> or <host>, got {0}", s)) {
            CmdResult::Ok(address) => {
                let data_read = ctx.data.read().await;
                let guild = data_read
                    .get::<GuildController>()
                    .expect("MongoDB Client not found in bot state");
                match command.guild_id {
                    Some(id) => {
                        let guild_id = GuildId::new(id.0.to_string());
                        match guild.add_server(guild_id, address).await {
                            Ok(_) => format!("Successfully added server address!"),
                            Err(e) => format!("Couldn't add server, reason: {}", e.to_string()),
                        }
                    },
                    None => format!("Something went really bad and we couldn't get your servers id"),
                }
            },
            CmdResult::Error(e) => e,
        }
}

pub async fn remove_server_by_address_handler(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    match parse_arg::<Address, _>(command,
        0, 
        |s| format!("Expected an address in the form of <host>:<port> or <host>, got {0}", s)) {
            CmdResult::Ok(address) => {
                let data_read = ctx.data.read().await;
                let guild = data_read
                    .get::<GuildController>()
                    .expect("MongoDB Client not found in bot state");
                match command.guild_id {
                    Some(id) => {
                        let guild_id = GuildId::new(id.0.to_string());
                        match guild.remove_address(guild_id, address.clone()).await {
                            Ok(_) => format!("Successfully removed server address {0}!", address.clone()),
                            Err(e) => format!("Couldn't add server, reason: {}", e.to_string()),
                        }
                    },
                    None => format!("Something went really bad and we couldn't get your servers id"),
                }
            },
            CmdResult::Error(e) => e,
        }
}

pub async fn list_servers_handler(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let data_read = ctx.data.read().await;
    let guild = data_read
        .get::<GuildController>()
        .expect("MongoDB Client not found in bot state");
    match command.guild_id {
        Some(id) => {
            let guild_id = GuildId::new(id.0.to_string());
            match guild.list_addresses(guild_id).await {
                Ok(adrs) => format!("Those are the addresses already added: {:?}!", adrs),
                Err(e) => format!("Couldn't add server, reason: {}", e.to_string()),
            }
        },
        None => format!("Something went really bad and we couldn't get your servers id"),
    }
}
