use async_trait::async_trait;
use mongodb::options::FindOneOptions;
use mongodb::{bson, options, Client};

use crate::entities::{Address, Guild, GuildId};
use crate::proxies::{GuildError, GuildProxy, GuildResult};

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
            Ok(client) => Ok(MongoGuildProxy { client, database_name, collection_name }),
            Err(error) => {
                let error_msg = error.to_string();
                Err(GuildError::FailedToCreateProxy(error_msg))
            }
        }
    }
}

#[async_trait]
impl GuildProxy for MongoGuildProxy {
    async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()> {
        let collection =
            self.client.database(&self.database_name).collection::<Guild>(&self.collection_name);

        match collection
            .insert_one(
                Guild::new(guild.id.to_string(), name),
                options::InsertOneOptions::default(),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(GuildError::FailedGuildCreation),
        }
    }

    async fn delete_guild(&self, guild: GuildId) -> GuildResult<()> {
        let collection = self
            .client
            .database(&self.database_name)
            .collection::<bson::Document>(&self.collection_name);

        match bson::to_document(&guild) {
            Ok(query) => {
                match collection.delete_one(query, options::DeleteOptions::default()).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(GuildError::FailedGuildDeletion),
                }
            }
            Err(_) => panic!("BSON can't be malformed cause it's struct with a `String`"),
        }
    }
    async fn add_address(&self, guild: GuildId, address: Address) -> GuildResult<()> {
        let collection =
            self.client.database(&self.database_name).collection::<Guild>(&self.collection_name);
        match collection
            .update_one(
                bson::doc! {"_id": guild.id},
                bson::doc! {"bot_settings.hosts": {"$addToSet": address.to_string()}},
                None,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(GuildError::FailedToAddAddress(address)),
        }
    }
    async fn server_addresses(&self, guild_id: GuildId) -> GuildResult<Vec<Address>> {
        let collection =
            self.client.database(&self.database_name).collection::<Guild>(&self.collection_name);
        let filter = bson::doc! {"_id": guild_id.id};
        let projection = bson::doc! {"bot_settings.hosts": true};
        let find_options = FindOneOptions::builder().projection(projection).build();
        if let Ok(cursor) = collection.find_one(filter, find_options).await {
            if let Some(guild) = cursor {
                Ok(guild.bot_settings.hosts)
            } else {
                Err(GuildError::GuildNotFound)
            }
        } else {
            Err(GuildError::FailedRetrievingAddresses)
        }
    }
}
