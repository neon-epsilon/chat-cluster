use std::{sync::Arc, time::Duration};

use anyhow::{Error, Result};
use async_trait::async_trait;
use futures::{future, TryStreamExt};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio_stream::wrappers::BroadcastStream;

use common::{ChatMessage, ChatMessageStream};

use crate::{
    channel_subscriber::ChannelSubscriber, chat_server::ChatServer,
    replication_log_client::ReplicationLogClient,
};

#[derive(Clone)]
struct MockChannelSubscriber {
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

struct MockReplicationLogClient {
    pub messages: Vec<ChatMessage>,
}

#[async_trait]
impl ReplicationLogClient for MockReplicationLogClient {
    async fn get_messages_for_channel(&self, channel_name: &str) -> Result<Vec<ChatMessage>> {
        let messages_for_channel = self
            .messages
            .iter()
            .filter(|chat_message| chat_message.channel == channel_name)
            .cloned()
            .collect();
        Ok(messages_for_channel)
    }
}

#[async_trait]
impl ChannelSubscriber for MockChannelSubscriber {
    async fn subscribe(&self, channel_name: &str) -> Result<ChatMessageStream> {
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
    let mock_replication_log_client = MockReplicationLogClient { messages: vec![] };
    let chat_server = ChatServer::new(
        Arc::new(mock_channel_subscriber.clone()),
        Arc::new(mock_replication_log_client),
    );

    insta::assert_debug_snapshot!(chat_server.messages_received(), @"[]");

    let channel_name = "test-channel".to_string();
    chat_server.subscribe(&channel_name).await.unwrap();

    mock_channel_subscriber
        .publish_message(ChatMessage {
            channel: channel_name.clone(),
            message_text: "This message should show up in the client.".to_string(),
        })
        .unwrap();
    // Since the message is handled asynchronously, we have to wait a little.
    tokio::time::sleep(Duration::from_millis(100)).await;
    insta::assert_debug_snapshot!(chat_server.messages_received(), @r###"
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
    insta::assert_debug_snapshot!(chat_server.messages_received(), @r###"
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
    let mock_replication_log_client = MockReplicationLogClient { messages: vec![] };
    let chat_server = ChatServer::new(
        Arc::new(mock_channel_subscriber.clone()),
        Arc::new(mock_replication_log_client),
    );

    let channel_name = "test-channel".to_string();
    chat_server.subscribe(&channel_name).await.unwrap();

    mock_channel_subscriber
        .publish_message(ChatMessage {
            channel: channel_name.clone(),
            message_text: "This message should only show up until we unsubscribe.".to_string(),
        })
        .unwrap();
    // Make sure the message had enough time to be handled.
    tokio::time::sleep(Duration::from_millis(100)).await;
    insta::assert_debug_snapshot!(chat_server.messages_received(), @r###"
    [
        ChatMessage {
            channel: "test-channel",
            message_text: "This message should only show up until we unsubscribe.",
        },
    ]
    "###);

    chat_server.unsubscribe(&channel_name);
    insta::assert_debug_snapshot!(chat_server.messages_received(), @"[]");

    mock_channel_subscriber
        .publish_message(ChatMessage {
            channel: channel_name.clone(),
            message_text:
                "This message should not show up in the client because we already unsubscribed."
                    .to_string(),
        })
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    insta::assert_debug_snapshot!(chat_server.messages_received(), @"[]");
}

#[tokio::test]
async fn retrieve_messages_from_replication_log() {
    let mock_channel_subscriber = MockChannelSubscriber::new();
    let mock_replication_log_client = MockReplicationLogClient {
        messages: vec![
            ChatMessage::new("test-channel1", "message 1 on test-channel1"),
            ChatMessage::new("test-channel2", "message 1 on test-channel2"),
            ChatMessage::new("test-channel1", "message 2 on test-channel1"),
            ChatMessage::new("test-channel2", "message 2 on test-channel2"),
        ],
    };

    let chat_server = ChatServer::new(
        Arc::new(mock_channel_subscriber.clone()),
        Arc::new(mock_replication_log_client),
    );
    insta::assert_debug_snapshot!(chat_server.messages_received(), @"[]");

    chat_server.subscribe("test-channel1").await.unwrap();
    insta::assert_debug_snapshot!(chat_server.messages_received(), @r###"
    [
        ChatMessage {
            channel: "test-channel1",
            message_text: "message 1 on test-channel1",
        },
        ChatMessage {
            channel: "test-channel1",
            message_text: "message 2 on test-channel1",
        },
    ]
    "###);

    chat_server.subscribe("test-channel2").await.unwrap();
    insta::assert_debug_snapshot!(chat_server.messages_received(), @r###"
    [
        ChatMessage {
            channel: "test-channel1",
            message_text: "message 1 on test-channel1",
        },
        ChatMessage {
            channel: "test-channel1",
            message_text: "message 2 on test-channel1",
        },
        ChatMessage {
            channel: "test-channel2",
            message_text: "message 1 on test-channel2",
        },
        ChatMessage {
            channel: "test-channel2",
            message_text: "message 2 on test-channel2",
        },
    ]
    "###);
}
