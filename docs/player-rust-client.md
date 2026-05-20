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

- Login and server selection screens.
- Character selection and character creation.
- The Bevy bootstrap now carries the client from login to character select and
  into the loaded world once the map handoff packet arrives.
- The world now opens a visible Bevy world shell with a camera, lighting,
  terrain, and sampled scene markers from the loaded world bundle.
- The world shell now seeds a local avatar marker and keeps it synchronized
  with authoritative movement replies when the live session is active.
- The world, HUD, inventory, NPC/shop, trade, quests, MU Helper, and GameShop
  surfaces documented elsewhere in this repo still remain a staged port, not a
  finished parity pass.
- Safe error screens when login, connection, or asset validation fails.

## World Controls

- `WASD` moves the local avatar in the world shell.
- When connected, movement requests are sent through the live session and the
  returned position updates reconcile the runtime pose.

## QA / Dev Control Plane

- Start the graphical client with `--control-http 127.0.0.1:0` to expose the
  local HTTP automation surface while the Bevy window is running.
- `GET /state` reports `state`, `ui_route`, `session_phase`, `last_command`,
  and `command_count`.
- `POST /command?name=ready-for-login|server-select|character-select|loading|world|login-success|login-failure|exit|ping`
  can step the auth/bootstrap flow for local QA and smoke tests.
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
