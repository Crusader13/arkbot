use serenity::{
    async_trait,
    client::Client,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

enum Map {
    Island,
    Center,
    Ragnarok,
    CrystalIsles,
    Valguero,
    Aberration,
    Extinction,
    SE,
    Gen1,
    Gen2,
    LostIsland,
}

struct Server {
    map_name: Map,
}
impl Server {
    fn from(map_name: Map) -> Server {
        Server { map_name }
    }
    fn is_active(&self) -> bool {
        let command = std::process::Command::new("systemctl")
            .arg("--user")
            .arg("is-active")
            .arg(format!("ark{}", self.map_name.borrow().into()))
            .output()
            .unwrap()
            .stdout;
        let output = String::from_utf8_lossy(&command).to_string();

        if output.contains("active") {
            true
        } else {
            false
        }
    }
    fn stop(&self) -> Result<(), String> {
        if !self.is_active() {
            return Err("Der Server läuft nicht!".to_string());
        }
        match std::process::Command::new("systemctl")
            .arg("--user")
            .arg("stop")
            .arg(format!("ark{}", self.map_name.borrow().into()))
            .output()
        {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(
                "Der Server konnte nicht gestoppt werden: {}",
                err.into()
            )),
        }
    }
}

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

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Der Client konnte nicht erstellt werden!");

    if let Err(err) = client.start().await {
        eprintln!("Client fehler: {:?}", err);
    }
}
