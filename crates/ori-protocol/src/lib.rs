//! Contrat public d'ORI : source unique des messages échangés entre clients, hub et plugins (D40).
//!
//! Cette crate ne dépend d'aucune autre crate ORI (ADR-011). Tout type entrant est parsé à la
//! frontière : un message mal formé, un champ inconnu ou un code d'erreur invalide est refusé.

pub mod error;
pub mod ids;
pub mod tool;
pub mod ws;

/// Version du contrat déclarée par les manifestes de plugin et le message `hello` (ADR-002).
pub const PROTOCOL_VERSION: u16 = 1;
