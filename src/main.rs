#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

use serenity::prelude::*;
use std::{env, time::Duration};

mod controllers;
mod discord;
mod entities;
mod errors;
mod proxies;
mod serde_helpers;
use discord::{BackgroundJobs, GuildController, Handler, SteamServerInfoController};
use proxies::{MongoGuildProxy, SteamServerInfoProxy};

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
        let database = env::var("APP_MONGODB_DATABASE").expect("Expected mongodb database in enviroment");
        let collection = env::var("APP_MONGODB_COLLECTION").expect("Expected mongodb collection in enviroment");
        let mongodb_client = MongoGuildProxy::new(uri, database, collection).await.unwrap();
        let mut data = client.data.write().await;
        data.insert::<GuildController>(controllers::GuildController::new(mongodb_client));
    }

    {
        let heartbeat_interval = env::var("APP_SCHEDULER_HEARTBEAT_INTERVAL_SECS")
            .map(|x| x.parse::<u64>().unwrap())
            .expect("Expected heartbeat interval in enviroment");
        let scheduler = rs_flow::Scheduler::new(Duration::from_secs(heartbeat_interval));
        let mut data = client.data.write().await;
        data.insert::<BackgroundJobs>(scheduler)
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
