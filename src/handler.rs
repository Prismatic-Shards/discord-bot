use serenity::all::Ready;
use serenity::prelude::*;
use serenity::{all::EventHandler, async_trait};
use tokio::sync::mpsc::Sender;

use crate::logger::LogMessage;

pub struct Handler {
  logger: Sender<LogMessage>,
}

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, context: Context, ready: Ready) {}
}

impl Handler {
  pub fn new(logger: Sender<LogMessage>) -> Self {
    Self { logger }
  }
}
