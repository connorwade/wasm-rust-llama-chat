use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub message: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            message: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}
