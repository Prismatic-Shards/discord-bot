use std::io::Write;
use std::{fs::OpenOptions, io::BufWriter, path::Path};

use crate::logger::LogMessage;
use crate::{app::App, handler::Handler};

pub mod app;
pub mod handler;
pub mod logger;

use serenity::{
  all::{ApplicationId, GuildId},
  prelude::*,
};
use tokio::sync::mpsc;

const APPLICATION_ID: u64 = 1487934102438023402;
const GUILD_ID: GuildId = GuildId::new(1439707494896373833);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (logger_tx, mut logger_rx) = mpsc::channel::<LogMessage>(100);
  let mut writer = logger::get_log_file()?;

  let mut discord: Client = Client::builder(
    "",
    GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT,
  )
  .event_handler(Handler::new(logger_tx.clone()))
  .await?;

  discord
    .http
    .set_application_id(ApplicationId::new(APPLICATION_ID));

  let http = discord.http.clone();

  let bot = tokio::spawn(async move { discord.start().await });
  let logger = tokio::spawn(async move {
    while let Some(message) = logger_rx.recv().await {
      let abort = matches!(message, LogMessage::ShutDown);
      logger::write_entry(&mut writer, message).await.ok();
      if abort {
        return;
      };
    }
  });

  let mut app = App::new(http);

  ratatui::run(|terminal| app.run(terminal))?;

  bot.abort();

  logger_tx.send(LogMessage::ShutDown).await.unwrap();
  logger.await.unwrap();

  Ok(())
}
