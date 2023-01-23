use anyhow::Result;
use async_trait::async_trait;

use common::ChatMessage;

#[async_trait]
pub trait ReplicationLogClient {
    async fn get_messages_for_channel(&self, channel_name: &str) -> Result<Vec<ChatMessage>>;
}

pub struct ReqwestReplicationLogClient {
    pub replication_log_url: String,
}

#[async_trait]
impl ReplicationLogClient for ReqwestReplicationLogClient {
    async fn get_messages_for_channel(&self, channel_name: &str) -> Result<Vec<ChatMessage>> {
        let replication_log_url = &self.replication_log_url;
        let url = format!("{replication_log_url}/{channel_name}");
        let body = reqwest::get(url).await?.text().await?;

        let deserialized_messages: Vec<ChatMessage> = serde_json::from_str(&body)?;

        Ok(deserialized_messages)
    }
}
