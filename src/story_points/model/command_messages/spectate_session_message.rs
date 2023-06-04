use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpectateSessionMessage {
    session_code: String,
    password: Option<String>,
}