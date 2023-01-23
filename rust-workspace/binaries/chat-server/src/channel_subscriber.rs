use anyhow::Result;
use async_trait::async_trait;
use futures::StreamExt;

use common::ChatMessageStream;

#[async_trait]
pub trait ChannelSubscriber: Send + Sync {
    async fn subscribe(&self, channel_name: &str) -> Result<ChatMessageStream>;
}

pub struct RedisChannelSubscriber {
    redis_url: String,
}

impl RedisChannelSubscriber {
    pub fn new(redis_url: String) -> Self {
        RedisChannelSubscriber { redis_url }
    }
}

#[async_trait]
impl ChannelSubscriber for RedisChannelSubscriber {
    async fn subscribe(&self, channel_name: &str) -> Result<ChatMessageStream> {
        let redis_client = redis::Client::open(self.redis_url.clone())?;
        let connection = redis_client.get_async_connection().await?;
        let mut pubsub = connection.into_pubsub();

        pubsub.subscribe(channel_name).await?;

        let stream = pubsub
            .into_on_message()
            .map(common::chat_message_from_redis_msg);

        Ok(Box::pin(stream))
    }
}
