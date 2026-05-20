# No pending work

The previously listed follow-ups were already completed in code or were stale
after later port work landed. Add new entries here only for real pending work.

# [critical]Complete playable Rust Bevy parity

The current Rust client only implements the first graphical boot slice. Remaining work: login UI, server select, character select, fake-server login/world validation, game-server protocol flow, terrain/player/entity rendering parity, movement sync, chat, inventory, GameShop, MU Helper, editor/admin tools, OpenMU compatibility, and final C++ parity evidence. Next step: create the next feature workflow for fake-server-backed login-to-world bootstrap before editing gameplay code.

# [high]Fake-server login/world bootstrap slice

The next phase is now planned in `.features/20260520-0400-fake-server-login-world-bootstrap/`. Implementation still needs to wire the Bevy bootstrap state machine, fake-server-driven session transitions, and initial world load; the pending work should be tracked until the slice ships.
