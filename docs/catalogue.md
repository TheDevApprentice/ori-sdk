# Catalogue signé — `catalog/`

Le catalogue liste les artefacts que le hub ORI est autorisé à installer :
serveurs MCP Docker et skills publiques. Il est **signé** : le hub télécharge
les fichiers en brut et le side-car `ori-mcp-gw` en vérifie la signature
**avant** toute résolution — `fail-closed` (signature invalide ou entrée
révoquée ⇒ rejet).

## Fichiers

| Fichier | Rôle |
|---|---|
| `mcp-servers.json` | `server_id` → `image@sha256` épinglé |
| `mcp-servers.json.minisig` | signature Ed25519 (format minisign, préhash BLAKE2b) |
| `revocations.json` + `.minisig` | entrées retirées — une révocation signée prime |
| `minisign.pub` | clé publique de vérification (la clé privée n'est pas ici) |
| `skills.json` | skills publiques : `name`, `description`, `path`, `version`, `license` |

## Règles

- **Épinglage** : jamais de tag flottant — `image@sha256` immuable.
- **Signature** : toute modification d'un fichier listé exige de re-signer
  avec la clé privée du porteur (`minisign -S -m <fichier> -s <clé>`).
- **`skills.json`** : la `description` doit être **strictement égale** au
  frontmatter `description` du `SKILL.md` pointé — le test
  `crates/ori-protocol/tests/catalog.rs` le vérifie à l'octet près.
- La clé privée et le processus de signature relèvent du porteur (décision
  réservée) — voir `ori/docs/DECISIONS.md` (ADR-053, D115/D116).

## Consommation côté hub

`ori-mcp-gw` : télécharge → vérifie `.minisig` → consulte `revocations.json` →
résout `server_id`. Détail du pont et de la confiance : condensé ADR-053 dans
`ori/docs/DECISIONS.md`.
