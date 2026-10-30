# ori-sdk

Dépôt public (à terme) de l'écosystème **ORI**, assistant IA auto-hébergé et local d'abord.

> **Statut :** dépôt **privé** jusqu'à la vérification de la marque et du domaine ORI (D18). Aucune version n'est publiée. Aucun code n'est encore présent : le contenu est ajouté tranche par tranche, après validation du porteur.

## Rôle

Ce dépôt porte la **frontière publique** d'ORI (D25) :

- `crates/ori-protocol` : messages, codes d'erreur, schémas, niveaux de risque et capacités. C'est la source unique du contrat (D40). Cette crate ne dépend d'aucune autre (ADR-011). C'est le seul contenu prévu pour la V0.1.
- Plus tard : SDK de plugin, kit de tests de conformité, plugin d'exemple « heure » et plugins officiels.

Le dépôt privé `ori` dépend de ce dépôt **uniquement via des versions taguées**, jamais l'inverse. Un test de CI vérifie cette frontière.

## Licence

Apache License 2.0 (D19, D49) : voir [`LICENSE`](LICENSE).

## Contribuer

Voir [`CONTRIBUTING.md`](CONTRIBUTING.md) (DCO obligatoire), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), [`SECURITY.md`](SECURITY.md) et [`VERSIONING.md`](VERSIONING.md). Les règles pour tout agent de code sont dans [`AGENTS.md`](AGENTS.md).
