//! Types d'outils partagés entre le cœur, le plugin-host et les plugins (ADR-002, D100).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Nom d'outil, par exemple `time.now`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct ToolName(String);

impl ToolName {
    /// Construit un nom d'outil (la validation de forme est appliquée par le plugin-host).
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Représentation textuelle.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Nature déclarée de l'outil ; un minimum que le hub peut durcir, jamais assouplir (dossier technique §5.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Risk {
    /// Lecture sans effet.
    Read,
    /// Écriture réversible.
    WriteReversible,
    /// Écriture destructive.
    WriteDestructive,
    /// Action physique sur un appareil.
    Physical,
}

/// Fiabilité d'un résultat : un résultat non fiable est une donnée, jamais une instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Trust {
    /// Résultat produit par un composant de confiance.
    Trusted,
    /// Résultat issu d'une source externe (mail, page, serveur MCP…).
    Untrusted,
}

/// Effet de bord d'un outil : puits sortant ou action non annulable. Le hub retient
/// le plus strict entre le manifeste et le descripteur ; déclaré à partir du protocole 1
/// étendu (ADR-042 côté dépôt privé) — absent des anciens descripteurs : `none`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SinkKind {
    /// Pas un puits : lecture ou écriture locale réversible.
    #[default]
    None,
    /// Fait sortir une donnée de l'environnement (`mail.send`, `calendar.create`…).
    Outbound,
    /// Effet non annulable (futurs actuateurs).
    Irreversible,
}

/// Description d'un outil exposée par `describe`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToolDescriptor {
    /// Nom unique.
    pub name: ToolName,
    /// Description destinée au modèle.
    pub description: String,
    /// Risque minimal déclaré.
    pub risk: Risk,
    /// Schéma JSON des arguments.
    pub input_schema: serde_json::Value,
    /// Fiabilité des résultats.
    pub output_trust: Trust,
    /// Puits déclaré : coupé par le hub tant qu'un contenu non fiable est dans le tour.
    #[serde(default)]
    pub sink: SinkKind,
}

/// Appel d'outil proposé par le modèle puis autorisé par le cœur (`call`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToolCall {
    /// Outil appelé.
    pub name: ToolName,
    /// Arguments conformes au schéma de l'outil.
    pub arguments: serde_json::Value,
}

/// Résultat d'un appel d'outil.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToolResult {
    /// Contenu du résultat.
    pub content: serde_json::Value,
    /// Fiabilité imposée par le hub.
    pub trust: Trust,
}
