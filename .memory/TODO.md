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

# [medium] Guild action follow-up

Friend add/delete control-plane commands are now wired through the live
session bridge, but the guild-side action commands and any remaining social
management flows still need their own slice if parity is still required.

# [medium] Remaining guild action follow-up

`guild-role-assign` is done; guild join/create/union packet commands still
remain for later if full guild parity is still required.

# [medium] Remaining guild create/union follow-up

`guild-join` is now done. The next guild parity slice can wire
`guild_create_request` or the remaining guild union management flows if full
guild parity is still required.

# [medium] Duel lifecycle follow-up

`duel-start` and `duel-stop` are now bridged through the control plane, but
the rest of the legacy duel lifecycle still needs a later slice if full duel
parity is still required.

# [medium] Selected-character e2e validation

The selected-character world-identity slice is implemented and unit-tested,
but the required e2e-validator handoff was not run in this turn. Next step is
to validate the running client end to end and record the verdict in the
feature progress doc.

# [medium] Friend inbox windowed e2e handoff

The friend inbox live-data slice is implemented and headless-smoke tested,
but the windowed e2e pass was not available in this session. Next step is to
run the running client in a visible window, exercise `friend-inbox`, and
record the verdict in the feature progress doc.
