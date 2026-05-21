# Playable Rust Bevy Boot

Status: READY_FOR_PLAN

## Goal

Make the Rust client start a real Bevy graphical runtime instead of exiting in non-headless mode, while preserving headless smoke behavior and removing Rust cross-platform build automation from GitHub Actions.

This is the first executable slice toward full C++ client parity. It is not the complete playable game.

## Users And Journeys

- Player/developer: runs `mu_client` without `--headless`; a game window opens and remains alive instead of printing `exit`.
- QA/developer: runs `mu_client --headless`; existing headless state output remains deterministic.
- QA/developer: runs `mu_client --control-http 127.0.0.1:0`; state inspection still works for automated smoke.
- Maintainer: validates Rust locally with documented commands; Rust cross-platform build is not performed by GitHub Actions.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Graphical boot | `mu_client` | `graphical-boot` | Cliente MU | Bevy window | optional `--asset-root` | local assets/config | asset validation error exits safely | player/dev starts graphical runtime |
| Headless boot | `mu_client --headless` | `headless-boot` | ready-for-login | stdout state | optional `--asset-root` | local assets/config | `asset-check-failed` | QA/dev smoke only |
| Control smoke | `--control-http ADDR` | `control-http` | control-http listening | local HTTP state | command name | app state | bind/server error | QA/dev automation only |
| Rust validation | local commands/scripts | `local-validation` | terminal output | logs | command selection | Cargo workspace | non-zero command exit | maintainer/dev only |

## Requirements

- Non-headless mode must not return `AppState::Exit` immediately.
- The graphical runtime must use Bevy and register existing app/render/UI/audio resources where practical.
- Headless and control-http modes must keep current behavior unless explicitly extended.
- Rust GitHub Actions workflows must be removed from this repo; local validation replaces them.
- Every code change must have automated tests or smoke validation.
- Docs must explain current usage and the fact that this is an initial graphical boot slice.

## Acceptance Criteria

- AC-01. `cargo run --manifest-path port_rust/Cargo.toml -p mu_client` starts the graphical runtime path instead of printing `exit`.
- AC-02. `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless` still prints `ready-for-login`.
- AC-03. Invalid `--asset-root` still returns `asset-check-failed` and exit code `3`.
- AC-04. Rust cross-platform GitHub Actions workflows are removed.
- AC-05. Local validation commands are documented.
- AC-06. Tests cover boot state and graphical app construction.

## Scope

In scope:

- Initial Bevy runtime boot.
- Minimal graphical app construction.
- Local validation documentation.
- Removing Rust GitHub Actions workflows.
- Feature workflow docs and pending-work tracking.

Out of scope:

- Full login UI.
- Fake-server playable world flow.
- Real terrain rendering in the window.
- Full C++ parity completion.
- New GitHub Actions for Rust or cross-platform builds.

## Open Questions

None blocking. Assumption: this slice may prove graphical boot without completing gameplay.
