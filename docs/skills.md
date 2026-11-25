# Skills — `skills/`

Une skill est un `SKILL.md` : un guide installé que le modèle lit à la
demande (`skills.read`, mention `@`). Format **agentskills.io** strict
(ADR-052, condensé dans `ori/docs/DECISIONS.md`).

## Format

```markdown
---
name: cuisine-maison          # ASCII [a-z0-9-], ≤ 64, = nom du dossier
description: …                # ≤ 1024, dit quand l'utiliser — = skills.json
license: Apache-2.0
---

# Titre

Corps ≤ 16 Kio : principes, méthode, garde-fous.
```

- `allowed-tools` / `metadata` : tolérés en local, **informatifs** — jamais
  une pré-approbation d'outils ; rejetés au catalogue communautaire (CI).
- Clés frontmatter inconnues : tolérées en local, rejetées au catalogue.
- La `description` du catalogue (`catalog/skills.json`) doit être
  **strictement égale** au frontmatter — `catalog.rs` l'exige à l'octet près.

## Sécurité

Le contenu d'une skill est une **donnée non fiable** comme tout contenu
externe : il guide le modèle, il ne doit pas chercher à l'instrumenter.
Garde-fous attendus dans une skill publique : portée claire, pas
d'exfiltration de données, pas d'appel à contourner la politique d'outils.

## Écrire une skill

La méta-skill [`skills/rediger-skill/SKILL.md`](../skills/rediger-skill/SKILL.md)
guide la rédaction (format, bornes, règles de sécurité, proposition
d'installation). Exemples en production : `cuisine-maison`, `expert-bourse`.

## Installation côté hub

- `<data>/skills/<env>/community/` : skills du catalogue (partagées,
  installation admin)
- `<data>/skills/<env>/person/<id>/` : skills propres à une personne
