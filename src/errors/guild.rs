use thiserror::Error;

pub type GuildResult<T> = std::result::Result<T, GuildError>;

#[derive(Error, Debug)]
pub enum GuildError {
    #[error("Couldn't find guild")]
    GuildNotFound,
    #[error("Couldn't process request, reason {0}")]
    RequestFailedWithReason(String),
    #[error("Channel was not set, please set channel and try again!")]
    ChannelNotSet,
}
