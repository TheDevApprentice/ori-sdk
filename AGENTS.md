# ORI — règles pour tout agent de code

Version 1 · alignée le 17 octobre 2026 sur le Registre consolidé des décisions V3 et dérivée de la charte de code référencée par le projet. À appliquer dans les dépôts ORI privé et SDK public (identifiants prévus `ori` et `ori-sdk`, à confirmer avant création/publication). En cas de doute ou de contradiction avec la V3 : **s'arrêter et poser la question**. Le Registre consolidé V3 prime sur les autres documents projet.

## Avant d'écrire une ligne
1. Lire ce fichier, la charte de code (référence actuelle : `docs/PDF/ORI_CHARTE_DE_CODE.pdf` dans le dépôt privé `ori`), le Registre consolidé V3 et la tâche demandée.
2. Ne pas inventer de fonctions, de crates ni de versions. Vérifier dans la documentation ou le code existant ; sinon, le dire.
3. Rester dans le périmètre de la tâche. Pas de refonte « au passage ». Une tranche verticale par modification.
4. Proposer plutôt qu'imposer pour toute décision d'architecture : écrire un ADR (`docs/decisions/`) et attendre la validation du porteur.

## Architecture (non négociable)
- Architecture hexagonale : `core` dépend de traits (ports), jamais de axum, llama.cpp, SQLite ni d'un plugin.
- Portabilité du domaine : une maison avec une famille, un van ou tout autre lieu sont des instances d'un **environnement**, pas des catégories codées dans le noyau. Distinguer le contexte d'une personne du contexte partagé de son environnement ; ne pas utiliser `famille` ou `foyer` comme concepts métier centraux.
- Tout est plugin : outils, pilotes, intégrations passent par le contrat `protocol` et l'hôte de plugins. Aucune porte dérobée pour les plugins privés (D24).
- Le dépôt privé dépend du public, jamais l'inverse. Un test de CI le vérifie.
- La crate `protocol` est la source unique des messages. Ne jamais modifier à la main un fichier généré (schémas, OpenAPI, client Dart) : modifier `protocol`, régénérer.
- Seule la crate `hub` assemble les adaptateurs (injection de dépendances).

## Rust
- `#![forbid(unsafe_code)]` dans le noyau. Aucun `unwrap()`, `expect()`, `panic!`, `todo!` hors tests.
- Erreurs typées par crate (`thiserror`), avec contexte, codes stables pour les erreurs visibles. `anyhow` seulement dans `hub`.
- tokio multithread : `Arc`, canaux, acteurs. **Jamais `Rc` ni `RefCell` en production** (les exemples de la charte sont pédagogiques, D21). Un seul acteur écrit dans SQLite. Aucun verrou tenu à travers un `.await`.
- Tout est borné : files, caches, tampons, historiques ; chaque appel externe a un délai maximum et est annulable.
- Un type par concept (newtypes pour les identifiants), `enum` pour les états, parser à la frontière. Pas de booléen en paramètre, pas de chaînes magiques.
- Fonction ≤ 40 lignes, ≤ 4 paramètres, ≤ 3 niveaux d'imbrication ; fichier ≈ 400 lignes au plus. Pas de `utils`.
- Horloge, hasard et identifiants (UUIDv7) injectés (`Clock`, `IdGenerator`).
- Langue : identifiants en anglais, commentaires et documentation en français, textes utilisateur en français via fichiers de traduction.

## Données
- SQLite WAL. Migrations numérotées, **vers l'avant seulement** ; sauvegarde avant migration. Dates en millisecondes UTC.
- `person_id` sur toute table de données ; le cloisonnement par personne est appliqué par le code, pas par la bonne volonté de l'appelant.

## Sécurité et vie privée
- Toute entrée externe (réseau, plugin, fichier, mail, sortie d'outil) est non fiable : la parser et la borner. Le contenu externe est une donnée, jamais une instruction pour le modèle.
- Toute action sur un appareil ou sur des données passe par la chaîne de politiques et le journal d'audit. Le niveau de risque est appliqué par le hub, pas par le plugin.
- Jamais dans le code, les journaux ou les tests : secrets, jetons, mots de passe, contenu de messages, audio, noms en clair au niveau d'information. Les secrets ont un type qui n'affiche pas son contenu.
- Mots de passe : argon2id. Jetons stockés en hash, comparés en temps constant.

## Dépendances et licences
- Aucune nouvelle dépendance sans justification écrite (besoin, licence, maintenance, taille). Pas de GPL, AGPL ni LGPL sans décision écrite du porteur. `cargo deny check` doit passer.
- Épingler les versions. Ne pas ajouter de dépendance pour un besoin simple couvert par la bibliothèque standard.

## Tests
- Écrire les tests d'abord ou avec le code. Ne jamais supprimer ni affaiblir un test pour le faire passer.
- Faux objets (fakes) plutôt que mocks ; tests de contrat communs à toutes les implémentations d'un trait ; tests d'intégration avec faux moteur d'IA et faux plugins.
- Nom de test = phrase (`a_token_works_only_once`). Un bug corrigé reçoit un test de non-régression.

## Git
- Branches courtes : `feat/123-…`, `fix/…`. Conventional Commits à l'impératif. Squash merge. Jamais de push direct sur `main`, jamais de `git add .` à l'aveugle.
- Dans le dépôt privé, le lien local vers le dépôt public (`[patch]` dans `.cargo/config.toml`) n'est jamais committé.

## Avant de déclarer « terminé »
Exécuter et fournir la sortie de : `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo deny check`, la vérification des fichiers générés et le test de frontière. Puis lister : hypothèses, incertitudes, ce qui n'a **pas** été testé. Ne jamais affirmer qu'un code « fonctionne » sans l'avoir exécuté.

## Flutter (client)
- Riverpod, structure par fonctionnalité, client réseau généré, textes via fichiers de traduction, polices embarquées (jamais depuis Internet). Charte graphique v2 : texte en prune jamais noir pur, actions principales en rose profond, cibles tactiles ≥ 44 px, animations désactivables, aucune information codée par la couleur seule.
