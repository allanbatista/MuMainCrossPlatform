# Siege Warfare

This slice ports the legacy Castle Siege window as a snapshot-oriented UI
surface.

## Modes

- `Inactive` hides the battle surface and leaves only the close action.
- `Observer` shows the battlefield minimap without command controls.
- `Soldier` shows the battle skill strip and the minimap command icons.
- `Commander` shows the battle skill strip plus the commander groups and map
  command controls.

## Behavior

- The alpha button cycles the minimap opacity.
- Commander mode keeps the current command buffer and guild member markers in
  the snapshot state.
- The battle skill strip follows the legacy guild-status split used by the
  siege shell.

## Rust Code Locations

- `port_rust/crates/mu_ui/src/siege.rs`
- `port_rust/crates/mu_ui/src/lib.rs`
- `port_rust/crates/mu_ui/src/routes.rs`
- `src/source/UI/NewUI/Combat/NewUISeigeWarfare.cpp`
- `src/source/UI/NewUI/Combat/NewUISiegeWarBase.cpp`
- `src/source/UI/NewUI/Combat/NewUISiegeWarCommander.cpp`
- `src/source/UI/NewUI/Combat/NewUISiegeWarSoldier.cpp`
- `src/source/UI/NewUI/Combat/NewUISiegeWarObserver.cpp`
