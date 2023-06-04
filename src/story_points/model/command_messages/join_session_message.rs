use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinSessionMessage {
    session_code: String,
    participant_name: String,
    password: Option<String>,
}