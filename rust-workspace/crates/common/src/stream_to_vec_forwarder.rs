use std::sync::{Arc, Mutex};

use anyhow::Result;
use futures::TryStreamExt;
use stream_cancel::{Trigger, Tripwire};
use tokio::task::JoinHandle;

use crate::{ChatMessage, ChatMessageStream};

pub struct StreamToVecForwarder {
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
    pub fn new(
        incoming_message_stream: ChatMessageStream,
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
    mut incoming_message_stream: ChatMessageStream,
    message_list: Arc<Mutex<Vec<ChatMessage>>>,
) -> Result<()> {
    while let Some(msg) = incoming_message_stream.try_next().await? {
        let mut message_list_inner = message_list.lock().unwrap();

        message_list_inner.push(msg);
    }

    Ok(())
}
