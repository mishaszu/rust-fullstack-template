use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hello {
    text: String,
}
