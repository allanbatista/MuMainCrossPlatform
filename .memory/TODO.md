# [high] Expand gameplay coverage beyond duel

`mu_gameplay` now ports the legacy duel state, but the gameplay layer is still
far from parity. Next work should cover the next isolated gameplay/social slice
(`party`, then related social/admin surfaces) and wire the resource into the
runtime where it is actually consumed.

# [medium] Wire party manager into a runtime consumer

`mu_gameplay::party` is implemented, but the first runtime/UI consumer that
reads party state is still pending. The next step is to hook the resource into
the surface that actually uses party membership and helper/HUD logic.

# [high] Continue gameplay systems parity

`F8.S1.T1` through `F8.S1.T3` are done, but the gameplay slice still needs
items/inventory/equipment/vault, NPC, social, events, and pet/summon coverage to
reach parity. Next concrete step is `F8.S2.T1`.

# [medium] Start NPC/shop slice

`F8.S2.T1` is complete. The next concrete gameplay slice is `F8.S2.T2`, which
should add `mu_gameplay::npc` and `mu_ui::{npc,shop}`.
