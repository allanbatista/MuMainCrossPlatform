# [high] Expand gameplay coverage beyond duel

`mu_gameplay` now ports the legacy duel state, but the gameplay layer is still
far from parity. Next work should cover the next isolated gameplay/social slice
(`party`, then related social/admin surfaces) and wire the resource into the
runtime where it is actually consumed.

# [medium] Wire party manager into a runtime consumer

`mu_gameplay::party` is implemented, but the first runtime/UI consumer that
reads party state is still pending. The next step is to hook the resource into
the surface that actually uses party membership and helper/HUD logic.
