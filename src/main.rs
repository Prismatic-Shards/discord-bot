use std::{io::Error, path::Path};

use crate::{app::App, handler::Handler};

pub mod app;
pub mod handler;

use chrono::{DateTime, Local};
use serenity::{
  all::{ApplicationId, GuildId},
  prelude::*,
};
use tokio::{fs::File, sync::mpsc};

const APPLICATION_ID: u64 = 1487934102438023402;
const GUILD_ID: GuildId = GuildId::new(1439707494896373833);

#[tokio::main]
async fn main() -> Result<(), Error> {
  let (logger_tx, mut logger_rx) = mpsc::channel::<()>(100);

  let now = Local::now();
  let path = format!("{}.txt", now.format("%Y-%m-%d_%H-%M-%S"));
  let path = Path::new(path.as_str());
  let display = path.display();
  // Open a file in write-only mode, returns `io::Result<File>`
  let mut file = match File::create(&path).await {
    Err(why) => panic!("couldn't create {}: {}", display, why),
    Ok(file) => file,
  };

  let mut discord: Client = Client::builder(
    "asfdasfd",
    GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT,
  )
  .event_handler(Handler {})
  .await
  .expect("Err creating client");

  discord
    .http
    .set_application_id(ApplicationId::new(APPLICATION_ID));

  let http = discord.http.clone();

  let bot = tokio::spawn(async move { discord.start().await });

  let mut app = App::new(http);

  ratatui::run(|terminal| app.run(terminal))?;

  bot.abort();

  Ok(())
}
