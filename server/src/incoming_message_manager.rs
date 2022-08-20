use futures::StreamExt;
use redis::{Msg, RedisError};
use thiserror::Error;
use warp::Stream;

pub struct ChatMessage {
    pub channel: String,
    pub message_text: String,
}

pub struct IncomingMessageManager {
    redis_url: String,
}

impl IncomingMessageManager {
    pub fn new(redis_url: String) -> IncomingMessageManagerResult<Self> {
        Ok(IncomingMessageManager { redis_url })
    }

    pub async fn subscribe(
        &mut self,
        channel_name: &str,
    ) -> IncomingMessageManagerResult<impl Stream<Item = IncomingMessageManagerResult<ChatMessage>>>
    {
        let redis_client = redis::Client::open(self.redis_url.clone())?;
        let connection = redis_client.get_async_connection().await?;
        let mut pubsub = connection.into_pubsub();

        pubsub.subscribe(channel_name).await?;

        let stream = pubsub.into_on_message().map(chat_message_from_redis_msg);

        Ok(stream)
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
