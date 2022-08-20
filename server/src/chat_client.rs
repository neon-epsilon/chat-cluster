use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::incoming_message_manager::IncomingMessageManager;

#[derive(Clone)]
pub struct ChatClient {
    incoming_message_manager: Arc<dyn IncomingMessageManager>,
    messages_received: Arc<Mutex<Vec<String>>>,
}

impl ChatClient {
    pub fn new(incoming_message_manager: Arc<dyn IncomingMessageManager>) -> Self {
        ChatClient {
            messages_received: Arc::new(Mutex::new(vec![
                "Yoohoo, dummy message, you big dummy!".to_string()
            ])),
            incoming_message_manager,
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
