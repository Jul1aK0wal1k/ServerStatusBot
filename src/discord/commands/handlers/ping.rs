use serenity::model::interactions::application_command::ApplicationCommandInteraction;

pub fn ping_handler(_command: &ApplicationCommandInteraction) -> String {
    "Pong".to_string()
}
