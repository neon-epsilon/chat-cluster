use anyhow::Result;
use async_trait::async_trait;

use crate::incoming_message_manager::{IncomingMessageManager, MessageStream};

pub struct MockIncomingMessageManager {}

#[async_trait]
impl IncomingMessageManager for MockIncomingMessageManager {
    async fn subscribe(&self, channel_name: &str) -> Result<MessageStream> {
        todo!()
    }
}
