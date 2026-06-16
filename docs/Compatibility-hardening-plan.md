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

## Follow-up candidates

### JavaScript UA data consistency

`navigator.userAgentData` should be derived from the active User-Agent profile so `brands`, `fullVersionList`, `uaFullVersion`, and `platform` stay aligned with request headers.

### Locale and timezone consistency

Locale, `Accept-Language`, `navigator.language`, `navigator.languages`, timezone, and optional geolocation should be configured through one profile path so deployments do not accidentally advertise contradictory regions.

### WebRTC unsupported boundary

Obscura should expose a stable unsupported boundary for WebRTC constructors if full WebRTC is not implemented. Pages should be able to feature-detect the boundary without `ReferenceError`, while methods should return clear `NotSupportedError` failures.

### Profile regression coverage

Profiles should have tests for obvious mismatches such as Windows User-Agent with macOS platform, or Chrome major version mismatches between UA and UA-CH.

## Validation checklist

Run targeted checks first:

```bash
cargo test -p obscura-net cookie
cargo test -p obscura-net client_hints_follow_selected_user_agent
cargo test -p obscura-cdp input
cargo test -p obscura-cdp isolated_world_context_ids_are_monotonic_and_registered
cargo check -p obscura-cli
```

Then run the full suite before merging dependent branches:

```bash
cargo test
```

## Non-goals

These repairs should not add challenge-solving behavior, CAPTCHA bypass logic, or site-specific risk-control workarounds. Changes should stay framed as browser compatibility, consistency, explicit unsupported API boundaries, or test coverage.
