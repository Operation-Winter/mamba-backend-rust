use std::collections::HashSet;

use crate::{
    story_points::model::{
        host_command::host_client_to_server_command::HostClientToServerCommand, system::card::Card, command_messages::start_session_message::StartSessionMessage,
    },
};
use uuid::Uuid;

static START_COMMAND_JSON: &str = "{\"type\":\"START_SESSION\",\"uuid\":\"ca4dfcce-c671-4bbc-b9a8-d8a74a4946c7\",\"message\":{\"sessionName\":\"Test session\",\"autoCompleteVoting\":true,\"availableCards\":[\"ONE\",\"TWO\"],\"password\":null,\"tags\":[\"iOS\"]}}";

#[test]
fn test_start_session_serialize() {
    let start_command = HostClientToServerCommand::StartSession {
        uuid: Uuid::parse_str(&String::from("ca4dfcce-c671-4bbc-b9a8-d8a74a4946c7")).unwrap(),
        message: StartSessionMessage {
            session_name: String::from("Test session"),
            auto_complete_voting: true,
            available_cards: Vec::from([Card::One, Card::Two]),
            password: None,
            tags: HashSet::from([String::from("iOS")]),
        },
    };

    let json = serde_json::to_string(&start_command).ok();

    assert_eq!(json.unwrap(), START_COMMAND_JSON);
}

#[test]
fn test_start_session_deserialize() {
    let parsed_command: Option<HostClientToServerCommand> = serde_json::from_str(START_COMMAND_JSON).ok();

    let start_command = HostClientToServerCommand::StartSession {
        uuid: Uuid::parse_str(&String::from("ca4dfcce-c671-4bbc-b9a8-d8a74a4946c7")).unwrap(),
        message: StartSessionMessage {
            session_name: String::from("Test session"),
            auto_complete_voting: true,
            available_cards: Vec::from([Card::One, Card::Two]),
            password: None,
            tags: HashSet::from([String::from("iOS")]),
        },
    };

    assert_eq!(parsed_command.unwrap(), start_command);
}
