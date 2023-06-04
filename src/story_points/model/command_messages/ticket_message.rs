use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketMessage {
    title: String,
    description: String,
    selected_tags: HashSet<String>,
}