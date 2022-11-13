use std::sync::{Arc, Mutex};

use common::{stream_to_vec_forwarder::StreamToVecForwarder, ChatMessage, ChatMessageStream};

#[derive(Clone)]
pub struct MessageLog {
    //TODO: instead of storing messages in memory, save them in a database/key-value store
    messages_received: Arc<Mutex<Vec<ChatMessage>>>,
    _message_forwarder: Arc<StreamToVecForwarder>,
}

impl MessageLog {
    pub fn new(incoming_messages: ChatMessageStream) -> Self {
        let messages_received: Arc<Mutex<Vec<ChatMessage>>> = Default::default();

        let _message_forwarder = Arc::new(StreamToVecForwarder::new(
            incoming_messages,
            Arc::clone(&messages_received),
        ));

        MessageLog {
            messages_received,
            _message_forwarder,
        }
    }

    //TODO: we need an API that returns messages for a chat channel, not all of them.
    pub fn messages_received(&self) -> Vec<ChatMessage> {
        self.messages_received.lock().unwrap().clone()
    }
}
