use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use common::ChatMessage;
use futures::Stream;

mod message_log;

struct TestMessageStream {
    messages: Vec<ChatMessage>,
}

impl TestMessageStream {
    pub fn new(mut messages: Vec<ChatMessage>) -> Self {
        // The stream pops from the back; we need to reverse.
        messages.reverse();

        TestMessageStream { messages }
    }
}

impl Stream for TestMessageStream {
    type Item = Result<ChatMessage>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.get_mut().messages.pop().map(Result::Ok))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.messages.len();

        (size, Some(size))
    }
}
