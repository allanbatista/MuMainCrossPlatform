# [medium] Remaining port parity

The Rust port still has remaining gameplay and UI parity work beyond the
trade shell slice. Continue breaking the overall goal into smaller planned
phases before implementing the next surface.

# [medium] Terrain mesh follow-up

The world route now renders a bundle-driven heightfield terrain, layered
terrain surface, and converted object/NPC/monster models, but broader legacy
terrain/material parity still remains. The next slice should decide which
remaining visible surface or interaction is highest priority.

# [medium] Friend/guild interaction parity

The Rust client now exposes visible friend and guild shells, but the actual
interactive friend list, mail actions, guild member management, and union
flows remain snapshot-only. The next slice should wire the live gameplay
actions if parity is still required.

# [medium] Duel interaction parity

The Rust client now exposes a visible duel shell, but live duel interaction
and networked duel state changes remain out of scope for this slice. The next
duel-related pass should wire the interactive flow if parity is still needed.

# [medium] Friend/guild live packet wiring

The friend and guild shells now support local control-http view overrides,
but the actual friend, letter, and guild gameplay packet handlers are still
snapshot-only. The next slice should wire the live networking path if parity
is still required.

# [medium] Friend/guild response decoding

The friend and guild routes now trigger their live list requests on login,
but the response decoding and snapshot population for the roster, letters,
and guild data still remain for a later slice.

# [medium] Character create packet wiring

The character-create shell now renders locally and the control HTTP smoke
command opens it, but the actual create-character packet submission and name
validation/network response handling still need a later networking slice.
