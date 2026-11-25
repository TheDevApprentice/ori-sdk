# ori-sdk

**Frontière publique** d'**ORI**, assistant IA auto-hébergé et local d'abord :
contrat de messages, catalogue signé et skills publiques.

> Dépôt public — Apache License 2.0 · `main` protégé (PR + checks requis)

## Contenu

| Dossier | Rôle |
|---|---|
| `crates/ori-protocol` | **Source unique du contrat** : messages WebSocket, appels d'outils, niveaux de risque, codes d'erreur stables. Ne dépend d'aucune autre crate |
| `schemas/` | JSON Schema **générés** depuis `ori-protocol` — jamais édités à la main |
| `catalog/` | catalogue **signé** (minisign/Ed25519) : serveurs MCP épinglés par `image@sha256`, skills publiques, révocations — `fail-closed` |
| `skills/` | skills publiques au format `SKILL.md` (agentskills.io) |

Détail technique : [`docs/protocol.md`](docs/protocol.md) ·
[`docs/catalogue.md`](docs/catalogue.md) · [`docs/skills.md`](docs/skills.md).

## Relation avec `ori`

ORI (dépôt privé, le produit) consomme ce dépôt **uniquement via des versions
taguées** — jamais l'inverse. Un check CI (`boundaries`) vérifie la frontière.
Ce dépôt est une **bibliothèque** : il ne se lance pas.

Modifier le contrat : types d'abord dans `ori-protocol`, puis régénérer
`schemas/` (`ORI_UPDATE_SCHEMAS=1 cargo test -p ori-protocol --test schemas`),
même commit — détail : [`docs/protocol.md`](docs/protocol.md).

## Vérifier

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo deny check
```

## Documentation et contribution

Index : [`docs/README.md`](docs/README.md) · [`CONTRIBUTING.md`](CONTRIBUTING.md)
(DCO) · [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) · [`SECURITY.md`](SECURITY.md)
· [`VERSIONING.md`](VERSIONING.md) · [`AGENTS.md`](AGENTS.md).
