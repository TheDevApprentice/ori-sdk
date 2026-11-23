# ori-sdk

Dépôt **public** de l'écosystème **ORI**, assistant IA auto-hébergé et local d'abord.

> **Statut :** dépôt public depuis la mise en place du catalogue MCP signé. `main` est
> protégé : PR obligatoire, checks requis (`fmt`, `clippy`, `test`, `deny`, `boundaries`,
> `secrets`), historique linéaire. Le contenu est ajouté tranche par tranche, après
> validation du porteur.

## Rôle

Ce dépôt porte la **frontière publique** d'ORI (D25) :

- `crates/ori-protocol` : messages, codes d'erreur, schémas, niveaux de risque et
  capacités. Source unique du contrat (D40) — ne dépend d'aucune autre crate (ADR-011).
- `catalog/` : catalogue signé des serveurs MCP Docker (`mcp-servers.json` +
  `mcp-servers.json.minisig`, `revocations.json` signé, `minisign.pub`). Le hub télécharge
  ces fichiers en brut et le side-car `ori-mcp-gw` en vérifie les signatures Ed25519
  (format minisign, préhash BLAKE2b) avant de résoudre `server_id` → `image@sha256` —
  voir [`../ori` ADR-053](../). Une entrée révoquée ou une signature invalide est
  rejetée (fail-closed).
- `skills/` : skills publiques consommées par le hub.
- Plus tard : SDK de plugin, kit de tests de conformité, plugins officiels.

Le dépôt privé `ori` dépend de ce dépôt **uniquement via des versions taguées**,
jamais l'inverse. Un test de CI vérifie cette frontière.

## Lancer ORI

ORI s'exécute depuis le dépôt **privé `ori`** — ce dépôt-ci est une bibliothèque, pas
un produit lançable. Collaborateurs : le manuel complet est dans
`ori/DEVELOPPEMENT.md` ; en résumé :

```powershell
.\scripts\ori-up.ps1        # Windows
./scripts/ori-up.sh         # Linux / macOS
```

Le script propose la pile selon le matériel : **prod CPU** (Intel ou AMD — même image),
**prod GPU** (`compose.gpu-nvidia.yaml` CUDA, `compose.gpu-amd.yaml` ROCm — Linux natif,
`compose.gpu-intel.yaml` SYCL — Linux natif) ou **dev type prod + GPU** (interface
reconstruite à chaud sur http://127.0.0.1:8081).

## Licence

Apache License 2.0 (D19, D49) : voir [`LICENSE`](LICENSE).

## Contribuer

Voir [`CONTRIBUTING.md`](CONTRIBUTING.md) (DCO obligatoire), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), [`SECURITY.md`](SECURITY.md) et [`VERSIONING.md`](VERSIONING.md). Les règles pour tout agent de code sont dans [`AGENTS.md`](AGENTS.md).
