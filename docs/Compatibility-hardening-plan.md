# Compatibility hardening plan

This document tracks small, reviewable repairs that make Obscura's browser surfaces more internally consistent and easier to validate.

## Goals

- Prefer compatibility fixes over broad stealth claims.
- Keep network, JavaScript, CDP, cookie, and profile surfaces aligned.
- Make unsupported APIs explicit instead of silently crashing pages.
- Keep every change independently testable.

## Current repair branches

| Area | Branch / PR | Purpose |
| --- | --- | --- |
| Cookies | `fix/cookie-year` / PR #1 | Normalize two-digit `Expires` years using browser cookie-date rules. |
| HTTP client hints | `fix/profile-header-consistency` / PR #2 | Derive `sec-ch-ua` and `sec-ch-ua-platform` from the selected User-Agent. |
| CDP input | `fix/cdp-input-event-sequence` / PR #3 | Dispatch mouse events in `mousedown -> mouseup -> click` order. |
| Update visibility | `feat/cli-update-check` / PR #4 | Add an explicit read-only release check helper. |
| CDP tests | PR #5 | Cover isolated-world execution context id allocation. |
| Planning docs | PR #6 | Track compatibility-hardening scope, non-goals, and validation. |
| Profile tests | PR #7 | Cover obvious Windows/macOS profile surface mismatches. |
| Validation helper | PR #8 | Add a local script for targeted compatibility repair checks. |
| PR template | PR #9 | Add a compatibility repair PR template for future scoped changes. |
| Follow-up tracker | PR #10 | Track remaining runtime candidates outside the current small PR series. |

## Follow-up candidates

The detailed follow-up checklist lives in PR #10 (`docs/Compatibility-hardening-followups.md`) because GitHub Issues are disabled for this repository.

### JavaScript UA data consistency

`navigator.userAgentData` should be derived from the active User-Agent profile so `brands`, `fullVersionList`, `uaFullVersion`, and `platform` stay aligned with request headers.

### Locale and timezone consistency

Locale, `Accept-Language`, `navigator.language`, `navigator.languages`, timezone, and optional geolocation should be configured through one profile path so deployments do not accidentally advertise contradictory regions.

### WebRTC unsupported boundary

Obscura should expose a stable unsupported boundary for WebRTC constructors if full WebRTC is not implemented. Pages should be able to feature-detect the boundary without `ReferenceError`, while methods should return clear `NotSupportedError` failures.

### Profile regression coverage

Profiles should have tests for obvious mismatches such as Windows User-Agent with macOS platform, or Chrome major version mismatches between UA and UA-CH.

## Validation checklist

Run the helper from PR #8 for the targeted checks:

```bash
bash scripts/validate-compatibility-repairs.sh
```

Or run the commands directly:

```bash
cargo test -p obscura-net cookie
cargo test -p obscura-net client_hints_follow_selected_user_agent
cargo test -p obscura-net ssrf_tests
cargo test -p obscura-cdp input
cargo test -p obscura-cdp isolated_world_context
cargo test -p obscura-browser profile_consistency
cargo check -p obscura-cli --bin obscura-check-update
```

Then run the full suite before merging dependent branches:

```bash
cargo test
# or, after PR #8 is merged:
FULL=1 bash scripts/validate-compatibility-repairs.sh
```

## Review workflow

Use the compatibility repair template from PR #9 for future changes. It keeps each PR explicit about scope, changed surfaces, validation, and non-goals.

## Non-goals

These repairs should not add challenge-solving behavior, CAPTCHA bypass logic, or site-specific risk-control workarounds. Changes should stay framed as browser compatibility, consistency, explicit unsupported API boundaries, or test coverage.
