# Documentation d'ori-sdk

Point d'entrée de la documentation du dépôt public. Ce dépôt est une
**bibliothèque** — la documentation produit et l'architecture d'ORI vivent dans
le dépôt privé `ori` (`docs/`, voir `ori/docs/README.md`).

## Le contrat — `ori-protocol`

**[`protocol.md`](protocol.md)** — source unique des messages entre le hub et
ses clients (D40) ; ne dépend d'aucune autre crate (ADR-011).

- **Code** : [`crates/ori-protocol`](../crates/ori-protocol/) — messages
  WebSocket, codes d'erreur stables, niveaux de risque, capacités des outils.
- **Schémas générés** : [`schemas/`](../schemas/) — `client-message`,
  `server-message`, `tool-call`, `tool-descriptor`, `tool-result`,
  `error-body`. *Ne jamais modifier à la main : modifier `ori-protocol`,
  régénérer.*
- **Test de conformité** : `crates/ori-protocol/tests/` — le contrat est
  vérifié par le code, pas par la prose.

## Décisions d'architecture

[`decisions/`](decisions/) — ADR propres à ce dépôt (ADR-000 modèle
interchangeable, ADR-001 génération du contrat). Les ADR sont immuables.
L'autorité amont reste le registre consolidé V3 du dépôt privé
(`ori/docs/DECISIONS.md`).

## Catalogue et skills

- **[`catalogue.md`](catalogue.md)** — [`catalog/`](../catalog/) : catalogue
  signé (minisign/Ed25519) des serveurs MCP Docker et des skills publiques,
  révocations, règle `fail-closed`.
- **[`skills.md`](skills.md)** — [`skills/`](../skills/) : format `SKILL.md`
  (frontmatter, bornes, sécurité), égalité exacte description ↔ catalogue,
  installation côté hub.

## Règles de contribution

[`../AGENTS.md`](../AGENTS.md) (agents de code) ·
[`../CONTRIBUTING.md`](../CONTRIBUTING.md) (DCO) ·
[`../VERSIONING.md`](../VERSIONING.md) (SemVer + releases) ·
[`../SECURITY.md`](../SECURITY.md) · [`../CODE_OF_CONDUCT.md`](../CODE_OF_CONDUCT.md).
