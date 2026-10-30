#!/usr/bin/env bash
# Prouve que le test de frontière échoue lorsqu'une flèche est violée (fixtures cargo metadata minimales).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
script="$here/../check-boundaries.sh"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
failures=0

fixture() { # nom paquet dépendance source
  local src="null"; [[ "$4" != "path" ]] && src="\"$4\""
  printf '{"packages":[{"name":"%s","dependencies":[{"name":"%s","source":%s}]}]}' "$2" "$3" "$src" > "$tmp/$1.json"
}
expect() { # attendu(pass|fail) mode fixture
  if bash "$script" "$2" "$tmp/$3.json" >/dev/null 2>&1; then got=pass; else got=fail; fi
  if [[ "$got" == "$1" ]]; then echo "ok   $3 ($2) -> $got"; else echo "FAIL $3 ($2) : attendu $1, obtenu $got"; failures=$((failures + 1)); fi
}

fixture core_on_adapter   ori-core ori-adapters-store path
fixture core_on_axum      ori-core axum "registry+https://github.com/rust-lang/crates.io-index"
fixture api_on_store      ori-adapters-api ori-adapters-store path
fixture hub_on_store      ori-hub ori-adapters-store path
fixture core_on_thiserror ori-core thiserror "registry+https://github.com/rust-lang/crates.io-index"
fixture public_on_core    ori-protocol ori-core path
fixture public_on_git     ori-plugin-sdk something "git+https://github.com/TheDevApprentice/ori?tag=v0.1.0#abc"
fixture protocol_on_path  ori-protocol ori-plugin-sdk path
fixture sdk_on_protocol   ori-plugin-sdk ori-protocol path
fixture public_on_sdk_git ori-x ori-protocol "git+https://github.com/TheDevApprentice/ori-sdk?tag=v0.1.0#abc"

expect fail private core_on_adapter
expect fail private core_on_axum
expect fail private api_on_store
expect pass private hub_on_store
expect pass private core_on_thiserror
expect fail public  public_on_core
expect fail public  public_on_git
expect fail public  protocol_on_path
expect pass public  sdk_on_protocol
expect pass public  public_on_sdk_git

[[ $failures -eq 0 ]] || { echo "$failures cas en échec." >&2; exit 1; }
echo "Tous les cas du test de frontière sont conformes."
