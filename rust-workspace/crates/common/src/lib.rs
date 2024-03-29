use std::pin::Pin;

use anyhow::Result;
use futures::Stream;
use redis::Msg;
use serde::{Deserialize, Serialize};

pub mod stream_to_vec_forwarder;

pub static DEFAULT_CHANNEL: &str = "default-channel";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub channel: String,
    pub message_text: String,
}

impl ChatMessage {
    pub fn new<S1: Into<String>, S2: Into<String>>(channel: S1, message_text: S2) -> Self {
        ChatMessage {
            channel: channel.into(),
            message_text: message_text.into(),
        }
    }
}

pub type ChatMessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage>> + Send>>;

pub fn chat_message_from_redis_msg(msg: Msg) -> Result<ChatMessage> {
    Ok(ChatMessage {
        channel: msg.get_channel()?,
        message_text: msg.get_payload()?,
    })
}
