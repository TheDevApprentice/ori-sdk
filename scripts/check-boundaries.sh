#!/usr/bin/env bash
# Test de frontière (D39, ADR-011) : lit `cargo metadata` et échoue si une flèche de dépendance est violée.
# Usage : check-boundaries.sh <private|public> [fichier-metadata.json]
set -euo pipefail

mode="${1:?usage: check-boundaries.sh <private|public> [metadata.json]}"
if [[ $# -ge 2 ]]; then
  metadata="$(cat "$2")"
else
  metadata="$(cargo metadata --format-version 1 --no-deps)"
fi

violations=0
report() { echo "VIOLATION: $1" >&2; violations=$((violations + 1)); }

# Couples "paquet dépendance" (dépendances normales, de build et de dev) des membres du workspace.
edges="$(jq -r '.packages[] | .name as $p | .dependencies[] | "\($p) \(.name) \(.source // "path")"' <<<"$metadata")"

private_crate='^(ori-core|ori-hub|ori-plugin-host|ori-adapters-.*)$'
adapter_crate='^(ori-plugin-host|ori-adapters-.*)$'
core_forbidden='^(ori-hub|ori-plugin-host|ori-adapters-.*|axum|rusqlite|sqlx|reqwest|hyper|tokio)$'

while read -r pkg dep source; do
  [[ -z "${pkg:-}" ]] && continue
  case "$mode" in
    private)
      if [[ "$pkg" == "ori-core" && "$dep" =~ $core_forbidden ]]; then
        report "ori-core dépend de $dep (le noyau ne dépend que de ports)"
      fi
      if [[ "$pkg" != "ori-hub" && "$dep" =~ $adapter_crate ]]; then
        report "$pkg dépend de $dep (seul ori-hub assemble les adaptateurs)"
      fi
      ;;
    public)
      if [[ "$dep" =~ $private_crate ]]; then
        report "$pkg dépend de la crate privée $dep (le public ne dépend jamais du privé)"
      fi
      if [[ "$source" =~ github\.com/TheDevApprentice/ori(\.git)?([?#]|$) ]]; then
        report "$pkg dépend du dépôt privé ori ($source)"
      fi
      if [[ "$pkg" == "ori-protocol" && "$source" == "path" ]]; then
        report "ori-protocol dépend de $dep (ori-protocol ne dépend d'aucune autre crate ORI)"
      fi
      ;;
    *) echo "mode inconnu : $mode" >&2; exit 2 ;;
  esac
done <<<"$edges"

if [[ $violations -gt 0 ]]; then
  echo "$violations violation(s) de frontière." >&2
  exit 1
fi
echo "Frontières respectées ($mode)."
