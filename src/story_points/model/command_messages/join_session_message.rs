use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinSessionMessage {
    session_code: String,
    participant_name: String,
    password: Option<String>,
}