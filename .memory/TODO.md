# [high] Expand gameplay coverage beyond duel

`mu_gameplay` now ports the legacy duel state, but the gameplay layer is still
far from parity. Next work should cover the next isolated gameplay/social slice
(`party`, then related social/admin surfaces) and wire the resource into the
runtime where it is actually consumed.

