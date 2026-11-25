# Documentation d'ori-sdk

Point d'entrée de la documentation du dépôt public. Ce dépôt est une
**bibliothèque** — la documentation produit et l'architecture d'ORI vivent dans
le dépôt privé `ori` (`docs/`, voir `ori/docs/README.md`).

## Le contrat — `ori-protocol`

Source unique des messages entre le hub et ses clients (D40) ; ne dépend
d'aucune autre crate (ADR-011).

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
(`ori/docs/reference/ORI_Registre_des_decisions_v3.md`).

## Catalogue et skills

- [`catalog/`](../catalog/) — catalogue signé des serveurs MCP Docker
  (`mcp-servers.json` + `.minisig`, `revocations.json`, `minisign.pub`) et des
  skills publiques (`skills.json`). Format et vérification : ADR-053 dans `ori`.
- [`skills/`](../skills/) — skills publiques (`SKILL.md` : frontmatter
  `name`/`description`/`license` + consignes). La description du catalogue doit
  égaler **exactement** le frontmatter — le test `catalog_entries_…` le vérifie.
  Écrire une skill : [`skills/rediger-skill/SKILL.md`](../skills/rediger-skill/SKILL.md).

## Règles de contribution

[`../AGENTS.md`](../AGENTS.md) (agents de code) ·
[`../CONTRIBUTING.md`](../CONTRIBUTING.md) (DCO) ·
[`../VERSIONING.md`](../VERSIONING.md) (SemVer + releases) ·
[`../SECURITY.md`](../SECURITY.md) · [`../CODE_OF_CONDUCT.md`](../CODE_OF_CONDUCT.md).
