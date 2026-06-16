#!/usr/bin/env bash
set -euo pipefail

# Run the targeted checks for the compatibility-hardening PR series.
# This script is intentionally local/manual: it does not change code,
# download binaries, or contact external services.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

run() {
  echo
  echo "+ $*"
  "$@"
}

run cargo test -p obscura-net cookie
run cargo test -p obscura-net client_hints_follow_selected_user_agent
run cargo test -p obscura-net ssrf_tests
run cargo test -p obscura-cdp input
run cargo test -p obscura-cdp isolated_world_context
run cargo test -p obscura-browser profile_consistency
run cargo check -p obscura-cli --bin obscura-check-update

if [[ "${FULL:-0}" == "1" ]]; then
  run cargo test
fi

echo
echo "Compatibility repair validation completed."
