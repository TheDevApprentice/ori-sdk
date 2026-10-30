# Contribuer à ori-sdk

Ce document applique la **charte d'ingénierie et de collaboration v1.1** (gouvernance D46). En cas de conflit, le registre des décisions ORI et `AGENTS.md` priment.

## Gouvernance

- Le **porteur du projet** (@TheDevApprentice) est le seul décisionnaire des fusions dans `main`.
- La communauté contribue par **fork et pull request**. La branche `main` est protégée dès que la plateforme le permet : pas de push direct, pas de force-push, CI obligatoire, conversations résolues.
- **Exception documentée à la charte (§4.3)** : aucune seconde personne n'est disponible pour la revue. Compensation : CI stricte et relecture du porteur. **Une revue faite par une IA ne compte pas comme revue indépendante.**

## Certificat d'origine du développeur (DCO)

Chaque commit doit porter une ligne `Signed-off-by` qui certifie le [Developer Certificate of Origin 1.1](https://developercertificate.org/) :

```bash
git commit -s -m "feat(protocol): ajoute le type ConversationId"
```

La certification DCO est un engagement **humain**. Pour un commit préparé par un agent IA, la ligne `Signed-off-by` qui fait foi est celle du porteur, ajoutée au moment de la fusion squash. Le recours à un CLA reste à examiner avec un juriste (Q11).

## Workflow Git

- *Trunk-based* léger. Branches courtes liées à une issue : `feat/123-…`, `fix/…`, `docs/…`, `chore/…`.
- **Conventional Commits** à l'impératif (`feat`, `fix`, `docs`, `refactor`, `test`, `build`, `ci`, `chore`…). Pour une rupture : `!` et `BREAKING CHANGE:`.
- **Squash merge** uniquement ; la branche est supprimée après fusion.
- Jamais de push direct ni de force-push sur `main`. Jamais de `git add .` à l'aveugle : inspecter `git status` et `git diff --staged`.
- Une issue, une branche, une PR, de taille relisible en une session.

## Avant d'ouvrir une PR

- Lire `AGENTS.md` et respecter ses règles (architecture, Rust, sécurité, dépendances, tests).
- Fournir les sorties brutes de `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo deny check`, de la vérification des fichiers générés et du test de frontière, dès que ces outils existent dans le dépôt.
- Aucune nouvelle dépendance sans justification écrite (utilité, licence, maintenance, sécurité, taille — D69). Pas de GPL, AGPL ni LGPL (D68).
- Aucun secret, jeton, mot de passe, donnée personnelle ni fichier `.env` réel.
- Toute décision d'architecture passe par une ADR dans `docs/decisions/` et attend la validation du porteur.

## Definition of Done

Critères d'acceptation satisfaits ; code relu et intégré par PR ; CI verte ; tests de régression pertinents ; erreurs et journaux traités ; documentation, ADR et migrations à jour ; licences acceptables ; aucun secret ajouté.
