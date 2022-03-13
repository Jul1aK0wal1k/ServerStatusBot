use crate::discord::commands::handlers::CmdResult;
use serenity::model::{
    channel::PartialChannel,
    interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};
use std::str::FromStr;

fn parse(
    command: &ApplicationCommandInteraction,
    opt_pos: usize,
) -> CmdResult<&ApplicationCommandInteractionDataOptionValue> {
    command
        .data
        .options
        .get(opt_pos)
        .ok_or_else(|| "No argument was supplied.".to_string())?
        .resolved
        .as_ref()
        .ok_or_else(|| "Argument was of wrong type.".to_string())
}

pub fn parse_arg<T: FromStr, ErrFn: Fn(&String) -> String>(
    command: &ApplicationCommandInteraction,
    opt_pos: usize,
    error_msg_factory: ErrFn,
) -> CmdResult<T> {
    match parse(command, opt_pos)? {
        ApplicationCommandInteractionDataOptionValue::String(s) => match T::from_str(s) {
            Ok(res) => CmdResult::Ok(res),
            Err(_) => Err(error_msg_factory(s)),
        },
        _ => Err("Supplied argument was not a string.".to_string()),
    }
}

pub fn parse_channel(
    command: &ApplicationCommandInteraction,
    opt_pos: usize,
) -> CmdResult<&PartialChannel> {
    parse(command, opt_pos).map(|opt| match opt {
        ApplicationCommandInteractionDataOptionValue::Channel(ch) => Ok(ch),
        _ => Err("Supplied argument was not a channel.".to_string()),
    })?
}
