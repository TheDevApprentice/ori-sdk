//! Messages WebSocket de la V0 (D44, dossier technique §6.2), étiquetés par le champ `type`.

use serde::{Deserialize, Serialize};

use crate::error::ErrorBody;
use crate::ids::{ClientMessageId, ConversationId, EventId, MessageId};
use crate::tool::ToolName;

/// Message envoyé par un client au hub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum ClientMessage {
    /// Ouverture : version du protocole demandée (le jeton voyage hors du message, ADR-013).
    #[serde(rename = "hello")]
    Hello { proto: u16 },
    /// Envoi d'un message texte.
    #[serde(rename = "message.send")]
    MessageSend {
        conversation_id: ConversationId,
        client_msg_id: ClientMessageId,
        text: String,
    },
    /// Annulation d'une génération en cours.
    #[serde(rename = "cancel")]
    Cancel {
        conversation_id: ConversationId,
        message_id: MessageId,
    },
    /// Reprise après coupure depuis le dernier événement reçu.
    #[serde(rename = "resume")]
    Resume {
        conversation_id: ConversationId,
        last_event_id: EventId,
    },
}

/// État de session envoyé au client ; il pilote la mascotte (dossier technique §7.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Idle,
    Queued,
    Thinking,
    Tool,
    Speaking,
}

/// Statut d'un appel d'outil signalé au client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolStatus {
    Started,
    Succeeded,
    Failed,
}

/// Message envoyé par le hub à un client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum ServerMessage {
    /// Version acceptée.
    #[serde(rename = "ready")]
    Ready { proto: u16 },
    /// Message reçu et identifié par le hub.
    #[serde(rename = "message.accepted")]
    MessageAccepted {
        client_msg_id: ClientMessageId,
        message_id: MessageId,
    },
    /// Changement d'état de session.
    #[serde(rename = "state")]
    State {
        conversation_id: ConversationId,
        state: SessionState,
    },
    /// Morceau de texte généré.
    #[serde(rename = "token")]
    Token {
        event_id: EventId,
        message_id: MessageId,
        text: String,
    },
    /// Appel d'outil en cours ou terminé.
    #[serde(rename = "tool")]
    Tool {
        event_id: EventId,
        tool: ToolName,
        status: ToolStatus,
    },
    /// Message complet.
    #[serde(rename = "message.final")]
    MessageFinal {
        event_id: EventId,
        message_id: MessageId,
        text: String,
    },
    /// Erreur au format unique.
    #[serde(rename = "error")]
    Error(ErrorBody),
}
