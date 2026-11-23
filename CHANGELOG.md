# Journal des changements

Toutes les évolutions notables d'ori-sdk sont consignées ici. Ce fichier est
généré par [release-please](https://github.com/googleapis/release-please) à
partir des commits conventionnels fusionnés dans `main` — ne pas éditer à la
main : pour corriger une entrée, corriger le titre de la PR fusionnée.

La section `0.5.0` est la base initiale : elle résume l'historique du dépôt
jusqu'à la mise en place du versionnage automatique et a été rédigée à la
main. Les sections suivantes sont générées par release-please.

## [0.5.0](https://github.com/TheDevApprentice/ori-sdk/releases/tag/v0.5.0) (2026-11-24)

### Nouveautés

* **protocol** : contrat v0 — identifiants, erreurs à codes stables, messages
  WebSocket, descripteurs d'outils ; schémas JSON générés et versionnés dans
  le dépôt
* **protocol** : aperçu `detail` sur les événements d'outils ; déclaration du
  puits d'un outil sur son descripteur (ADR-042)
* **skills** : skills sur `message.send` et catalogue de skills
* **mcp** : catalogue de marketplace `mcp-servers.json` signé + liste de
  révocations `revocations.json` pour le hub ORI (ADR-053 T5)
* **repo** : dépôt public — README de statut et de lancement des piles ori,
  secret scanning et push protection, `main` protégé avec CI exigée
* **versionnage** : workspace Cargo sous version unique, SemVer automatisé
  par release-please (PR de release, tag et GitHub Release à la fusion)

### Corrections

* **mcp** : signatures du catalogue calculées sur le contenu en fins de ligne
  LF (`.gitattributes`) — les vérifications ne dépendent plus du checkout
