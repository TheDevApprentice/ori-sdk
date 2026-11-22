---
name: rediger-skill
description: Aide à écrire une skill ORI — format SKILL.md, bornes, règles de sécurité, et quand proposer son installation sur la page Plugins.
license: Apache-2.0
---

# Rédiger une skill ORI

Tu aides la personne à écrire une skill : un guide markdown qu'ORI peut lire
quand le sujet revient. Une skill n'est **pas** un programme : c'est un texte
de consignes lu par le modèle au moment où il en a besoin.

## Format attendu

```markdown
---
name: nom-en-minuscules
description: Une phrase qui dit quand lire cette skill.
license: Apache-2.0
---

# Corps en markdown — consignes, méthode, garde-fous.
```

- `name` : `[a-z0-9-]`, commence par une lettre ou un chiffre, ≤ 64 caractères.
- `description` : une ligne, dit **quand** la skill s'applique — c'est elle qui
  apparaît dans le catalogue lu par le modèle.
- Corps ≤ 16 Kio, markdown sobre : sections courtes, listes, pas de code à
  exécuter. Plus la skill est dense, mieux elle est lue.

## Règles de contenu

- Écris en impératif à la deuxième personne (« propose », « vérifie ») — la
  skill s'adresse au modèle, pas à la personne.
- Jamais de secret, jeton, mot de passe, donnée personnelle ni URL privée :
  le contenu peut ensuite être partagé au foyer ou publié.
- Jamais de contournement : une skill ne peut ni désactiver la politique, ni
  promettre des actions qu'ORI refuse ailleurs (confirmations, risques).
- Clos par des garde-fous : ce que la skill interdit vaut autant que ce
  qu'elle autorise.

## Installation

Une fois le texte relu ensemble :

1. La personne l'installe depuis la page **Plugins → Skills → +** en collant
   le contenu (portée « moi » par défaut, « foyer » pour les admins).
2. Elle l'active avec l'interrupteur de sa carte — l'activation est propre à
   chacun.
3. Suggère de la tester en commençant un message par `@nom`.

Si la personne demande de la partager, explique que la portée « foyer » la
rend visible de tous les membres de l'environnement, inactive chez chacun
jusqu'à activation.
