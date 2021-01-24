use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
