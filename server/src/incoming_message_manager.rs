use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use futures::StreamExt;
use redis::Msg;
use warp::Stream;

pub struct ChatMessage {
    pub channel: String,
    pub message_text: String,
}

pub type MessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage>>>>;

#[async_trait]
pub trait IncomingMessageManager: Send + Sync {
    async fn subscribe(&self, channel_name: &str) -> Result<MessageStream>;
}

pub struct RedisIncomingMessageManager {
    redis_url: String,
}

impl RedisIncomingMessageManager {
    pub fn new(redis_url: String) -> Self {
        RedisIncomingMessageManager { redis_url }
    }
}

#[async_trait]
impl IncomingMessageManager for RedisIncomingMessageManager {
    async fn subscribe(&self, channel_name: &str) -> Result<MessageStream> {
        let redis_client = redis::Client::open(self.redis_url.clone())?;
        let connection = redis_client.get_async_connection().await?;
        let mut pubsub = connection.into_pubsub();

        pubsub.subscribe(channel_name).await?;

        let stream = pubsub.into_on_message().map(chat_message_from_redis_msg);

        Ok(Box::pin(stream))
    }
}

fn chat_message_from_redis_msg(msg: Msg) -> Result<ChatMessage> {
    Ok(ChatMessage {
        channel: msg.get_channel()?,
        message_text: msg.get_payload()?,
    })
}
