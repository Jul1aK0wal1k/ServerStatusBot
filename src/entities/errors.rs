use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Couldn't connect to server: {0}")]
    A2SConnection(String),
    #[error("Couldn't create UDP socket for A2S client")]
    A2SClientCreation,

    #[error("Couldn't connect to MongoDB, reason: {0}")]
    MongoDBClientConnection(String),
    #[error("Couldn't execute command, reason: {0}")]
    MongoDBOperation(String),

    #[error("Couldn't serialize object to Bson, reason: {0}")]
    BsonSerialization(String),
}
