use std::sync::{Arc, Mutex};

use anyhow::Result;
use dashmap::{mapref::entry::Entry, DashMap};

use common::{stream_to_vec_forwarder::StreamToVecForwarder, ChatMessage, ChatMessageStream};

use crate::channel_subscriber::ChannelSubscriber;

#[derive(Clone)]
pub struct ChatServer {
    channel_subscriber: Arc<dyn ChannelSubscriber>,
    messages_received: Arc<Mutex<Vec<ChatMessage>>>,
    active_subscriptions: Arc<DashMap<String, ChannelSubscription>>,
}

impl ChatServer {
    pub fn new(channel_subscriber: Arc<dyn ChannelSubscriber>) -> Self {
        ChatServer {
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
    // TODO: retrieve already sent messages from replication log when subscribing.
    pub async fn subscribe(&self, channel_name: &str) -> Result<bool> {
        match self.active_subscriptions.entry(channel_name.to_string()) {
            Entry::Occupied(_) => Ok(false),
            Entry::Vacant(empty_entry) => {
                //TODO: Retrieve already sent messages from message log after subscription
                let incoming_message_stream =
                    self.channel_subscriber.subscribe(channel_name).await?;
                let message_list = Arc::clone(&self.messages_received);

                let subscription = ChannelSubscription::new(incoming_message_stream, message_list);

                empty_entry.insert(subscription);

                Ok(true)
            }
        }
    }

    /// Returns whether we have been subscribed in the first place.
    pub fn unsubscribe(&self, channel_name: &str) -> bool {
        // The subscription should stop providing messages once it is dropped.
        self.active_subscriptions.remove(channel_name).is_some()
    }
}

struct ChannelSubscription {
    _stream_to_vec_forwarder: StreamToVecForwarder,
}

impl ChannelSubscription {
    /// Subscribe to the channel represented by the MessageStream, asynchronously writing to the
    /// message list on any new message.
    ///
    /// Will automatically stop writing to the message list when dropped.
    fn new(
        incoming_message_stream: ChatMessageStream,
        message_list: Arc<Mutex<Vec<ChatMessage>>>,
    ) -> Self {
        let _stream_to_vec_forwarder =
            StreamToVecForwarder::new(incoming_message_stream, message_list);

        Self {
            _stream_to_vec_forwarder,
        }
    }
}
