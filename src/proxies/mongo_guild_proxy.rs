use async_trait::async_trait;
use mongodb::bson::to_bson;
use mongodb::{bson, options, Client};

use crate::entities::{Address, Guild, GuildId};
use crate::errors::{GuildError, GuildResult};
use crate::proxies::GuildProxy;

pub struct MongoGuildProxy {
    client: Client,
    database_name: String,
    collection_name: String,
}

impl MongoGuildProxy {
    pub async fn new(
        uri: String,
        database_name: String,
        collection_name: String,
    ) -> GuildResult<Self> {
        match Client::with_uri_str(uri).await {
            Ok(client) => Ok(MongoGuildProxy {
                client,
                database_name,
                collection_name,
            }),
            Err(error) => {
                let error_msg = error.to_string();
                Err(GuildError::RequestFailedWithReason(error_msg))
            }
        }
    }
}

#[async_trait]
impl GuildProxy for MongoGuildProxy {
    async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<Guild>(&self.collection_name);

        match collection
            .insert_one(
                Guild::new(guild.id.to_string(), name),
                options::InsertOneOptions::default(),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(GuildError::RequestFailedWithReason(err.to_string())),
        }
    }

    async fn delete_guild(&self, guild: GuildId) -> GuildResult<()> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<bson::Document>(&self.collection_name);

        match bson::to_document(&guild) {
            Ok(query) => match collection
                .delete_one(query, options::DeleteOptions::default())
                .await
            {
                Ok(_) => Ok(()),
                Err(err) => Err(GuildError::RequestFailedWithReason(err.to_string())),
            },
            Err(_) => panic!("BSON can't be malformed cause it's struct with a `String`"),
        }
    }

    async fn set_channel(&self, guild: GuildId, channel: String) -> GuildResult<()> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<Guild>(&self.collection_name);

        match collection
            .update_one(
                bson::doc! {"_id": guild.id},
                bson::doc! {"$set": {"channel": channel}},
                None,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(GuildError::RequestFailedWithReason(err.to_string())),
        }
    }

    async fn add_address(&self, guild: GuildId, address: Address) -> GuildResult<()> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<Guild>(&self.collection_name);
        match collection
            .update_one(
                bson::doc! {"_id": guild.id},
                bson::doc! {"$addToSet": {"addresses": address.to_string()}},
                None,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(GuildError::RequestFailedWithReason(err.to_string())),
        }
    }
    async fn list_addresses(&self, guild_id: GuildId) -> GuildResult<Vec<Address>> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<Guild>(&self.collection_name);
        let filter = bson::doc! {"_id": guild_id.id};
        match collection.find_one(filter, None).await {
            Ok(maybe_guild) => {
                if let Some(guild) = maybe_guild {
                    Ok(guild.addresses)
                } else {
                    Err(GuildError::GuildNotFound)
                }
            }
            Err(err) => Err(GuildError::RequestFailedWithReason(err.to_string())),
        }
    }
    async fn remove_address(&self, guild: GuildId, address: Address) -> GuildResult<()> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<Guild>(&self.collection_name);
        let bson_address = to_bson(&address).unwrap();
        let update = bson::doc! {"$pull": {"addresses": bson_address}};
        match collection
            .update_one(bson::doc! {"_id": guild.id}, update, None)
            .await
        {
            Ok(_result) => Ok(()),
            Err(error) => Err(GuildError::RequestFailedWithReason(error.to_string())),
        }
    }

    async fn status_channel_id(&self, guild: GuildId) -> GuildResult<String> {
        let filter = bson::doc! {"_id": guild.id};
        self.client
            .database(&self.database_name)
            .collection::<Guild>(&self.collection_name)
            .find_one(filter, None)
            .await
            .map_err(|err| GuildError::RequestFailedWithReason(err.to_string()))?
            .ok_or(GuildError::GuildNotFound)?
            .channel_id
            .ok_or(GuildError::ChannelNotSet)
    }
}
