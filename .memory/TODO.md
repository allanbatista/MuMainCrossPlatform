# Pending work

Track real follow-up work here. Remove entries only when the debt is no longer
relevant.

# [critical]Complete playable Rust Bevy parity

The current Rust client only implements the first graphical boot slice. Remaining work: login UI, server select, character select, fake-server login/world validation, game-server protocol flow, terrain/player/entity rendering parity, movement sync, chat, inventory, GameShop, MU Helper, editor/admin tools, OpenMU compatibility, and final C++ parity evidence.

# [medium]Session-backed UI flow

The movement-authority slice is done. Next work is to finish the
login/server-select/character-select HUD flow in the Bevy runtime so the
client can progress through the UI instead of only booting into world state.

# [medium]Visible session UI shell

The control-plane bridge is now done. Next work is to render the actual
login/server-select/character-select shell in Bevy so the mirrored route and
session state become a visible UI instead of HTTP-only automation.
