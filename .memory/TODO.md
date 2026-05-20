# No pending work

The previously listed follow-ups were already completed in code or were stale
after later port work landed. Add new entries here only for real pending work.

# [critical]Complete playable Rust Bevy parity

The current Rust client only implements the first graphical boot slice. Remaining work: login UI, server select, character select, fake-server login/world validation, game-server protocol flow, terrain/player/entity rendering parity, movement sync, chat, inventory, GameShop, MU Helper, editor/admin tools, OpenMU compatibility, and final C++ parity evidence.

# [high]Local movement sync

The world scene shell is shipped. Next work should make the player avatar
interactive: movement input, server-authoritative position sync, and the first
HUD/chat/inventory hooks that turn the visible world into a playable loop.
