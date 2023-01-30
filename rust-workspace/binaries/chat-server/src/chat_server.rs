use std::sync::{Arc, Mutex};

use anyhow::Result;
use dashmap::{mapref::entry::Entry, DashMap};

use common::{stream_to_vec_forwarder::StreamToVecForwarder, ChatMessage, ChatMessageStream};

use crate::{channel_subscriber::ChannelSubscriber, replication_log_client::ReplicationLogClient};

#[derive(Clone)]
pub struct ChatServer {
    active_subscriptions: Arc<DashMap<String, ChannelSubscription>>,
    channel_subscriber: Arc<dyn ChannelSubscriber>,
    messages_received: Arc<Mutex<Vec<ChatMessage>>>,
    replication_log_client: Arc<dyn ReplicationLogClient>,
}

impl ChatServer {
    pub fn new(
        channel_subscriber: Arc<dyn ChannelSubscriber>,
        replication_log_client: Arc<dyn ReplicationLogClient>,
    ) -> Self {
        ChatServer {
            active_subscriptions: Default::default(),
            channel_subscriber,
            messages_received: Default::default(),
            replication_log_client,
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

                // Retrieve messages that were previously sent on the channel.
                //
                // There is a chance that this already contains messages that are also sent to
                // incoming_message_stream - this could be avoided by identifying the messages via
                // some sort of id.
                //
                // There is also a small chance that a message takes a long time to arrive at the
                // replication_log_client but we're too late to receive it from the
                // channel_subscriber - in this case we will never receive the message. This could
                // be avoided by waiting a bit after subscribing to the channel before retrieving
                // the previous messages.
                let previous_messages = self
                    .replication_log_client
                    .get_messages_for_channel(channel_name)
                    .await?;
                {
                    // Open a new block to reduce the scope in which the mutex is being held.
                    let mut message_list = self.messages_received.lock().unwrap();
                    message_list.extend(previous_messages);
                }

                let message_list_clone = Arc::clone(&self.messages_received);
                let subscription =
                    ChannelSubscription::new(incoming_message_stream, message_list_clone);

                empty_entry.insert(subscription);
                Ok(true)
            }
        }
    }

    /// Returns whether we have been subscribed in the first place.
    pub fn unsubscribe(&self, channel_name: &str) -> bool {
        // The subscription should stop providing messages once it is dropped.
        let was_subscribed = self.active_subscriptions.remove(channel_name).is_some();

        if was_subscribed {
            let mut message_list = self.messages_received.lock().unwrap();
            message_list.retain(|chat_message| chat_message.channel != channel_name);
        }

        was_subscribed
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
