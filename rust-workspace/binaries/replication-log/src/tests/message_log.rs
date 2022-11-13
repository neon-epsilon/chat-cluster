use std::time::Duration;

use common::ChatMessage;
use futures::StreamExt;

use crate::message_log::MessageLog;

use super::TestMessageStream;

#[tokio::test]
async fn retrieve_messages() {
    let test_message_stream = TestMessageStream::new(vec![
        ChatMessage::new("default-channel", "first message"),
        ChatMessage::new("some-other-channel", "second message"),
        ChatMessage::new("yet-another-channel", "third message"),
    ])
    .boxed();

    let message_log = MessageLog::new(test_message_stream);

    // Since the message is handled asynchronously, we have to wait a little.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // assert that the messages can be retrieved
    insta::assert_debug_snapshot!(message_log.messages_received(), @r###"
    [
        ChatMessage {
            channel: "default-channel",
            message_text: "first message",
        },
        ChatMessage {
            channel: "some-other-channel",
            message_text: "second message",
        },
        ChatMessage {
            channel: "yet-another-channel",
            message_text: "third message",
        },
    ]
    "###);
}
