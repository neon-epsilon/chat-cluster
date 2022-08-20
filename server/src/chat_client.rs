use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::channel_subscriber::ChannelSubscriber;

#[derive(Clone)]
pub struct ChatClient {
    channel_subscriber: Arc<dyn ChannelSubscriber>,
    messages_received: Arc<Mutex<Vec<String>>>,
}

impl ChatClient {
    pub fn new(incoming_message_manager: Arc<dyn ChannelSubscriber>) -> Self {
        ChatClient {
            messages_received: Arc::new(Mutex::new(vec![
                "Yoohoo, dummy message, you big dummy!".to_string()
            ])),
            channel_subscriber: incoming_message_manager,
        }
    }

    pub fn messages_received(&self) -> Vec<String> {
        self.messages_received.lock().unwrap().clone()
    }

    pub fn subscribe(&self, channel_name: &str) -> Result<()> {
        todo!()
    }

    pub fn unsubscribe(&self, channel_name: &str) -> Result<()> {
        todo!()
    }
}
