use dotenv::dotenv;
use serenity::model::i
use env_logger;
use log;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    init_logging();

    // Login with a bot token from the environment
    let token: String = read_bot_token();
    let client: Client = init_client(&token).await;
    start_client(client).await;
}

/// Initializes the logger.
///
/// Without calling this function, logging functions like `debug!()` or `info!()` will not work.
fn init_logging() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

/// Returns the Discord bot token from the .env file.
fn read_bot_token() -> String {
    dotenv().ok();
    std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set in .env file.")
}

/// Initializes the serenity Discord bot client object.
async fn init_client(token: &str) -> Client {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client.")
}

/// Starts up the client, and listens for events.
async fn start_client(mut client: Client) {
    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
}
