use anyhow::Result;
use async_trait::async_trait;

use crate::incoming_message_manager::{ChannelSubscriber, MessageStream};

pub struct MockChannelSubscriber {}

#[async_trait]
impl ChannelSubscriber for MockChannelSubscriber {
    async fn subscribe(&self, channel_name: &str) -> Result<MessageStream> {
        todo!()
    }
}
