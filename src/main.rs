mod commands;

use reqwest::Url;
use serenity::{
  all::{
    ActivityData, ApplicationId, CommandOptionType, CreateCommand, CreateCommandOption, GuildId,
    Message, Ready,
  },
  async_trait,
  prelude::*,
};
use std::{
  env,
  io::{Write, stdin, stdout},
};

const APPLICATION_ID: u64 = 1487934102438023402;
const GUILD_ID: GuildId = GuildId::new(1439707494896373833);

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "!regencommands" && msg.author.id.eq(&1016862766079938662) {
      let commands = ctx
        .http
        .get_guild_commands(GUILD_ID)
        .await
        .expect("Err getting global commands");

      for command in commands {
        ctx
          .http
          .delete_guild_command(GUILD_ID, command.id)
          .await
          .expect("Err deleting command");
      }

      ctx
        .http
        .create_guild_command(
          GUILD_ID,
          &CreateCommand::new("info")
            .description("Get info on random things")
            .add_option(CreateCommandOption::new(
              CommandOptionType::String,
              "thing",
              "What do you want more info on?",
            )),
        )
        .await
        .map_err(|e| println!("Err creating command: {}", e))
        .ok();

      msg.reply(ctx.http, "regenerated commands").await.ok();
    }
  }

  async fn ready(&self, ctx: Context, _data: Ready) {
    let mut activity = ActivityData::playing("Stellarity");

    activity.url = Some(
      Url::parse("https://github.com/Prismatic-Shards/Stellarity").expect("Expected correct url"),
    );
    activity.kind = serenity::all::ActivityType::Playing;
    activity.state = Some(String::from("Stellarity on top!"));
    ctx.set_activity(Some(activity));
  }
}

#[tokio::main]

async fn main() {
  let token = env::var("BOT_TOKEN").ok().unwrap_or_else(|| {
    let mut buffer = String::new();
    println!("Bot Token: ");
    stdout().flush().ok();

    stdin()
      .read_line(&mut buffer)
      .expect("We need a string dumbass");
    println!();

    buffer
  });

  let mut client: Client = Client::builder(
    &token,
    GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT,
  )
  .event_handler(Handler {})
  .await
  .expect("Err creating client");

  client
    .http
    .set_application_id(ApplicationId::new(APPLICATION_ID));

  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  };
}
