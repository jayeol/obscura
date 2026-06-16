#!/usr/bin/env bash
set -euo pipefail

# Print the JavaScript-visible UA surfaces from a local Obscura binary.
# By default this is diagnostic-only. Set STRICT=1 to fail when the Chrome
# major version in navigator.userAgent does not match navigator.userAgentData.

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
printf '<!doctype html><title>ua-data</title>' > "$tmpdir/index.html"
url="file://$tmpdir/index.html"

expr='JSON.stringify({userAgent:navigator.userAgent,userAgentData:navigator.userAgentData?{brands:navigator.userAgentData.brands,mobile:navigator.userAgentData.mobile,platform:navigator.userAgentData.platform}:null})'
json="$($OBSCURA_BIN fetch "$url" --eval "$expr" --quiet)"

echo "$json"

if [[ "${STRICT:-0}" != "1" ]]; then
  exit 0
fi

python3 - "$json" <<'PY'
import json
import re
import sys

data = json.loads(sys.argv[1])
ua = data.get("userAgent", "")
match = re.search(r"Chrome/(\d+)", ua)
if not match:
    raise SystemExit("navigator.userAgent does not contain Chrome/<major>")
ua_major = match.group(1)
ua_data = data.get("userAgentData") or {}
brands = ua_data.get("brands") or []
versions = {str(item.get("version", "")) for item in brands if item.get("brand") != "Not;A=Brand"}
if ua_major not in versions:
    raise SystemExit(
        f"UA major {ua_major} was not present in navigator.userAgentData.brands: {brands}"
    )
print(f"UA data major version matches userAgent: {ua_major}")
PY
