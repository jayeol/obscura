# Compatibility hardening follow-up tracker

This document tracks the remaining compatibility-hardening work after the initial small PR series.

The repository currently has GitHub Issues disabled, so this document acts as the follow-up tracker.

## Scope

This tracker is for browser compatibility, internal consistency, explicit unsupported API boundaries, tests, and documentation.

It is not for challenge-solving behavior, CAPTCHA bypass logic, or site-specific risk-control workarounds.

## Current PR series

- [ ] PR #1 — Cookie two-digit `Expires` year normalization
- [ ] PR #2 — HTTP client hints derived from selected User-Agent
- [ ] PR #3 — CDP mouse click event sequence alignment
- [ ] PR #4 — Explicit update-check helper
- [ ] PR #5 — CDP isolated-world context id tests
- [ ] PR #6 — Compatibility hardening plan
- [ ] PR #7 — Browser profile consistency tests
- [ ] PR #8 — Compatibility repair validation script
- [ ] PR #9 — Compatibility repair PR template

## Remaining follow-up candidates

### JavaScript `navigator.userAgentData` consistency

Derive the following from the active User-Agent/profile instead of leaving stale hard-coded values:

- `brands`
- `fullVersionList`
- `uaFullVersion`
- `platform`

Suggested shape:

- Keep this as a small PR focused only on `userAgentData` consistency.
- Add tests or a small validation snippet for Chrome major version alignment.
- Avoid introducing a broad fingerprint rewrite in the same PR.

### Locale / timezone / geolocation consistency

Keep these surfaces aligned through one profile/config path:

- HTTP `Accept-Language`
- `navigator.language`
- `navigator.languages`
- process timezone / V8 timezone
- optional geolocation override

Suggested shape:

- Start with explicit config plumbing and tests.
- Keep defaults stable.
- Avoid geo-IP lookup or external service calls in the core engine.

### Explicit WebRTC unsupported API boundary

If full WebRTC is not implemented, expose a stable unsupported boundary so pages can feature-detect without `ReferenceError`.

Suggested shape:

- Add `RTCSessionDescription`, `RTCIceCandidate`, and `RTCPeerConnection` constructors only if they can return clear unsupported behavior.
- Methods that require real WebRTC should reject with `NotSupportedError`.
- Do not pretend to implement ICE, media capture, or peer networking.

### Profile-driven fingerprint surface refactor

Move JS-side fingerprint surfaces toward a single profile-driven source of truth.

Candidate surfaces:

- screen dimensions
- hardware fields
- GPU strings
- canvas/audio seed values
- battery fields
- UA / UA-CH / platform

Suggested shape:

- Split into multiple small PRs by surface.
- Keep each PR testable.
- Avoid large all-at-once edits to `bootstrap.js`.

### CI workflow for repair validation

Once PR #8 lands, consider wiring the helper into CI.

Suggested shape:

- Start with targeted package checks.
- Keep full workspace tests as a separate job if they are slower.
- Avoid network-dependent tests.

### Temporary branch cleanup

Short-lived branches such as `a1`, `a2`, and `z1` were reset to `main`.

Delete them from the remote when convenient. They no longer carry test changes.

## Suggested validation baseline

After PR #8 lands:

```bash
bash scripts/validate-compatibility-repairs.sh
FULL=1 bash scripts/validate-compatibility-repairs.sh
```

Before PR #8 lands, run targeted checks manually:

```bash
cargo test -p obscura-net cookie
cargo test -p obscura-net client_hints_follow_selected_user_agent
cargo test -p obscura-net ssrf_tests
cargo test -p obscura-cdp input
cargo test -p obscura-cdp isolated_world_context
cargo test -p obscura-browser profile_consistency
cargo check -p obscura-cli --bin obscura-check-update
cargo test
```
