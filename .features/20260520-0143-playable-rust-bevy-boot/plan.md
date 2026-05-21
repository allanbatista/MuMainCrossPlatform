# Playable Rust Bevy Boot

Status: READY_FOR_EXEC

## Summary

Implement the first executable playable-port slice: remove Rust cross-platform GitHub Actions, replace non-headless `exit` with a Bevy runtime boot, preserve headless/control smoke, and document local validation.

## Interfaces / Contracts

- `AppState::Boot` becomes the non-headless boot state instead of `Exit`.
- New `mu_app::graphical_runtime` builds and runs a Bevy app.
- `run(Cli)` keeps `--headless` and `--control-http` behavior, but runs Bevy when state is `Boot` without `--control-http`.
- Removed files: `.github/workflows/rust-client.yml`, `.github/workflows/rust-client-windows.yml`.
- Local validation commands are the Rust build/test contract.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters/URL state | Dataset/permission gating | Renderer/test target | Retailer/industry compatibility |
|---|---|---|---|---|---|---|---|
| `graphical-boot` | `mu_app`, Bevy | window | none | CLI/env asset root | local assets/config only | `mu_app` unit test + client build | no split |
| `headless-boot` | `mu_app` state shell | stdout | none | CLI/env asset root | local assets/config only | headless smoke | no split |
| `control-http` | `mu_app::control_http` | local HTTP | `GET /state`, `POST /command` | command name | local loopback only | control smoke | no split |
| `local-validation` | docs/scripts | terminal logs | none | command selection | local repo access | grep/docs/build tests | no split |

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Workflow docs | local | `.features/20260520-0143-playable-rust-bevy-boot/*` | none | docs exist and audit passes | feature-workflow audit |
| F0.S2.T1 Remember rules | local | `.memory/RULES_AND_DEFINITION.md` | none | new user rules are appended | file diff |
| F1.S1.T1 Remove Rust Actions | local | `.github/workflows/rust-client.yml`, `.github/workflows/rust-client-windows.yml` | F0 | Rust cross-platform Actions gone | `rg rust-client .github/workflows` |
| F1.S2.T1 Document local validation | local | `docs/build-guide.md`, `port_rust/README.md` | F1.S1 | commands documented | docs diff |
| F2.S1.T1 Graphical runtime module | local | `mu_app/src/graphical_runtime.rs`, `mu_app/src/lib.rs`, `mu_app/Cargo.toml`, `port_rust/Cargo.toml` | F0 | Bevy app can be built in tests | unit test |
| F2.S2.T1 Runtime state wiring | local | `mu_app/src/runtime.rs`, `mu_app/src/state.rs` | F2.S1 | non-headless state is `Boot`; headless unchanged | unit tests + smoke |
| F3.S1.T1 Validation | local | progress/docs | F1-F2 | fmt, tests, build, smoke pass or blockers recorded | command logs |
| F3.S2.T1 Review and pending work | local | `.memory/TODO.md`, progress | F3.S1 | remaining full-port work tracked | TODO entry |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0143-playable-rust-bevy-boot`
- Gate F1: `rtk rg -n "Rust Client|rust-client" .github/workflows` returns no Rust workflow definitions.
- Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F3: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; headless smoke; e2e-validator handoff records this slice as graphical-boot limited.

## AC Traceability

| AC | Tasks | Evidence |
|---|---|---|
| AC-01 | F2.S1.T1, F2.S2.T1 | Validation Gate F2/F3 evidence: `mu_app` graphical app construction test and `mu_client` build. |
| AC-02 | F2.S2.T1, F3.S1.T1 | Validation Gate F3 evidence: headless smoke prints `ready-for-login`. |
| AC-03 | F2.S2.T1, F3.S1.T1 | Validation Gate F3 evidence: invalid asset smoke prints `asset-check-failed` and exits `3`. |
| AC-04 | F1.S1.T1 | Validation Gate F1 evidence: Rust workflow grep returns no workflow definitions. |
| AC-05 | F1.S2.T1 | Validation Gate F3 evidence: docs contain local validation commands. |
| AC-06 | F2.S1.T1, F2.S2.T1 | Validation Gate F2 evidence: `mu_app` tests cover state and app construction. |

## Assumptions

- This slice opens the runtime shell only; gameplay remains pending.
- Existing C++/MinGW workflows remain untouched.
- No Rust GitHub Actions are introduced.
