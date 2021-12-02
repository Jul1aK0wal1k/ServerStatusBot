use serenity::prelude::*;
use std::env;

mod entities;
mod proxies;
use proxies::{MongoGuildProxy, SteamServerInfoProxy};
mod controllers;
mod discord;
use discord::{GuildController, Handler, SteamServerInfoController};

#[tokio::main]
async fn main() {
    let token = env::var("APP_DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id: u64 = env::var("APP_OAUTH_APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("Application id is not a valid id");

    let mut client = Client::builder(&token)
        .event_handler(Handler::default())
        .application_id(application_id)
        .await
        .expect("Err creating client");

    {
        let proxy = SteamServerInfoProxy::new().await.unwrap();

        let mut data = client.data.write().await;
        data.insert::<SteamServerInfoController>(controllers::ServerInfoController::new(proxy));
    }

    {
        let uri = env::var("APP_MONGODB_URI").expect("Expected mongodb uri in enviroment");
        let database =
            env::var("APP_MONGODB_DATABASE").expect("Expected mongodb database in enviroment");
        let collection =
            env::var("APP_MONGODB_COLLECTION").expect("Expected mongodb collection in enviroment");
        let mongodb_client = MongoGuildProxy::new(uri, database, collection)
            .await
            .unwrap();
        let mut data = client.data.write().await;
        data.insert::<GuildController>(controllers::GuildController::new(mongodb_client));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
