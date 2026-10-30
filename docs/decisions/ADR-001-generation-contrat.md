# ADR-001 — Génération des artefacts du contrat

**Statut :** Décidé le 30 octobre 2026 par l'agent ingénieur, sur délégation du porteur pour les choix techniques courants (révisable par le porteur).
**Références :** D40, D44, ADR-009 (cadrage), dossier technique §6.

## Contexte et problème

D40 fait d'`ori-protocol` la source unique du contrat : les messages WebSocket, les erreurs et les types d'outils ne doivent pas être recopiés à la main côté client ou plugin. Il faut choisir comment produire les artefacts dérivés (JSON Schema, OpenAPI, client Dart) et comment empêcher leur dérive.

## Objectifs et contraintes

- Les types Rust sont la seule source ; les artefacts sont générés, jamais édités.
- Une dérive entre types et artefacts doit faire échouer la CI.
- Dépendances limitées, licences autorisées (D67), aucun outil réseau dans le test.

## Décision proposée

1. **JSON Schema (draft 2020-12)** généré par `schemars` (MIT) à partir des types d'`ori-protocol`, versionné dans `schemas/`.
2. Le test `crates/ori-protocol/tests/schemas.rs` compare les schémas versionnés aux types et échoue en cas d'écart. Régénération explicite : `ORI_UPDATE_SCHEMAS=1 cargo test -p ori-protocol --test schemas`.
3. **OpenAPI** : produit par `ori-hub` pour ses routes HTTP, à partir des mêmes types, au moment où ces routes existent (tranche A/B). L'outil sera choisi et justifié (D69) dans cette tranche.
4. **Client Dart** : généré à partir des schémas JSON et de l'OpenAPI lorsque le client Flutter consommera l'API (tranche C). L'outil sera choisi et justifié dans cette tranche.

## Options envisagées et raisons de leur rejet

- Schémas écrits à la main : dérive assurée, contraire à D40.
- Génération dans `build.rs` : écrit hors de `OUT_DIR`, comportement implicite à chaque compilation.
- Outil externe (typify, quicktype) comme source : inverserait la source de vérité.

## Conséquences positives et négatives

- \+ Les artefacts publics sont relisibles dans les PR et toute modification du contrat y est visible.
- \+ Les annotations `schemars` portent aussi des bornes (motif et longueur d'`ErrorCode`).
- − Une dépendance de plus dans `ori-protocol` (`schemars`, compilée aussi pour les consommateurs).

## Sécurité et vie privée

Les schémas servent à valider les entrées côté client et plugin ; ils ne contiennent aucune donnée. La validation côté hub reste le parsing strict des types Rust (`deny_unknown_fields`).

## Impact sur les versions et migrations

Toute modification d'un fichier de `schemas/` est un changement de contrat : elle suit `VERSIONING.md` (incompatible ⇒ version majeure du protocole).

## Tests ou preuves nécessaires

Le test de fraîcheur échoue lorsqu'un schéma versionné diverge (preuve dans la PR d'introduction).

## Validation attendue

Le porteur peut revenir sur ce choix ; sinon il s'applique à partir de la tranche T1.
