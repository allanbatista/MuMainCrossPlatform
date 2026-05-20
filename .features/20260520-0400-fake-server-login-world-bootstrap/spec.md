# Fake-Server Login Bootstrap

Status: READY_FOR_PLAN

## Goal

Connect the Bevy client shell to the existing fake-server, session, and UI
snapshot infrastructure so the Rust client can progress from login to server
select, character select, and initial world bootstrap instead of stopping at
the graphical boot shell. This slice is the first playable login path after
the Bevy runtime boot; it is not full gameplay parity.

## Users And Journeys

- QA/developer: runs `mu_client --headless --control-http 127.0.0.1:0` with a
  local fake server, drives login success/failure/disconnect, and inspects the
  reported state without the process exiting.
- Player/developer: starts graphical `mu_client`, sees login/server select/
  character select surfaces, and enters an initial world once the session is
  established.
- QA/developer: invalid credentials, missing server, or a disconnect return to
  a safe login/error surface and keep the process alive for inspection.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Login | `UiRoute::Login` | `login` | Login | UI snapshot | Username, password, remember username | Login response, selected server, account state | Invalid credentials and server failures stay on login with a notice | Players use the normal auth shell; QA drives deterministic state changes |
| Loading | `UiRoute::Loading` | `loading` | Loading | UI snapshot | Current bootstrap target | Session phase, bootstrap progress | Shows a loading surface while the runtime advances between login and world | QA can observe deterministic progress; players see a normal wait screen |
| Server select | `UiRoute::ServerSelect` | `server-select` | Server Select | UI snapshot | Server list, selected server | Connect-server response, server capacity list | Empty or unavailable server lists keep the user on server select with a notice | Players choose a server; QA can script the list locally |
| Character select | `UiRoute::CharacterSelect` | `character-select` | Character Select | UI snapshot | Character roster, selected slot | Login response, character roster | Empty roster or a bad selection stays on character select with a notice | Players pick the active character; QA can assert deterministic roster state |
| World bootstrap | `UiRoute::World` | `world` | World | Bevy window + HUD snapshot | Selected character, selected world id | `--asset-root`, terrain bundle, render/world projection, session phase | Missing assets or world data stay on a safe loading/error surface instead of exiting | Players enter the initial world; QA can smoke the bootstrap path locally |
| Error | `UiRoute::Error` | `error` | Error | UI snapshot | Failure type, retry target | Validation, login, or bootstrap failure details | Shows a safe error surface with retry/return actions | QA sees a deterministic failure state; players are not dumped to process exit |

## Requirements

- The runtime must use the existing `mu_network::Session` and
  `mu_app::SessionState` transition model rather than inventing a second login
  state machine.
- The fake-server harness must remain local-only and deterministic.
- Successful bootstrap must load the initial world bundle from the configured
  asset root and keep the render/world projections in sync.
- Headless and control-http smoke behavior must remain deterministic.
- Failure states must not exit the process; they must land on login, loading,
  server select, character select, or error.
- Every code change must have automated tests.
- Usage docs must explain how to smoke the login path with the fake server.

## Acceptance Criteria

- AC-01. A local fake-server scenario can drive `mu_client` from login success
  through server select, character select, and initial world bootstrap.
- AC-02. Login failure or disconnect keeps the client alive and lands on a
  safe login/error state instead of exiting.
- AC-03. A valid `--asset-root` loads the initial world bundle and the world
  projection stays consistent with the selected character/world.
- AC-04. Headless `--control-http` smoke continues to report deterministic
  state transitions.
- AC-05. Automated tests cover the session transitions, route changes, and the
  initial world bootstrap.
- AC-06. Usage docs explain the local fake-server smoke path.

## Scope

In scope:

- Session bootstrap from login to world entry.
- UI route progression for login, server select, character select, loading,
  error, and world bootstrap.
- Fake-server-backed smoke and test coverage.
- Initial world bundle loading from the asset root.
- Usage documentation for the new smoke path.

Out of scope:

- Full gameplay systems such as movement, combat, chat, inventory, trade, or
  GameShop.
- Editor/admin parity.
- Network protocol expansion beyond what the existing login/bootstrap path
  already needs.
- Long-term world rendering beyond the initial bootstrap scene.

## Open Questions

None blocking. Assumption: the first bootstrap world uses the existing
`world_1` sample bundle until character/world selection is wired to server data
in a later slice.
