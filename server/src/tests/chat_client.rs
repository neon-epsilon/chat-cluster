use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use futures::{future, TryStreamExt};
use tokio::sync::broadcast::{self, Sender};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    channel_subscriber::ChannelSubscriber, chat_client::ChatClient, ChatMessage, MessageStream,
};

#[derive(Clone)]
pub struct MockChannelSubscriber {
    message_sender: Sender<ChatMessage>,
}

impl MockChannelSubscriber {
    pub fn new() -> Self {
        let (message_sender, _receiver) = broadcast::channel(16);

        MockChannelSubscriber { message_sender }
    }

    pub fn publish_message(&self, msg: ChatMessage) -> Result<()> {
        let _num_receivers = self.message_sender.send(msg)?;

        Ok(())
    }
}

#[async_trait]
impl ChannelSubscriber for MockChannelSubscriber {
    async fn subscribe(&self, channel_name: &str) -> Result<MessageStream> {
        let channel_name = channel_name.to_string();
        let message_receiver = self.message_sender.subscribe();
        let stream = BroadcastStream::new(message_receiver)
            .try_filter(move |msg| future::ready(msg.channel == channel_name))
            .map_err(Error::from);

        Ok(Box::pin(stream))
    }
}

#[tokio::test]
async fn subscribe() {
    let mock_channel_subscriber = MockChannelSubscriber::new();

    let chat_client = ChatClient::new(Arc::new(mock_channel_subscriber.clone()));

    insta::assert_debug_snapshot!(chat_client.messages_received(), @"");
    //TODO: Subscribe, send some messages, assert that they were received. Then send some messages
    //with another channel name, assert that they were not received.
}
