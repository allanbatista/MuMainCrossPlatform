# [medium] Remaining port parity

The Rust port still has remaining gameplay and UI parity work beyond the
trade shell slice. Continue breaking the overall goal into smaller planned
phases before implementing the next surface.

# [medium] Chat input and packet send

The chat shell slice exposes the chat route visually and via control-http,
but it does not yet accept typed chat text or send/receive chat packets.
Next step: add the gameplay/chat state and network path for real chat input.
