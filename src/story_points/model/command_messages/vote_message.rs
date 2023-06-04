use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteMessage {
    session_code: String,
    password: Option<String>,
}