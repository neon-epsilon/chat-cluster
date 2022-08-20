use std::pin::Pin;

use anyhow::Result;
use futures::Stream;

pub mod channel_subscriber;
pub mod chat_client;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub channel: String,
    pub message_text: String,
}

pub type MessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage>>>>;
