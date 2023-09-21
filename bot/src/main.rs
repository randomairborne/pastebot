#![deny(clippy::all, clippy::pedantic)]

use ahash::AHashMap;
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, Arc},
};
use tokio::task::JoinSet;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use twilight_gateway::{CloseFrame, Config, Event, Intents, Shard};
use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        AllowedMentions, Component, ReactionType,
    },
    gateway::payload::incoming::MessageCreate,
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(concat!(env!("CARGO_PKG_NAME"), "=info").parse().unwrap())
        .from_env()
        .expect("failed to parse env");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .init();
    let token =
        std::env::var("DISCORD_TOKEN").expect("Failed to get DISCORD_TOKEN environment variable");
    let paste_url = std::env::var("PASTEBIN").expect("No PASTEBIN url set!");

    let client = twilight_http::Client::new(token.clone());
    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
    let config = Config::new(token.clone(), intents);
    let shards: Vec<Shard> =
        twilight_gateway::stream::create_recommended(&client, config, |_, builder| builder.build())
            .await
            .expect("Failed to create reccomended shard count")
            .collect();
    let senders: Vec<twilight_gateway::MessageSender> =
        shards.iter().map(twilight_gateway::Shard::sender).collect();
    println!("Connecting to discord");
    let should_shutdown = Arc::new(AtomicBool::new(false));
    let state = AppState {
        should_shutdown,
        paste_url: Arc::new(paste_url),
        http: Arc::new(client),
        del_cache: Arc::new(MessageReplies::new()),
    };
    let mut set = JoinSet::new();
    for shard in shards {
        set.spawn(event_loop(shard, state.clone()));
    }

    #[cfg(not(target_family = "unix"))]
    compile_error!("Windows is not supported");
    let mut term_handler =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = term_handler.recv() => {}
    }

    eprintln!("Shutting down..");

    // Let the shards know not to reconnect
    state
        .should_shutdown
        .store(true, std::sync::atomic::Ordering::Relaxed);

    // Tell the shards to shut down
    for sender in senders {
        sender.close(CloseFrame::NORMAL).ok();
    }

    // Await all tasks to complete.
    while set.join_next().await.is_some() {}
    println!("Done, see ya!");
}

async fn event_loop(mut shard: Shard, state: AppState) {
    loop {
        match shard.next_event().await {
            Ok(event) => {
                trace!(?event, "got event");
                let state = state.clone();
                tokio::spawn(async move {
                    if let Err(source) = handle_event(state, event).await {
                        // this includes even user caused errors. User beware. Don't set up automatic emails or anything.
                        error!(?source, "Handler error");
                    }
                });
            }
            Err(source) => error!(?source, "Shard loop error"),
        }
        if state
            .should_shutdown
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            break;
        }
    }
}

async fn handle_event(state: AppState, event: Event) -> Result<(), Error> {
    match event {
        Event::MessageCreate(msg) => new_message(*msg, state).await?,
        Event::MessageDelete(msg) => message_delete(msg.id, msg.channel_id, state).await?,
        Event::MessageDeleteBulk(msgs) => {
            for msg in msgs.ids {
                let state = state.clone();
                tokio::spawn(message_delete(msg, msgs.channel_id, state));
            }
        }
        Event::ThreadCreate(thread) => {
            let _ = state.http.join_thread(thread.id).await;
        }
        Event::Ready(ready) => {
            info!(
                "Bot ready, shard {}, id {}, name {}",
                ready
                    .shard
                    .map_or_else(|| "unknown".to_string(), |v| v.to_string()),
                ready.user.id,
                ready.user.name
            );
        }
        _ => {}
    };
    Ok(())
}

