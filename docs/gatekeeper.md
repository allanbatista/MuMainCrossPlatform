# Gatekeeper

The Rust `gate` route snapshots the legacy gatekeeper access window used around
Castle Siege hunting zones.

It models the shared `CUIGateKeeper` state and the three interaction modes:

- `guest-public`: public access is enabled, the fee is shown, and the enter
  button is disabled when the player cannot pay.
- `guest-private`: the gate is private and the enter flow is blocked.
- `guild-member`: guild members can enter directly.
- `guild-master`: the public toggle and fee controls are visible.

Use `gate_screen` with `GateScreenState::{GuestPublic, GuestPrivate,
GuildMember, GuildMaster}`.

```rust
use mu_ui::{gate_screen, GateScreenState};

let snapshot = gate_screen(GateScreenState::GuildMaster).snapshot();
```

The snapshot keeps the legacy `public`, entrance fee, editable fee buffer,
fee-step, max fee, and enter enablement values so the UI route stays
deterministic in tests.
