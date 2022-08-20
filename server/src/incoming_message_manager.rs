use std::pin::Pin;

use async_trait::async_trait;
use futures::StreamExt;
use redis::{Msg, RedisError};
use thiserror::Error;
use warp::Stream;

pub struct ChatMessage {
    pub channel: String,
    pub message_text: String,
}

pub type MessageStream = Pin<Box<dyn Stream<Item = IncomingMessageManagerResult<ChatMessage>>>>;

#[async_trait]
pub trait IncomingMessageManager {
    async fn subscribe(&self, channel_name: &str) -> IncomingMessageManagerResult<MessageStream>;
}

pub struct RedisIncomingMessageManager {
    redis_url: String,
}

impl RedisIncomingMessageManager {
    pub fn new(redis_url: String) -> IncomingMessageManagerResult<Self> {
        Ok(RedisIncomingMessageManager { redis_url })
    }
}

#[async_trait]
impl IncomingMessageManager for RedisIncomingMessageManager {
    async fn subscribe(&self, channel_name: &str) -> IncomingMessageManagerResult<MessageStream> {
        let redis_client = redis::Client::open(self.redis_url.clone())?;
        let connection = redis_client.get_async_connection().await?;
        let mut pubsub = connection.into_pubsub();

        pubsub.subscribe(channel_name).await?;

        let stream = pubsub.into_on_message().map(chat_message_from_redis_msg);

        Ok(Box::pin(stream))
    }
}

fn chat_message_from_redis_msg(msg: Msg) -> IncomingMessageManagerResult<ChatMessage> {
    Ok(ChatMessage {
        channel: msg.get_channel()?,
        message_text: msg.get_payload()?,
    })
}

#[derive(Error, Debug)]
pub enum IncomingMessageManagerError {
    #[error(transparent)]
    Redis(#[from] RedisError),
}

pub type IncomingMessageManagerResult<T> = Result<T, IncomingMessageManagerError>;
