use std::pin::Pin;

use anyhow::Result;
use futures::Stream;
use redis::Msg;

pub mod stream_to_vec_forwarder;

#[derive(Clone, Debug)]
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

//TODO: rename to ChatMessageStream
pub type MessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage>> + Send>>;

pub fn chat_message_from_redis_msg(msg: Msg) -> Result<ChatMessage> {
    Ok(ChatMessage {
        channel: msg.get_channel()?,
        message_text: msg.get_payload()?,
    })
}
