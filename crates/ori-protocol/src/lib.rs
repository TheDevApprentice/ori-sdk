//! Contrat public d'ORI : source unique des messages échangés entre clients, hub et plugins (D40).
//!
//! Cette crate ne dépend d'aucune autre crate ORI (ADR-011). Son contenu (identifiants, format
//! d'erreur, messages WebSocket, types d'outils) est ajouté dans la tranche T1.

/// Version du contrat déclarée par les manifestes de plugin (`protocol = 1`, ADR-002).
pub const PROTOCOL_VERSION: u16 = 1;

#[cfg(test)]
mod tests {
    use super::PROTOCOL_VERSION;

    #[test]
    fn the_first_contract_version_is_one() {
        assert_eq!(PROTOCOL_VERSION, 1);
    }
}
