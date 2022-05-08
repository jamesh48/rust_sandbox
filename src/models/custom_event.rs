
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CustomEvent {
    pub first_name: String,
    pub last_name: String,
    pub command: String
}
