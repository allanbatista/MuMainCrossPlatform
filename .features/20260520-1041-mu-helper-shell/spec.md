# MU Helper Shell

Status: READY_FOR_PLAN

## Goal

Expose the existing MU Helper runtime and snapshot model in the Bevy client so
the Rust port can show the helper surface from `mu_app` instead of keeping it
isolated in `mu_gameplay` / `mu_ui`.

This slice is a runtime exposure pass, not full MU Helper interaction parity.

## Users And Journeys

- QA/developer: starts `mu_client --headless --control-http 127.0.0.1:0`,
  drives a local `mu-helper` smoke command, and inspects the reported route
  and session state without the process exiting.
- Player/developer: opens the graphical client and reaches the MU Helper
  surface from the runtime route system, seeing the existing helper snapshot.
- QA/developer: a disconnected or invalid helper state stays visible in the
  shell so the process remains inspectable.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| MU Helper | `UiRoute::MuHelper` | `mu-helper` | MU Helper | Bevy shell + helper snapshot | Helper execution state, config summary, status detail, actions | `MuHelperRuntime`, helper config validation, route/session state | Invalid config and resource/server limits stay visible in the shell with a reason | Players see the helper window; QA can smoke it through control HTTP |
| Control smoke | local HTTP command | `mu-helper` | MU Helper | control snapshot | route and session phase | local control plane | stays local and deterministic | QA uses it to drive the runtime shell |

## Requirements

- The Bevy runtime must register the existing `MuHelperRuntimePlugin`.
- The runtime must render `mu_ui::mu_helper_screen()` when `UiRoute::MuHelper`
  is active.
- The control HTTP plane must expose a `mu-helper` command that can drive the
  visible shell for local smoke tests.
- The shell must preserve the helper status detail and visible config summary
  from the existing runtime model.
- Headless and graphical smoke behavior must remain deterministic.
- Every code change must have automated tests.
- Usage docs must explain how to reach the helper shell and smoke it locally.

## Acceptance Criteria

- AC-01. The graphical client can show the MU Helper shell from the runtime
  route system.
- AC-02. `--control-http` can drive the helper shell with a `mu-helper`
  command.
- AC-03. The shell renders the existing helper state, config summary, and
  status detail.
- AC-04. Automated tests cover the shell visibility and control-plane wiring.
- AC-05. Usage docs explain the helper shell and smoke path.

## Scope

In scope:

- Runtime registration of the helper gameplay resource.
- Route-gated Bevy shell rendering for MU Helper.
- Local control-plane smoke command and state sync.
- Usage documentation for the shell.

Out of scope:

- Full interactive MU Helper button handling.
- Save-payload editor parity beyond the existing runtime model.
- New gameplay rules or config limits.

## Open Questions

None blocking. Assumption: the shell is route-gated like the other gameplay
surfaces and is driven by the existing helper runtime snapshot, not by new
network behavior.
