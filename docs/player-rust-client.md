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
- The shared Options window is also available as a visible Bevy route and
  through the local control HTTP API with `options`.
- Character selection and the visible character-create route.
- The Bevy bootstrap now carries the client from login to character select and
  immediately requests the character list using the legacy locale byte
  (`en`/`eng` -> `0`, `pt`/`por` -> `1`, `es`/`spn` -> `2`) before the
  loaded world handoff starts.
- When the roster arrives, the bootstrap stays on character select until a
  `select-character` request names a roster entry; the same request can be
  sent through the local control HTTP API with the character name in the body
  or a `character=` query parameter.
- The character-create route opens the class picker and name prompt;
  `character-create` opens the shell and `create-character` submits the
  name, returning to character select on success or keeping the create
  error surface on failure.
- On the visible character-select surface, use `Up`/`Down` or `Left`/`Right`
  to move the selection and `Enter` to confirm without the control HTTP API.
  The shell shows the loading state until the roster is ready.
- The world now opens a visible Bevy world shell with a camera, lighting,
  a heightfield terrain derived from the loaded world bundle, a layered
  terrain surface from the bundle's first two converted texture slots plus
  the alpha map, the bundled terrain lightmap, sampled local/remote player
  markers grounded to the visible terrain surface, and real converted models
  for nearby objects, NPCs, and monsters.
- The world shell now prefers nearby scene entities when sampling the visible
  object, NPC, monster, and remote-player set instead of taking the first
  arbitrary entries, so the 3D scene better matches the local play area while
  still staying sampled for performance.
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
- The friend route now opens a visible Bevy shell that mirrors the mail
  manager snapshot and can be driven from the local control HTTP smoke path
  with `friend`, `friend-roster`, `friend-inbox`, `friend-compose`, and
  `friend-chat-rooms`; when the live friend list arrives, the shell overlays
  the decoded roster and server-state data from the session.
- The friend control plane also accepts `friend-add` and `friend-delete`
  commands with a friend name in the body or `friend=` so QA can drive the
  existing friend packet helpers through the live session.
- The guild route now opens a visible Bevy shell that mirrors the guild
  snapshot and can be driven from the local control HTTP smoke path with
  `guild`, `guild-summary`, `guild-members`, `guild-union`, `guild-no-guild`,
  and `guild-error`; when the live guild list arrives, the shell overlays the
  decoded score, rival name, and member roles from the session.
- The guild control plane also accepts `guild-join` commands with a guild
  master player ID in the body or `master_id=` so QA can drive the existing
  guild join packet helper through the live session.
- Opening `guild-union` in a logged-in session also sends the alliance list
  request once per activation before keeping the shell visible.
- When the friend or guild route opens while logged in, the client now sends
  the matching live list request once per activation before keeping the shell
  on screen, and the decoded roster state stays visible until logout or
  disconnect clears it.
- The duel route now opens a visible Bevy shell that mirrors the duel
  manager snapshot and can be driven from the local control HTTP smoke path
  with `duel`.
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
- `POST /command?name=ready-for-login|server-select|character-select|character-create|create-character|loading|world|login-success|login-failure|mu-helper|exit|ping`
  can step the auth/bootstrap flow for local QA and smoke tests.
- `POST /command?name=select-character` can continue from character select
  once the roster is visible. Pass the character name in the body or as
  `character=` when using the local control HTTP API.
- `POST /command?name=character-create` can open the visible character-create
  shell for local QA smoke.
- `POST /command?name=create-character` submits the visible character-create
  form. Pass the name in the body or as `character=`; names shorter than 4
  characters are rejected by the control plane.
- `POST /command?name=chat` can step into the visible chat route shell for
  local QA smoke; once open, the shell accepts typed chat and Enter sends the
  draft.
- `POST /command?name=npc|shop` can step into the visible NPC and shop route
  shells for local QA smoke.
- `POST /command?name=game-shop` can step into the visible GameShop route
  shell for local QA smoke.
- `POST /command?name=trade` can step into the visible trade route shell for
  local QA smoke.
- `POST /command?name=friend` can step into the visible friend route shell
  for local QA smoke. `friend-roster`, `friend-inbox`, `friend-compose`, and
  `friend-chat-rooms` select the matching friend subview.
- `POST /command?name=friend-add` and `POST /command?name=friend-delete`
  queue the matching friend packet helpers through the live session. Pass
  the friend name in the body or as `friend=`.
- `POST /command?name=guild` can step into the visible guild route shell for
  local QA smoke. `guild-summary`, `guild-members`, `guild-union`,
  `guild-no-guild`, and `guild-error` select the matching guild subview.
- `POST /command?name=guild-join` queues the guild join packet through the
  live session. Pass the guild master player ID in the body or as
  `master_id=`.
- `POST /command?name=guild-role-assign` queues the guild role-assignment
  packet through the live session. Pass the target player in `player=`, the
  role in `role=`, and the assignment type in `type=`.
- `POST /command?name=duel` can step into the visible duel route shell for
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
