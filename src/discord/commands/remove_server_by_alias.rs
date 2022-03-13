use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn remove_server_by_alias_command(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("remove_server_by_alias")
        .description("Removes a server from tracking using it's alias")
        .create_option(|option| {
            option
                .name("alias")
                .description("Alias of a tracked server")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
}
