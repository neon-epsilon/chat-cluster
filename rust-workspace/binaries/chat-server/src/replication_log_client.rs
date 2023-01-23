use anyhow::Result;
use async_trait::async_trait;

use common::ChatMessage;

#[async_trait]
pub trait ReplicationLogClient {
    async fn get_messages_for_channel(&self, channel_name: &str) -> Result<Vec<ChatMessage>>;
}
