//! Tests de contrat : sérialisation stable et refus des entrées mal formées.

#![allow(clippy::unwrap_used)] // fichier de test uniquement (AGENTS.md : unwrap autorisé dans les tests)

use ori_protocol::error::{ErrorBody, ErrorCode, ErrorCodeRejection, Retry};
use ori_protocol::ids::{ClientMessageId, ConversationId, EventId, MessageId, RequestId};
use ori_protocol::ws::{ClientMessage, ServerMessage, SessionState};
use uuid::Uuid;

const A: &str = "0192f0b2-7d2e-7c1a-9b3e-1f2a3b4c5d6e";

fn uuid() -> Uuid {
    Uuid::parse_str(A).unwrap()
}

#[test]
fn a_message_send_round_trips_with_its_wire_name() {
    let msg = ClientMessage::MessageSend {
        conversation_id: ConversationId::from_uuid(uuid()),
        client_msg_id: ClientMessageId::from_uuid(uuid()),
        text: "Bonjour <script>".to_owned(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains(r#""type":"message.send""#));
    assert_eq!(serde_json::from_str::<ClientMessage>(&json).unwrap(), msg);
}

#[test]
fn an_unknown_message_type_is_rejected() {
    let json = r#"{"type":"admin.backdoor","proto":1}"#;
    assert!(serde_json::from_str::<ClientMessage>(json).is_err());
}

#[test]
fn an_unknown_field_is_rejected() {
    let json = r#"{"type":"hello","proto":1,"person_id":"x"}"#;
    assert!(serde_json::from_str::<ClientMessage>(json).is_err());
}

#[test]
fn a_malformed_identifier_is_rejected() {
    let json = r#"{"type":"resume","conversation_id":"../etc","last_event_id":3}"#;
    assert!(serde_json::from_str::<ClientMessage>(json).is_err());
}

#[test]
fn server_events_keep_their_numbering() {
    let msg = ServerMessage::Token {
        event_id: EventId::new(7),
        message_id: MessageId::from_uuid(uuid()),
        text: "mot".to_owned(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains(r#""event_id":7"#));
    assert_eq!(serde_json::from_str::<ServerMessage>(&json).unwrap(), msg);
}

#[test]
fn session_states_use_snake_case() {
    let msg = ServerMessage::State {
        conversation_id: ConversationId::from_uuid(uuid()),
        state: SessionState::Thinking,
    };
    assert!(
        serde_json::to_string(&msg)
            .unwrap()
            .contains(r#""state":"thinking""#)
    );
}

#[test]
fn a_valid_error_code_is_accepted() {
    assert!(ErrorCode::parse("auth.invalid_credentials").is_ok());
}

#[test]
fn error_codes_outside_the_convention_are_rejected() {
    assert_eq!(ErrorCode::parse("auth"), Err(ErrorCodeRejection::Shape));
    assert_eq!(
        ErrorCode::parse("db.timeout"),
        Err(ErrorCodeRejection::UnknownDomain)
    );
    assert_eq!(
        ErrorCode::parse("auth.Invalid"),
        Err(ErrorCodeRejection::Character)
    );
    assert_eq!(ErrorCode::parse("auth.a.b"), Err(ErrorCodeRejection::Shape));
    assert_eq!(ErrorCode::parse(""), Err(ErrorCodeRejection::Length));
}

#[test]
fn an_error_body_carries_only_code_params_request_and_retry() {
    let body = ErrorBody {
        code: ErrorCode::parse("rate.limited").unwrap(),
        params: Default::default(),
        request_id: RequestId::from_uuid(uuid()),
        retry: Retry::Later,
    };
    let json = serde_json::to_string(&ServerMessage::Error(body.clone())).unwrap();
    let back: ServerMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(back, ServerMessage::Error(body));
    let forged = format!(
        r#"{{"type":"error","code":"rate.limited","params":{{}},"request_id":"{A}","retry":"later","trace":"x"}}"#
    );
    assert!(serde_json::from_str::<ServerMessage>(&forged).is_err());
}

#[test]
fn an_invalid_error_code_cannot_be_deserialized() {
    let json = format!(
        r#"{{"type":"error","code":"sql.syntax","params":{{}},"request_id":"{A}","retry":"never"}}"#
    );
    assert!(serde_json::from_str::<ServerMessage>(&json).is_err());
}
