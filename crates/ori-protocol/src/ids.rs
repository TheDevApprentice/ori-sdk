//! Identifiants du contrat : un type par concept (AGENTS.md), UUIDv7 sérialisés en texte (D41).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! uuid_id {
    ($(#[$doc:meta] $name:ident),+ $(,)?) => {$(
        #[$doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Construit l'identifiant à partir d'un UUID déjà généré (générateur injecté côté hub).
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            /// Valeur UUID sous-jacente.
            pub const fn as_uuid(&self) -> Uuid {
                self.0
            }
        }
    )+};
}

uuid_id!(
    /// Identifiant d'une conversation.
    ConversationId,
    /// Identifiant d'un message attribué par le hub.
    MessageId,
    /// Identifiant d'idempotence choisi par le client pour `message.send`.
    ClientMessageId,
    /// Identifiant de requête propagé du client au plugin (ADR-010).
    RequestId,
);

/// Numéro d'événement, croissant par conversation, utilisé pour la reprise (`resume`, D44).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct EventId(u64);

impl EventId {
    /// Construit un numéro d'événement.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Valeur numérique.
    pub const fn value(&self) -> u64 {
        self.0
    }
}