async fn new_message(message: MessageCreate, state: AppState) -> Result<(), Error> {
    if message.attachments.is_empty() {
        return Ok(());
    }
    let mut buttons: Vec<Button> = Vec::with_capacity(10);
    for attachment in &message.attachments {
        if let Some(ctype) = &attachment.content_type {
            let ctype_lowercase = ctype.to_ascii_lowercase();
            if !ctype_lowercase.contains("charset=utf") {
                debug!(
                    "skipping attachment {} for having a content type of {}",
                    attachment.filename, ctype
                );
                continue;
            }
        }
        let button = Button {
            custom_id: None,
            disabled: false,
            emoji: Some(ReactionType::Unicode {
                name: "📜".to_owned(),
            }),
            label: Some(format!("View {}", attachment.filename)),
            style: ButtonStyle::Link,
            url: Some(format!(
                "{}#/{}/{}/{}",
                state.paste_url, message.channel_id, attachment.id, attachment.filename
            )),
        };
        buttons.push(button);
    }
    // ugh i hate this but there isn't really a better way
    let mut row0 = ActionRow {
        components: Vec::with_capacity(5),
    };
    let mut row1 = ActionRow {
        components: Vec::with_capacity(5),
    };
    for (i, button) in buttons.into_iter().enumerate() {
        if i < 5 {
            row0.components.push(Component::Button(button));
        } else {
            row1.components.push(Component::Button(button));
        }
    }
    let components = if row0.components.is_empty() {
        return Err(Error::InvalidAttachment);
    } else if row1.components.is_empty() {
        vec![Component::ActionRow(row0)]
    } else {
        vec![Component::ActionRow(row0), Component::ActionRow(row1)]
    };
    trace!(?components, "sending components to discord");
    let new_msg = state
        .http
        .create_message(message.channel_id)
        .reply(message.id)
        .content(&format!(
            "Web version of attachments from <@{}>",
            message.author.id
        ))?
        .components(components.as_slice())?
        .allowed_mentions(Some(&AllowedMentions::default()))
        .await?
        .model()
        .await?;
    state.del_cache.insert(message.id, new_msg.id);
    Ok(())
}

async fn message_delete(
    msg_id: Id<MessageMarker>,
    chan_id: Id<ChannelMarker>,
    state: AppState,
) -> Result<(), Error> {
    if let Some(reply_id) = state.del_cache.remove(msg_id) {
        state.http.delete_message(chan_id, reply_id).await?;
    }
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    should_shutdown: Arc<AtomicBool>,
    paste_url: Arc<String>,
    http: Arc<twilight_http::Client>,
    del_cache: Arc<MessageReplies>,
}

pub struct MessageReplies {
    replies: Mutex<AHashMap<Id<MessageMarker>, Id<MessageMarker>>>,
    oldest: Mutex<VecDeque<Id<MessageMarker>>>,
}

impl MessageReplies {
    const MAX_REFERENCES: usize = 1_000_000;
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&self, referenced: Id<MessageMarker>, reply: Id<MessageMarker>) {
        self.replies.lock().insert(referenced, reply);
        let mut oldest = self.oldest.lock();
        oldest.push_back(referenced);
        if oldest.len() > Self::MAX_REFERENCES {
            if let Some(referenced) = oldest.pop_front() {
                drop(oldest);
                self.replies.lock().remove(&referenced);
            }
        }
    }
    /// You probably want [`Self::remove()`] instead.
    pub fn get(&self, referenced: Id<MessageMarker>) -> Option<Id<MessageMarker>> {
        self.replies.lock().get(&referenced).copied()
    }
    pub fn remove(&self, referenced: Id<MessageMarker>) -> Option<Id<MessageMarker>> {
        self.replies.lock().remove(&referenced)
    }
}

impl Default for MessageReplies {
    fn default() -> Self {
        Self {
            replies: Mutex::new(AHashMap::with_capacity(Self::MAX_REFERENCES + 1)),
            oldest: Mutex::new(VecDeque::with_capacity(Self::MAX_REFERENCES + 1)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Twilight-Gateway error: {0}")]
    Send(#[from] twilight_gateway::error::SendError),
    #[error("Twilight-Http error: {0}")]
    Http(#[from] twilight_http::Error),
    #[error("Twilight-Http deserialize error: {0}")]
    HttpBody(#[from] twilight_http::response::DeserializeBodyError),
    #[error("Twilight-Validate message error: {0}")]
    MessageValidation(#[from] twilight_validate::message::MessageValidationError),
    #[error("Invalid attachment detected")]
    InvalidAttachment,
}
