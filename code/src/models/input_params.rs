use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub text: String,
    pub user: String,
    pub version: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostEventParams {
    pub pk: Option<String>,
    pub sk: Option<String>,
    pub carriers: Vec<String>,
    pub dates: String,
    pub description: String,
    pub event_category: String,
    pub headline: String,
    pub scope: String,
    pub severity: String,
    pub status: String,
    pub step: String,
    pub event_type: String,
    pub read_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetObjParams {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputParams {
    pub command: String,
    pub post_event_params: Option<PostEventParams>,
    pub get_object_params: Option<GetObjParams>,
}
