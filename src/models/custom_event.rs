
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CustomEvent {
    pub status: String,
    pub headline: String,
    pub event_type: String,
    pub command: String
}
