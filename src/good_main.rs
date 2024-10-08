use ::serenity::{all::ClientBuilder, Client};
use dotenv::dotenv;
use log::error;
use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(context_menu_command = "Pute")]
async fn pute(
    ctx: Context<'_>,
    #[description = "Pute sale pute"] message: serenity::Message,
) -> Result<(), Error> {
    let msg = format!("SALE PUTE ! {}", message.content);
    ctx.say(msg).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    init_logging();

    let token: String = read_bot_token();
    let mut client: Client = init_client(&token).await;

    // Start the bot
    if let Err(e) = client.start().await {
        error!("Failed to start the client: {e}");
    }
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
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![pute()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client.")
}
