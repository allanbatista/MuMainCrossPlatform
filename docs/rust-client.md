# Rust Client Difference Register

This document has two jobs:

1. record the approved or intentionally shipped differences between the legacy
   client and the Rust client; and
2. keep comparison evidence in a stable directory layout so CI, local smoke
   tests, and legacy-vs-Rust diffs are easy to find.

If a new user-visible difference appears, add it here with the reason and the
approval or validation evidence before release.

## Difference Register

| Area | Rust client behavior | Why it differs | Evidence / notes |
|---|---|---|---|
| Runtime | The client boots through `mu_client` on Bevy instead of the legacy C++/C# runtime. | The product goal is a Rust + Bevy client, not a wrapper around the old runtime. | `port_rust/README.md`, `port_rust/crates/mu_client/src/main.rs`, workspace tests. |
| Asset handling | The client requires converted assets and a valid manifest before entering gameplay. | Assets are converted ahead of runtime so the release client never runs the pipeline on the fly. | `docs/build-guide.md`, `port_rust/README.md`, `mu_assets` validation tests. |
| Diagnostics | Structured `tracing` logs are redacted and use safe user-facing messages. | Release logs must not expose secrets or raw session data. | `port_rust/crates/mu_app/src/logging.rs`, `port_rust/crates/mu_network/src/redaction.rs`, tests. |
| Configuration | Client settings persist in `config/client.toml`; passwords, tokens, and raw session IDs are never stored. | The Rust port uses a safer local config contract than the legacy client. | `port_rust/README.md`, `docs/player-rust-client.md`, config roundtrip tests. |
| Control surface | A local control HTTP server exists for smoke tests and QA automation. | The port needs a deterministic state probe during development and CI. | `docs/control-http.md`, `mu_app` control HTTP tests. |

No gameplay, economy, or permission difference is currently approved for
release unless it is explicitly listed above with evidence.

## Layout

Each scenario gets its own case root. The case name is slugified once and then
reused for every artifact type.

```text
<artifacts-root>/<case-slug>/
  logs/
  screenshots/
  reports/
  diffs/
```

Example:

```text
/tmp/evidence/login-main-window/
  logs/boot-finished.log
  screenshots/loading-screen.png
  reports/state-snapshot.json
  diffs/legacy-vs-rust.diff
```

## Naming Rules

- Case names are slugified to lower-case hyphenated paths.
- Artifact labels are also slugified before file creation.
- Logs use `.log`.
- Screenshots use `.png`.
- Diffs use `.diff`.
- Reports keep the extension supplied by the report writer.

## Usage

- Use one case root per comparison scenario.
- Keep the case name, artifact label, and artifact type stable between runs.
- Reuse the same root for legacy and Rust outputs when you want a direct diff.
- Prefer short, descriptive labels such as `Boot Finished` or `State Snapshot`.

## Legacy Source Classification

The root-level migration inventory lives in `port_rust/docs/inventory.md`. It
tracks every repository root that must be ported, kept as reference/validation
support, rejected from the Rust runtime, or marked outside the runtime before a
phase is considered complete.

The canonical audit lives in `port_rust/crates/mu_test_support/src/source_inventory.rs`
and is exercised by `port_rust/tests/rust/source_inventory.rs`.

- `reference-only`: keep the legacy path as behavior or protocol reference.
- `fixture-only`: use the path as test or asset fixture input only.
- `replaced`: the Rust client owns the equivalent behavior.
- `ported`: the legacy path is fully mirrored by Rust code.
- `rejected`: keep the path out of the Rust client and packaging flow.

When a new top-level legacy path appears under `ClientLibrary`, `src/source`,
`src/MuEditor`, `src/bin`, `src/ThirdParty`, `src/dependencies` or `tests`,
update the inventory helper and its test in the same change.
