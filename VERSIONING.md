# Politique de versions

- Le contrat `ori-protocol` et les paquets publics suivent le **versionnage sémantique** (`MAJEUR.MINEUR.CORRECTIF`).
- Une modification incompatible du contrat exige une nouvelle version majeure ou une période de compatibilité explicite, avec une stratégie de migration documentée.
- On **déprécie avant de supprimer**, en annonçant la version de retrait. Un champ du protocole n'est jamais retiré sans nouvelle version (dossier technique §6).
- Les manifestes de plugin déclarent la version du protocole (`protocol = 1`) et la fourchette d'hôte et de SDK compatibles.
- Le dépôt privé `ori` ne consomme que des **tags** de ce dépôt. Un changement de contrat suit l'ordre suivant : PR publique, puis tag, puis PR privée qui monte la version.
- Une matrice des versions supportées (hôte et SDK) sera publiée avec la première version distribuée.
