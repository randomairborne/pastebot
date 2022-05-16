mod event;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;

const PASTE_SITE: &str = "https://paste.valk.sh";

#[macro_use]
extern crate tracing;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = event::message(ctx, msg).await {
            error!("{:?}", err)
        }
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        message_id: MessageId,
        _guild_id: Option<GuildId>,
    ) {
        if let Err(err) = event::message_delete(ctx, channel_id, message_id).await {
            error!("{:?}", err)
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    tracing_subscriber::fmt::init();
    let mut client = Client::builder(
        token,
        create_intents!(
            GatewayIntents::GUILD_MESSAGES,
            GatewayIntents::MESSAGE_CONTENT
        ),
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}

#[macro_export]
macro_rules! create_intents {
    ( $( $x:expr ),* ) => {
        {
            let mut intents = serenity::prelude::GatewayIntents::empty();
            $(
                intents.insert($x);
            )*
            intents
        }
    };
}
