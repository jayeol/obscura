#!/usr/bin/env bash
set -euo pipefail

# Print JavaScript-visible locale surfaces from a local Obscura binary.
# Diagnostic-only by default. Set STRICT=1 to require navigator.language to
# match the first navigator.languages entry.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

OBSCURA_BIN="${OBSCURA_BIN:-target/release/obscura}"
if [[ ! -x "$OBSCURA_BIN" ]]; then
  cat >&2 <<EOF
Missing executable: $OBSCURA_BIN

Build it first, or set OBSCURA_BIN to an existing binary, for example:
  cargo build --release -p obscura-cli --bin obscura
  OBSCURA_BIN=target/release/obscura $0
EOF
  exit 2
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
printf '<!doctype html><title>locale</title>' > "$tmpdir/index.html"
url="file://$tmpdir/index.html"

expr='JSON.stringify({language:navigator.language,languages:navigator.languages,intlLocale:Intl.DateTimeFormat().resolvedOptions().locale})'
json="$($OBSCURA_BIN fetch "$url" --eval "$expr" --quiet)"

echo "$json"

if [[ "${STRICT:-0}" != "1" ]]; then
  exit 0
fi

python3 - "$json" <<'PY'
import json
import sys

data = json.loads(sys.argv[1])
language = data.get("language")
languages = data.get("languages") or []
if not language:
    raise SystemExit("navigator.language is empty")
if not languages:
    raise SystemExit("navigator.languages is empty")
if language != languages[0]:
    raise SystemExit(f"navigator.language {language!r} != navigator.languages[0] {languages[0]!r}")
print(f"Locale surfaces are internally aligned: {language}")
PY
