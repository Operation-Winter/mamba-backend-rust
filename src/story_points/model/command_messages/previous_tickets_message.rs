use serde::{Deserialize, Serialize};
use crate::story_points::model::system::ticket::Ticket;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousTicketsMessage {
    previous_tickets: Vec<Ticket>,
}