use std::env;

use anyhow::Result;
use serenity::model::prelude::{EmojiId, Message, ReactionType};
use serenity::{async_trait, prelude::*};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "hi" {
            info!("Received hi from user={}", msg.author.name);
            if let Err(why) = on_hi(ctx, &msg).await {
                error!("Failed to handle hi: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await?;

    client.start().await?;

    Ok(())
}

async fn on_hi(ctx: Context, msg: &Message) -> Result<()> {
    let emoji_id = EmojiId::new(env::var("EMOJI_ID")?.parse()?);
    let emoji_name = env::var("EMOJI_NAME")?;

    msg.react(
        &ctx.http,
        ReactionType::Custom {
            animated: false,
            id: emoji_id,
            name: Some(emoji_name),
        },
    )
    .await?;

    Ok(())
}
