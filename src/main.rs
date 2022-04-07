#![warn(clippy::suspicious, clippy::perf)]
#![deny(clippy::correctness)]
use serenity::prelude::*;
use std::{env, sync::Arc, time::Duration};

mod adapters;
mod config;
mod controllers;
mod discord;
mod entities;
mod errors;
mod serde_helpers;
mod tasks;
use adapters::{MongoGuildAdapter, SteamServerInfoAdapter};
use discord::{BackgroundJobs, GuildController, Handler, SteamServerInfoController};

fn load_env_file() {
    let env_file_path = env::var("APP_ENV_FILE_PATH").map_or_else(|_| ".env".to_string(), |i| i);
    dotenv::from_path(env_file_path)
        .unwrap_or_else(|err| panic!("Couldn't load env file, reason: {}", err));
}

// #[tokio::main]
// async fn main() {
//     load_env_file();

//     let token = env::var("APP_DISCORD_TOKEN").expect("Expected a token in the environment");
//     let application_id: u64 = env::var("APP_OAUTH_APPLICATION_ID")
//         .expect("Expected an application id in the environment")
//         .parse()
//         .expect("Application id is not a valid id");

//     let mut client = Client::builder(&token)
//         .event_handler(Handler::default())
//         .application_id(application_id)
//         .await
//         .expect("Err creating client");

//     {
//         let proxy = SteamServerInfoAdapter::new().await.unwrap();

//         let mut data = client.data.write().await;
//         data.insert::<SteamServerInfoController>(Arc::new(controllers::ServerInfoController::new(
//             Box::new(proxy),
//         )));
//     }

//     {
//         let uri = env::var("APP_MONGODB_URI").expect("Expected mongodb uri in enviroment");
//         let database =
//             env::var("APP_MONGODB_DATABASE").expect("Expected mongodb database in enviroment");
//         let collection =
//             env::var("APP_MONGODB_COLLECTION").expect("Expected mongodb collection in enviroment");
//         let mongodb_client = MongoGuildAdapter::new(uri, database, collection)
//             .await
//             .unwrap();
//         let mut data = client.data.write().await;
//         data.insert::<GuildController>(Arc::new(controllers::GuildController::new(Box::new(
//             mongodb_client,
//         ))));
//     }

//     {
//         let heartbeat_interval = env::var("APP_SCHEDULER_HEARTBEAT_INTERVAL_SECS")
//             .map(|x| x.parse::<u64>().unwrap())
//             .expect("Expected heartbeat interval in enviroment");
//         let scheduler = rs_flow::Scheduler::new(Duration::from_secs(heartbeat_interval));
//         let mut data = client.data.write().await;
//         data.insert::<BackgroundJobs>(Arc::new(RwLock::new(scheduler)))
//     }

//     if let Err(why) = client.start().await {
//         println!("Client error: {:?}", why);
//     }
// }

use config::AppConfig;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let c = AppConfig::default();
    let x = toml::to_string(&c).unwrap();
    let mut content = std::fs::File::create("Config.toml").unwrap();
    content.write_all(x.as_bytes()).unwrap();
    Ok(())
}
