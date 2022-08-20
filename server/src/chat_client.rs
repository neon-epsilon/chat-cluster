use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ChatClient {
    messages_received: Arc<Mutex<Vec<String>>>,
}

impl ChatClient {
    pub fn new() -> Self {
        ChatClient {
            messages_received: Arc::new(Mutex::new(vec![
                "Yoohoo, dummy message, you big dummy!".to_string()
            ])),
        }
    }

    pub fn messages_received(&self) -> Vec<String> {
        self.messages_received.lock().unwrap().clone()
    }
}
