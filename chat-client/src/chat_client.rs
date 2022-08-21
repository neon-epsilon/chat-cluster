use std::sync::{Arc, Mutex};

use anyhow::Result;
use dashmap::{mapref::entry::Entry, DashMap};
use futures::TryStreamExt;
use stream_cancel::{Trigger, Tripwire};
use tokio::task::JoinHandle;

use crate::{channel_subscriber::ChannelSubscriber, ChatMessage, MessageStream};

#[derive(Clone)]
pub struct ChatClient {
    channel_subscriber: Arc<dyn ChannelSubscriber>,
    messages_received: Arc<Mutex<Vec<ChatMessage>>>,
    active_subscriptions: Arc<DashMap<String, ChannelSubscription>>,
}

impl ChatClient {
    pub fn new(channel_subscriber: Arc<dyn ChannelSubscriber>) -> Self {
        ChatClient {
            channel_subscriber,
            messages_received: Default::default(),
            active_subscriptions: Default::default(),
        }
    }

    pub fn messages_received(&self) -> Vec<ChatMessage> {
        self.messages_received.lock().unwrap().clone()
    }

    /// Returns whether the subscription was newly created, similar to
    /// [`HashSet::insert`](std::collections::HashSet::insert).
    pub async fn subscribe(&self, channel_name: &str) -> Result<bool> {
        match self.active_subscriptions.entry(channel_name.to_string()) {
            Entry::Occupied(_) => Ok(false),
            Entry::Vacant(empty_entry) => {
                let incoming_message_stream =
                    self.channel_subscriber.subscribe(channel_name).await?;
                let message_list = Arc::clone(&self.messages_received);

                let subscription = ChannelSubscription::new(incoming_message_stream, message_list);

                empty_entry.insert(subscription);

                Ok(true)
            }
        }
    }

    pub fn unsubscribe(&self, channel_name: &str) -> Result<()> {
        //TODO
        Ok(())
    }
}

struct ChannelSubscription {
    _message_reception_worker_handle: JoinHandle<Result<()>>,
    stream_cancellation_trigger: Trigger,
}

impl ChannelSubscription {
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

        let join_handle = tokio::spawn(forward_messages_to_list(
            Box::pin(cancellable_stream),
            message_list,
        ));

        Self {
            _message_reception_worker_handle: join_handle,
            stream_cancellation_trigger,
        }
    }
}

async fn forward_messages_to_list(
    mut incoming_message_stream: MessageStream,
    message_list: Arc<Mutex<Vec<ChatMessage>>>,
) -> Result<()> {
    while let Some(msg) = incoming_message_stream.try_next().await? {
        let mut message_list_inner = message_list.lock().unwrap();

        message_list_inner.push(msg);
    }

    Ok(())
}
