//! Format d'erreur réseau unique (ADR-009) : codes stables `domaine.motif`, jamais de phrase.

use std::collections::BTreeMap;
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ids::RequestId;

/// Domaines autorisés pour un code d'erreur (ADR-009).
pub const ERROR_DOMAINS: [&str; 8] = [
    "auth",
    "conversation",
    "message",
    "engine",
    "plugin",
    "rate",
    "input",
    "internal",
];

const MAX_CODE_LEN: usize = 64;

/// Code d'erreur stable, de la forme `domaine.motif` en minuscules ASCII (`auth.invalid_credentials`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
#[schemars(transparent)]
pub struct ErrorCode(
    #[schemars(
        regex(
            pattern = r"^(auth|conversation|message|engine|plugin|rate|input|internal)\.[a-z0-9_]+$"
        ),
        length(max = 64)
    )]
    String,
);

/// Raison du refus d'un code d'erreur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCodeRejection {
    /// Longueur nulle ou supérieure à la borne.
    Length,
    /// Absence de séparateur unique `domaine.motif`.
    Shape,
    /// Domaine hors de la liste [`ERROR_DOMAINS`].
    UnknownDomain,
    /// Caractère autre que `a-z`, `0-9` ou `_` dans le motif.
    Character,
}

impl fmt::Display for ErrorCodeRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::Length => "longueur invalide",
            Self::Shape => "forme attendue domaine.motif",
            Self::UnknownDomain => "domaine inconnu",
            Self::Character => "caractère interdit",
        };
        f.write_str(reason)
    }
}

impl std::error::Error for ErrorCodeRejection {}

impl ErrorCode {
    /// Parse et valide un code d'erreur.
    pub fn parse(raw: &str) -> Result<Self, ErrorCodeRejection> {
        if raw.is_empty() || raw.len() > MAX_CODE_LEN {
            return Err(ErrorCodeRejection::Length);
        }
        let (domain, reason) = raw.split_once('.').ok_or(ErrorCodeRejection::Shape)?;
        if reason.is_empty() || reason.contains('.') {
            return Err(ErrorCodeRejection::Shape);
        }
        if !ERROR_DOMAINS.contains(&domain) {
            return Err(ErrorCodeRejection::UnknownDomain);
        }
        let allowed = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_';
        if !reason.chars().all(allowed) {
            return Err(ErrorCodeRejection::Character);
        }
        Ok(Self(raw.to_owned()))
    }

    /// Représentation textuelle du code.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for ErrorCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// Indication de nouvelle tentative transmise au client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Retry {
    /// Ne pas réessayer.
    Never,
    /// Réessayer plus tard.
    Later,
    /// Réessayer après une action (reconnexion, rafraîchissement).
    After,
}

/// Corps d'erreur unique sur le réseau : aucune trace, chemin, requête SQL ni nom de crate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ErrorBody {
    /// Code stable traduit par le client.
    pub code: ErrorCode,
    /// Paramètres structurés insérés comme texte par le client.
    #[serde(default)]
    pub params: BTreeMap<String, String>,
    /// Identifiant de requête pour corréler avec les journaux du hub.
    pub request_id: RequestId,
    /// Conduite à tenir.
    pub retry: Retry,
}
