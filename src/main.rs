use serenity::{
    async_trait,
    client::Client,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fmt::{format, Display, Formatter};
use std::process::Output;
use std::{env, io};

enum Map {
    TheIsland,
    TheCenter,
    Ragnarok,
    CrystalIsles,
    Valguero,
    Aberration,
    Extinction,
    ScorchedEarth,
    Genesis1,
    Genesis2,
    LostIsland,
    Fjordur,
}

enum ServerAction {
    Start,
    Restart,
    IsActive,
    Stop,
}

impl Display for ServerAction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ServerAction::Start => write!(f, "start"),
            ServerAction::Restart => write!(f, "restart"),
            ServerAction::IsActive => write!(f, "is-active"),
            ServerAction::Stop => write!(f, "stop"),
        }
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Map::TheIsland => write!(f, "Island"),
            Map::TheCenter => write!(f, "Center"),
            Map::Ragnarok => write!(f, "Ragnarok"),
            Map::CrystalIsles => write!(f, "CrystalIsles"),
            Map::Valguero => write!(f, "Valguero"),
            Map::Aberration => write!(f, "Aberration"),
            Map::Extinction => write!(f, "Extinction"),
            Map::ScorchedEarth => write!(f, "SE"),
            Map::Genesis1 => write!(f, "Gen1"),
            Map::Genesis2 => write!(f, "Gen2"),
            Map::LostIsland => write!(f, "LostIsland"),
            Map::Fjordur => write!(f, "Fjordur"),
        }
    }
}
struct Server {
    map_name: Map,
}
impl Server {
    fn from(map_name: Map) -> Option<Server> {
        // Überprüfen ob es eine Unit File fur map_name gibt
        let services = std::process::Command::new("systemctl")
            .arg("--user")
            .arg("list-unit-files")
            .arg("--type=service")
            .output()
            .unwrap()
            .stdout;
        let services = String::from_utf8_lossy(&services);

        if services.contains(&map_name.to_string()) {
            Some(Server { map_name })
        } else {
            None
        }
    }
    fn is_active(&self) -> Result<bool, String> {
        let stdout = match self.exec(ServerAction::IsActive) {
            Ok(out) => out.stdout,
            Err(err) => {
                return Err(format!(
                    "Der Status des Servers konnte nicht überprüft werden: {}",
                    err
                ))
            }
        };
        let stdout = String::from_utf8_lossy(&stdout).to_string();

        if stdout.contains("active") || stdout.contains("activating") {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn stop(&self) -> Result<(), String> {
        if !self.is_active()? {
            return Err("Der Server ist schon offline.".to_string());
        }
        match self.exec(ServerAction::Stop) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(
                "Der {} Server konnte nicht gestoppt werden: {}",
                self.map_name, err
            )),
        }
    }
    fn start(&self) -> Result<(), String> {
        if self.is_active()? {
            return Err(format!("Der {} Server läuft schon!", self.map_name));
        }
        match self.exec(ServerAction::Start) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(
                "Der {} Server konnte nicht gestartet werden: {}",
                self.map_name, err
            )),
        }
    }

    fn exec(&self, action: ServerAction) -> io::Result<Output> {
        let result = std::process::Command::new("systemctl")
            .arg("--user")
            .arg(action.to_string())
            .arg(format!("ark{}", &self.map_name))
            .output();
        result
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
