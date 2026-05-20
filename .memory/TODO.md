# [medium] Remaining port parity

The Rust port still has remaining gameplay and UI parity work beyond the
trade shell slice. Continue breaking the overall goal into smaller planned
phases before implementing the next surface.

# [medium] Terrain mesh follow-up

The world route now renders a bundle-driven heightfield terrain, layered
terrain surface, and converted object/NPC/monster models, but broader legacy
terrain/material parity still remains. The next slice should decide which
remaining visible surface or interaction is highest priority.

# [medium] World camera zoom follow-up

The world route camera now follows the local avatar, but it still uses a
fixed follow offset and does not yet consume the persisted `[Camera] zoom`
setting or orbital mouse controls. The next slice should decide whether to
wire the saved zoom into the follow camera or split out a full orbital-camera
pass.
