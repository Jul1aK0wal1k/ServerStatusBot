use serenity::{
    builder::CreateApplicationCommand, model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn ping_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Pong")
}

pub fn add_server_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_server")
        .description("Add a server address to watch")
        .create_option(|option| {
            option
                .name("address")
                .description("The address to watch in the format of <host or IP>:<port>")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("channel")
                .description("Where should I put the message about the server status?")
                .kind(ApplicationCommandOptionType::Channel)
                .required(true)
        })
}

pub fn list_servers_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list_servers").description("Lists all added servers that are currently being tracked")
}

pub fn remove_server_by_address_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("remove_server_by_address").description("Removes a server from tracking").create_option(|option| {
        option
            .name("address")
            .description("Address in the format of <host or IP>:<port>")
            .kind(ApplicationCommandOptionType::String)
            .required(true)
    })
}

pub fn remove_server_by_alias_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("remove_server_by_alias").description("Removes a server from tracking using it's alias").create_option(|option| {
        option
            .name("alias")
            .description("Alias of a tracked server")
            .kind(ApplicationCommandOptionType::String)
            .required(true)
    })
}