# Rust Client Player Guide

Use the Rust client the same way you would use a normal game release: start the
launcher, point it at converted assets, choose a server, log in, and play.

## Start

The Rust client expects a converted asset root and a server address:

```bash
mu_client --asset-root /path/to/converted/assets --server 127.0.0.1:55901
```

On the packaged release, the asset root and server settings are usually stored
in the local config file instead of being typed every time.

## First Run

- The client reads `config/client.toml` for local settings.
- Video, audio, controls, network, and locale settings are persisted there.
- Passwords, tokens, and raw session IDs are not saved.
- If the asset root is missing or invalid, the client stops before login and
  shows the asset validation failure state.

## What Players See

- A visible Bevy auth shell for login, server selection, and character
  selection while the client boots.
- Character selection and character creation.
- The Bevy bootstrap now carries the client from login to character select and
  immediately requests the character list using the legacy locale byte
  (`en`/`eng` -> `0`, `pt`/`por` -> `1`, `es`/`spn` -> `2`) before the
  loaded world handoff starts.
- When the roster arrives, the bootstrap stays on character select until a
  `select-character` request names a roster entry; the same request can be
  sent through the local control HTTP API with the character name in the body
  or a `character=` query parameter.
- On the visible character-select surface, use `Up`/`Down` or `Left`/`Right`
  to move the selection and `Enter` to confirm without the control HTTP API.
  The shell shows the loading state until the roster is ready.
- The world now opens a visible Bevy world shell with a camera, lighting,
  a heightfield terrain derived from the loaded world bundle, a layered
  terrain surface from the bundle's first two converted texture slots plus
  the alpha map, the bundled terrain lightmap, sampled player markers, and
  real converted models for objects, NPCs, and monsters.
- The world camera now follows the local avatar and uses the saved
  `[Camera] zoom` value, so mouse-wheel zoom in the world route stays framed
  while the 3D scene is active and persists through `config/client.toml`.
- The world now also shows a visible HUD overlay with the legacy main-frame
  gauges and buttons while the 3D scene is active.
- The world HUD stack now also shows the visible chat log, minimap, and
  hotkey surfaces while the 3D scene is active.
- The world shell now seeds a local avatar marker and keeps it synchronized
  with authoritative movement replies when the live session is active.
- If the terrain texture slots, alpha data, or lightmap are unavailable, the
  visible surface falls back safely to the solid shell instead of breaking
  the world route.
- Press Tab while the world route is active to open or close the visible
  inventory shell.
- The NPC and shop routes now open visible Bevy shells that can be driven
  from the runtime or the local control HTTP smoke path.
- The GameShop route now opens a visible Bevy shell that can be driven from
  the runtime or the local control HTTP smoke path with `game-shop`.
- The chat route now opens a visible Bevy shell with a draft line; when the
  session is logged in, typing printable text and pressing Enter sends a
  public chat packet from the local player.
- The trade route now opens a visible Bevy shell that can be driven from the
  runtime or the local control HTTP smoke path.
- The party route now opens a visible Bevy shell that can be driven from the
  runtime or the local control HTTP smoke path.
- The gate route now opens a visible Bevy shell that can be driven from the
  runtime or the local control HTTP smoke path.
- The quests route now opens a visible Bevy shell that can be driven from the
  runtime or the local control HTTP smoke path.
- The MU Helper route now opens a visible Bevy shell that mirrors the helper
  runtime snapshot and can be driven from the local control HTTP smoke path
  with `mu-helper`.
- The world, HUD, chat, inventory, NPC/shop, trade, party, gate, quests, and GameShop
  surfaces documented elsewhere in this repo still remain a staged port, not a
  finished parity pass.
- Safe error screens when login, connection, or asset validation fails.

## World Controls

- `WASD` moves the local avatar in the world shell.
- `Tab` opens and closes the inventory shell while the world route is active.
- When connected, movement requests are sent through the live session and the
  returned position updates reconcile the runtime pose.

## QA / Dev Control Plane

- Start the graphical client with `--control-http 127.0.0.1:0` to expose the
  local HTTP automation surface while the Bevy window is running.
- `GET /state` reports `state`, `ui_route`, `session_phase`, `last_command`,
  and `command_count`.
- `POST /command?name=ready-for-login|server-select|character-select|loading|world|login-success|login-failure|mu-helper|exit|ping`
  can step the auth/bootstrap flow for local QA and smoke tests.
- `POST /command?name=select-character` can continue from character select
  once the roster is visible. Pass the character name in the body or as
  `character=` when using the local control HTTP API.
- `POST /command?name=chat` can step into the visible chat route shell for
  local QA smoke; once open, the shell accepts typed chat and Enter sends the
  draft.
- `POST /command?name=npc|shop` can step into the visible NPC and shop route
  shells for local QA smoke.
- `POST /command?name=game-shop` can step into the visible GameShop route
  shell for local QA smoke.
- `POST /command?name=trade` can step into the visible trade route shell for
  local QA smoke.
- `POST /command?name=party` can step into the visible party route shell for
  local QA smoke.
- `POST /command?name=gate` can step into the visible gate route shell for
  local QA smoke.
- `POST /command?name=quests` can step into the visible quests route shell
  for local QA smoke.
- `POST /command?name=mu-helper` can step into the visible MU Helper route
  shell for local QA smoke.
- `exit` also shuts down the graphical Bevy process, which is handy for smoke
  automation.

## If Something Fails

- `asset-check-failed` means the converted asset root or manifest is wrong.
- Login failures return to the login state instead of corrupting the session.
- A disconnect during the map handoff is treated as part of the normal
  login-to-world transfer, not as a fatal error.
- Connection drops return the client to a safe state and log a diagnostic.

## QA / Dev Smoke

For local validation, the same client binary can be started in headless mode and
with a local control HTTP endpoint:

```bash
mu_client --headless --control-http 127.0.0.1:0
```

That mode is for testing and automation, not for normal players.

See also:

- `docs/rust-client.md`
- `docs/control-http.md`
- `port_rust/README.md`
