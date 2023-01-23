use common::{ChatMessage, DEFAULT_CHANNEL};
use httpmock::prelude::{MockServer, GET};

use crate::replication_log_client::{ReplicationLogClient, ReqwestReplicationLogClient};

#[tokio::test]
async fn reqwest_client_get_messages() {
    let server = MockServer::start();

    let messages_on_default_channel = vec![ChatMessage::new(DEFAULT_CHANNEL, "test-message1")];
    let messages_on_other_channel = vec![
        ChatMessage::new("other-channel", "test-message2"),
        ChatMessage::new("other-channel", "test-message3"),
    ];

    let _default_channel_mock = server.mock(|when, then| {
        when.method(GET).path(format!("/{}", DEFAULT_CHANNEL));
        then.status(200)
            .body(serde_json::to_string(&messages_on_default_channel).unwrap());
    });
    let _other_channel_mock = server.mock(|when, then| {
        when.method(GET).path("/other-channel");
        then.status(200)
            .body(serde_json::to_string(&messages_on_other_channel).unwrap());
    });

    let client = ReqwestReplicationLogClient {
        replication_log_url: server.base_url(),
    };

    let retrieved_messages_for_default_channel = client
        .get_messages_for_channel(DEFAULT_CHANNEL)
        .await
        .unwrap();
    insta::assert_debug_snapshot!(retrieved_messages_for_default_channel, @r###"
    [
        ChatMessage {
            channel: "default-channel",
            message_text: "test-message1",
        },
    ]
    "###);

    let retrieved_messages_for_other_channel = client
        .get_messages_for_channel("other-channel")
        .await
        .unwrap();
    insta::assert_debug_snapshot!(retrieved_messages_for_other_channel, @r###"
    [
        ChatMessage {
            channel: "other-channel",
            message_text: "test-message2",
        },
        ChatMessage {
            channel: "other-channel",
            message_text: "test-message3",
        },
    ]
    "###);
}
