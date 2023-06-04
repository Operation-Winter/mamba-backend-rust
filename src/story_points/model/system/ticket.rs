use serde::{Deserialize, Serialize};
use std::{vec::Vec, collections::HashSet};

use super::ticket_vote::TicketVote;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    title: String,
    description: String,
    selected_tags: HashSet<String>,
    ticket_votes: Vec<TicketVote>,
}