use crate::server::{Map, Server};
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use std::env;
use std::fmt::Display;
use std::sync::Mutex;
use strum::IntoEnumIterator;

mod server;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
static SERVERS: Mutex<Vec<Server>> = Mutex::new(Vec::new());

#[poise::command(slash_command)]
async fn start(
    ctx: Context<'_>,
    #[description = "Welcher Server soll gestartet werden?"] server_input: String,
) -> Result<(), Error> {
    let available_servers = SERVERS.lock().unwrap();
    let mut server_to_start: Option<Server> = None;
    
    for server in available_servers.iter() {
        let server_name: String = server.map_name.to_string();
        
        if server_name.contains(server_input.as_str()) {
            server_to_start = Some(*server.clone());
        }
    }
    
    todo!()
}

#[tokio::main]
async fn main() {
    let token =
        env::var("DISCORD_TOKEN").expect("Die DISCORD_TOKEN Umgebungsvariable gibt es nicht");

    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![start()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    
    let mut servers_temp: Vec<Server> = Vec::with_capacity(8);
    for map in Map::iter() {
        if let Some(server) = Server::from(map) {
            servers_temp.push(server);
        }
    }
    let mut servers = SERVERS.lock().unwrap();
    *servers = servers_temp;
    
    client.unwrap().start().await.unwrap();
}
