use std::sync::{Arc, Mutex};

use anyhow::Result;
use futures::TryStreamExt;

use crate::{channel_subscriber::ChannelSubscriber, ChatMessage, MessageStream};

#[derive(Clone)]
pub struct ChatClient {
    channel_subscriber: Arc<dyn ChannelSubscriber>,
    messages_received: Arc<Mutex<Vec<ChatMessage>>>,
}

impl ChatClient {
    pub fn new(channel_subscriber: Arc<dyn ChannelSubscriber>) -> Self {
        ChatClient {
            messages_received: Arc::new(Mutex::new(vec![])),
            channel_subscriber,
        }
    }

    pub fn messages_received(&self) -> Vec<ChatMessage> {
        self.messages_received.lock().unwrap().clone()
    }

    pub async fn subscribe(&self, channel_name: &str) -> Result<()> {
        let incoming_message_stream = self.channel_subscriber.subscribe(channel_name).await?;
        let message_list = Arc::clone(&self.messages_received);

        tokio::spawn(message_reception_worker(
            incoming_message_stream,
            message_list,
        ));

        Ok(())
    }

    pub fn unsubscribe(&self, channel_name: &str) -> Result<()> {
        //TODO
        Ok(())
    }
}

async fn message_reception_worker(
    mut incoming_message_stream: MessageStream,
    message_list: Arc<Mutex<Vec<ChatMessage>>>,
) -> Result<()> {
    while let Some(msg) = incoming_message_stream.try_next().await? {
        let mut message_list_inner = message_list.lock().unwrap();

        message_list_inner.push(msg);
    }

    Ok(())
}
