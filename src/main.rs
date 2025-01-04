use poise::serenity_prelude as serenity;
use serenity::{
    prelude::*,
    async_trait,
    client::Client,
    model::{channel::Message, gateway::Ready},
};
use std::env;

mod server;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if message.content == HELP_COMMAND {
            if let Err(error) = message.channel_id.say(&context.http, HELP_MESSAGE).await {
                eprintln!("Error sending message: {:?}", error);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} ist gestartet!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token =
        env::var("DISCORD_TOKEN").expect("Die DISCORD_TOKEN Umgebungsvariable gibt es nicht");

    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Der Client konnte nicht erstellt werden!");

    if let Err(err) = client.start().await {
        eprintln!("Client fehler: {:?}", err);
    }
}
