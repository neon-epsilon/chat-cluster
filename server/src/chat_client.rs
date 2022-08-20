use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::{channel_subscriber::ChannelSubscriber, ChatMessage};

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

    pub fn subscribe(&self, channel_name: &str) -> Result<()> {
        todo!()
    }

    pub fn unsubscribe(&self, channel_name: &str) -> Result<()> {
        todo!()
    }
}
