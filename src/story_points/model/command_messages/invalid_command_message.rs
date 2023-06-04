use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidCommandMessage {
    code: String,
    description: String,
}