# Le contrat — `ori-protocol`

`crates/ori-protocol` est la **source unique** des messages entre le hub ORI
et ses clients, outils et plugins (D40). Elle ne dépend d'aucune autre crate
(ADR-011) : n'importe qui peut la consommer sans tirer le reste du projet.

## Contenu

- Messages WebSocket client ↔ serveur (`client-message`, `server-message`)
- Appels d'outils : `tool-call`, `tool-descriptor`, `tool-result`
- Corps d'erreur (`error-body`) : **codes stables**, utilisables par un client
- Niveaux de risque (`ToolRisk` : R0→R3) et capacités des outils

## Schémas générés — ne jamais éditer à la main

[`schemas/`](../schemas/) est **généré** à partir des types Rust (`schemars`,
JSON Schema draft 2020-12). Un test de fraîcheur fait échouer la CI si un
schéma versionné diverge des types.

Modifier le contrat :

1. éditer les types dans `crates/ori-protocol` ;
2. régénérer : `ORI_UPDATE_SCHEMAS=1 cargo test -p ori-protocol --test schemas` ;
3. commiter types **et** schémas ensemble — même commit.

Toute modification d'un fichier de `schemas/` est un changement de contrat :
elle suit [`VERSIONING.md`](../VERSIONING.md) (incompatible ⇒ majeure).

## Validation

- Parsing strict à la frontière : `deny_unknown_fields` sur les types publics.
- Les schémas servent aux clients/plugins pour valider leurs entrées — la
  validation décisive reste le parsing Rust côté hub.
- Le contrat est vérifié par le code (`crates/ori-protocol/tests/`), jamais
  par la prose.

## Frontière du dépôt

Le dépôt privé `ori` dépend d'`ori-sdk` **uniquement via des versions
taguées** ; `ori-sdk` ne dépend de rien de privé. Un test de CI vérifie cette
frontière dans les deux sens.
