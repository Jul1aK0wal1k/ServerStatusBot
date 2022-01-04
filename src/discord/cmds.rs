use serenity::{
    builder::CreateApplicationCommand, model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn ping_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Pong")
}

pub fn watch_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("watch")
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
