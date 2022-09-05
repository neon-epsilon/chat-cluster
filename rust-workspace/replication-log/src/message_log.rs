use std::sync::{Arc, Mutex};

use anyhow::Result;
use futures::TryStreamExt;
use stream_cancel::{Trigger, Tripwire};
use tokio::task::JoinHandle;

use crate::{ChatMessage, MessageStream};

#[derive(Clone)]
pub struct MessageLog {
    //TODO: instead of storing messages in memory, save them in a database/key-value store
    messages_received: Arc<Mutex<Vec<ChatMessage>>>,
    _message_forwarder: Arc<StreamToVecForwarder>,
}

impl MessageLog {
    pub fn new(incoming_messages: MessageStream) -> Self {
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

//TODO: refactor into common crate
struct StreamToVecForwarder {
    /// The handle of the forwarding task. Could be used to check for errors in the stream.
    _message_reception_worker_handle: JoinHandle<Result<()>>,
    /// When this is dropped, the stream is cancelled and we stop forwarding.
    _stream_cancellation_trigger: Trigger,
}

impl StreamToVecForwarder {
    /// Subscribe to the channel represented by the MessageStream, asynchronously writing to the
    /// message list on any new message.
    ///
    /// Will automatically stop writing to the message list when dropped.
    fn new(
        incoming_message_stream: MessageStream,
        message_list: Arc<Mutex<Vec<ChatMessage>>>,
    ) -> Self {
        use stream_cancel::StreamExt;
        let (stream_cancellation_trigger, tripwire) = Tripwire::new();
        let cancellable_stream = incoming_message_stream.take_until_if(tripwire);

        let join_handle = tokio::spawn(forward_messages_to_vec(
            Box::pin(cancellable_stream),
            message_list,
        ));

        Self {
            _message_reception_worker_handle: join_handle,
            _stream_cancellation_trigger: stream_cancellation_trigger,
        }
    }
}

async fn forward_messages_to_vec(
    mut incoming_message_stream: MessageStream,
    message_list: Arc<Mutex<Vec<ChatMessage>>>,
) -> Result<()> {
    while let Some(msg) = incoming_message_stream.try_next().await? {
        let mut message_list_inner = message_list.lock().unwrap();

        message_list_inner.push(msg);
    }

    Ok(())
}
