use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::story_points::model::system::{
    card::Card, coffee_vote::CoffeeVote, participant::Participant, ticket::Ticket,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateMessage {
    session_code: String,
    session_name: String,
    available_cards: Vec<Card>,
    participants: Vec<Participant>,
    ticket: Option<Ticket>,
    time_left: u64,
    tags: HashSet<String>,
    spectator_count: u64,
    coffee_request_count: u64,
    coffee_votes: Option<Vec<CoffeeVote>>,
}
