use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn set_channel_command(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("set_channel")
        .description(
            "Set the channel where the bot will post self updating messages with the server status",
        )
        .create_option(|option| {
            option
                .name("channel")
                .description("Where should I put the message about the server status?")
                .kind(ApplicationCommandOptionType::Channel)
                .required(true)
        })
}
