# Session-Backed UI Flow

Status: READY_FOR_PLAN

## Goal

Make `--control-http` useful while the graphical client is running by exposing a
session-backed control plane for the login -> server select -> character select
-> loading -> world flow. The control surface stays local-only and is meant for
QA/dev automation, not players.

## Users And Journeys

- QA/developer: starts `mu_client` with a graphical window and `--control-http`,
  then drives the auth/bootstrap flow from HTTP instead of a real mouse/keyboard
  session.
- QA/developer: probes the current session phase, UI route, and bootstrap state
  from HTTP while the Bevy runtime is active.
- QA/developer: keeps the existing headless smoke path unchanged.

## Requirements

- `--control-http` must work in the graphical runtime, not only in headless
  mode.
- The control snapshot must expose the current app state plus the route/session
  state needed to observe auth/bootstrap progress.
- The control API must allow scripted progression through login, server select,
  character select, loading, and world bootstrap states.
- Existing headless `--control-http` behavior must remain deterministic.
- Automated tests must cover the control command parsing and the route/state
  transitions.
- Usage docs must describe the graphical control-plane flow and the local-only
  command set.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Graphical control plane | `mu_client --control-http` | `graphical-control-plane` | Control HTTP | Local HTTP + Bevy runtime state | command name, current route, current session phase | loopback only | Unknown commands stay rejected; state probe stays deterministic | QA drives the flow by HTTP; players never see it |
| Session bootstrap probe | `UiRoute::Login` -> `UiRoute::World` | `session-bootstrap-probe` | Auth/bootstrap | HTTP state snapshot | login/server/character/world steps | loopback only | Login/server/character failures stay on safe auth surfaces | QA can inspect state without a real server UI |

## Acceptance Criteria

- AC-01. `mu_client` can start with a graphical window and `--control-http`
  bound to a local address.
- AC-02. HTTP commands can step the client through login, server select,
  character select, loading, and world bootstrap states.
- AC-03. The control snapshot reports the current app state and the
  route/session state used by the graphical runtime.
- AC-04. Headless `--control-http` smoke still reports the same deterministic
  state behavior.
- AC-05. Tests cover command parsing, command application, and route/state
  reporting.
- AC-06. Player-facing docs explain that the control API is local-only and for
  QA/dev automation.

## Scope

In scope:

- Graphical-runtime control HTTP wiring.
- Session-backed route/state snapshot reporting.
- Scriptable control commands for bootstrap progression.
- Usage docs for the local automation flow.

Out of scope:

- New gameplay features.
- Actual UI widget rendering work beyond route/state control.
- Network protocol expansion beyond the control-plane bridge.
- Non-local or production control exposure.

## Open Questions

None blocking. Assumption: the control server remains loopback-only and is
strictly a dev/QA harness.
