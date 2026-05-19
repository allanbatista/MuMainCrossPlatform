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

# [high] Finish GameShop transaction safety

`F9.S2.T1` is complete. The next pending slice is `F9.S2.T2`, which should add
transaction/idempotency handling in `mu_gameplay::game_shop_transaction` and
`mu_network::shop`, with redacted logs and duplicate-purchase coverage.

# [high] Windows CI artifact pending

`F1.S2.T2` still needs a GitHub Actions run to produce the Windows x64 release
artifact, checksum, and log evidence for the workflow gate.

# [medium] Friend and guild UI routes

`mu_ui` now has a party consumer, but the `friend` and `guild` routes are still
missing concrete snapshot modules. Next pass should port the legacy friend
surface and then the guild surface so the route catalog stops advertising
unimplemented gameplay UI.
