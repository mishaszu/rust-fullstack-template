use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub text: String,
}

impl Hello {
    pub fn new(text: &str) -> Self {
        Hello {
            text: String::from(text),
        }
    }
}
