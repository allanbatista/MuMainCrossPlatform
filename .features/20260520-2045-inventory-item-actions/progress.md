---
status: DONE_FOR_INVENTORY_ITEM_ACTIONS
---

# Inventory Item Actions Progress

Current state: the control-plane, bootstrap, runtime plumbing, docs, and
validation are complete. The Rust client now handles inventory use, equip,
and unequip through the control plane and runtime bridge.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-2045-inventory-item-actions/spec.md`, `.features/20260520-2045-inventory-item-actions/plan.md`, `.features/20260520-2045-inventory-item-actions/progress.md` | `.features/20260520-2045-inventory-item-actions/spec.md`, `.features/20260520-2045-inventory-item-actions/plan.md`, `.features/20260520-2045-inventory-item-actions/progress.md` | feature-workflow audit output | feature-doc audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | control-command variants, snapshot payloads, and request parsing for inventory use/equip/unequip | `inventory-use`, `inventory-equip`, and `inventory-unequip` are routed through the control snapshot and runtime sync | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | consume-item queue helper and packet dispatcher for inventory-use | `queue_consume_item_request` and the bootstrap packet bridge dispatch the inventory-use request | none |
| F1.S1.T3 | done | local | `port_rust/crates/mu_gameplay/src/equipment.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_gameplay/src/equipment.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | runtime mutation for use/equip/unequip and unchecked equip helper | runtime inventory/equipment mutation mirrors the legacy move semantics while keeping the local shell in sync | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | unit tests for parsing, packet encoding, and runtime state transitions | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `docs/inventory.md`, `.codexpotter/kb/README.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `docs/inventory.md`, `.codexpotter/kb/inventory-item-actions-control-plane.md`, `.codexpotter/kb/README.md` | usage docs and KB note for inventory actions | docs and KB now describe `inventory-use`, `inventory-equip`, and `inventory-unequip` | none |
| F3.S1.T1 | done | local | validation logs | validation logs | fmt, tests, build, and smoke validation | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, headless control-http smoke on `inventory-use`, `inventory-equip`, `inventory-unequip`, and `exit` | none |
| F3.S1.T2 | done | local | `.codexpotter/kb/README.md`, `.memory/TODO.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | `.codexpotter/kb/README.md`, `.memory/TODO.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | KB/memory/progress sync and commit evidence | workflow bookkeeping already captured the inventory-item slice; this pass only synchronized the stale feature progress doc | none |

## Files Touched

New:

- `.features/20260520-2045-inventory-item-actions/spec.md`
- `.features/20260520-2045-inventory-item-actions/plan.md`
- `.features/20260520-2045-inventory-item-actions/progress.md`

Modified:

- `.features/20260520-2045-inventory-item-actions/progress.md`

Removed:

- none

## Done

- Created and audited the feature workflow docs for the inventory item action
  slice. Validation: `rtk node /home/allanbatista/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-2045-inventory-item-actions`.
- Synced the stale progress control doc to the shipped inventory-use/equip/
  unequip implementation so the workflow state matches the runtime and docs.
