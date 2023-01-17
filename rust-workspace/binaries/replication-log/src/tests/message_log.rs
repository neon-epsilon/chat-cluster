use std::time::Duration;

use common::{ChatMessage, DEFAULT_CHANNEL};
use futures::StreamExt;

use crate::message_log::MessageLog;

use super::TestMessageStream;

#[tokio::test]
async fn retrieve_messages() {
    let test_message_stream = TestMessageStream::new(vec![
        ChatMessage::new(DEFAULT_CHANNEL, "first message"),
        ChatMessage::new("some-other-channel", "second message"),
        ChatMessage::new("yet-another-channel", "third message"),
    ])
    .boxed();

    let message_log = MessageLog::new(test_message_stream);

    // Since the message is handled asynchronously, we have to wait a little.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // assert that the messages can be retrieved
    insta::assert_debug_snapshot!(message_log.messages_received(DEFAULT_CHANNEL), @r###"
    [
        ChatMessage {
            channel: "default-channel",
            message_text: "first message",
        },
    ]
    "###);

    insta::assert_debug_snapshot!(message_log.messages_received("some-other-channel"), @r###"
    [
        ChatMessage {
            channel: "some-other-channel",
            message_text: "second message",
        },
    ]
    "###);

    insta::assert_debug_snapshot!(message_log.messages_received("yet-another-channel"), @r###"
    [
        ChatMessage {
            channel: "yet-another-channel",
            message_text: "third message",
        },
    ]
    "###);
}
