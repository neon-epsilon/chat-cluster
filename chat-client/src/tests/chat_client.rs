use std::{sync::Arc, time::Duration};

use anyhow::{Error, Result};
use async_trait::async_trait;
use futures::{future, TryStreamExt};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    channel_subscriber::ChannelSubscriber, chat_client::ChatClient, ChatMessage, MessageStream,
};

#[derive(Clone)]
pub struct MockChannelSubscriber {
    message_sender: Sender<ChatMessage>,
    /// Keep a dummy receiver alive so that we can "publish" messages even if there are no active
    /// subscribers.
    _dummy_receiver: Arc<Receiver<ChatMessage>>,
}

impl MockChannelSubscriber {
    pub fn new() -> Self {
        let (message_sender, receiver) = broadcast::channel(16);
        let _dummy_receiver = Arc::new(receiver);

        MockChannelSubscriber {
            message_sender,
            _dummy_receiver,
        }
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

    insta::assert_debug_snapshot!(chat_client.messages_received(), @"[]");

    let channel_name = "test-channel".to_string();
    chat_client.subscribe(&channel_name).await.unwrap();

    mock_channel_subscriber
        .publish_message(ChatMessage {
            channel: channel_name.clone(),
            message_text: "This message should show up in the client.".to_string(),
        })
        .unwrap();
    // Since the message is handled asynchronously, we have to wait a little.
    tokio::time::sleep(Duration::from_millis(100)).await;
    insta::assert_debug_snapshot!(chat_client.messages_received(), @r###"
    [
        ChatMessage {
            channel: "test-channel",
            message_text: "This message should show up in the client.",
        },
    ]
    "###);

    let unrelated_channel_name = "some-other-channel".to_string();
    mock_channel_subscriber
        .publish_message(ChatMessage {
            channel: unrelated_channel_name,
            message_text: "This message should not show up.".to_string(),
        })
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    insta::assert_debug_snapshot!(chat_client.messages_received(), @r###"
    [
        ChatMessage {
            channel: "test-channel",
            message_text: "This message should show up in the client.",
        },
    ]
    "###);
}

#[tokio::test]
async fn unsubscribe() {
    let mock_channel_subscriber = MockChannelSubscriber::new();
    let chat_client = ChatClient::new(Arc::new(mock_channel_subscriber.clone()));

    let channel_name = "test-channel".to_string();
    chat_client.subscribe(&channel_name).await.unwrap();
    chat_client.unsubscribe(&channel_name);
    // Wait a little otherwise we might still be subscribed.
    tokio::time::sleep(Duration::from_millis(100)).await;

    mock_channel_subscriber
        .publish_message(ChatMessage {
            channel: channel_name.clone(),
            message_text:
                "This message should not show up in the client because we already unsubscribed."
                    .to_string(),
        })
        .unwrap();
    // Make sure the message had enough time to be handled.
    tokio::time::sleep(Duration::from_millis(100)).await;
    insta::assert_debug_snapshot!(chat_client.messages_received(), @"[]");
}
