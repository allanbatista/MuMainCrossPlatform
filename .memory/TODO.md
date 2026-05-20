# Pending work

Track real follow-up work here. Remove entries only when the debt is no longer
relevant.

# [critical]Complete playable Rust Bevy parity

The current Rust client only implements the first graphical boot slice. Remaining work: login UI, server select, character select, fake-server login/world validation, game-server protocol flow, terrain/player/entity rendering parity, movement sync, chat, inventory, GameShop, MU Helper, editor/admin tools, OpenMU compatibility, and final C++ parity evidence.

# [high]Local movement sync

The world scene shell is shipped. Next work should make the player avatar
interactive: movement input, server-authoritative position sync, and the first
HUD/chat/inventory hooks that turn the visible world into a playable loop.

# [medium]Server-authoritative movement follow-up

The local avatar is now seeded and moves in the world shell, but the motion is
still client-side. Next work is to replace the placeholder spawn with real
game-server authority, keep remote/local pose sync consistent, and begin the
HUD/chat/inventory hooks that build the playable loop.

# [medium]Session-backed UI flow

The movement-authority slice is done. Next work is to finish the
login/server-select/character-select HUD flow in the Bevy runtime so the
client can progress through the UI instead of only booting into world state.
