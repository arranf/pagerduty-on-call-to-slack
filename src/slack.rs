use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SlackIncomingWebhookMessage {
    text: String,
}

impl SlackIncomingWebhookMessage {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
