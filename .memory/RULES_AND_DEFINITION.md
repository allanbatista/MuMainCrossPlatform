# Document usage and behavior

When a change adds or alters a user-facing flow, add or update usage
documentation in `docs/` or `port_rust/docs/` so readers know how to use it
and, when useful, how it behaves. Keep the docs at the usage level unless the
rules or formulas themselves are the point.

# Prefix shell commands with rtk

Always prefix shell commands with `rtk`.

# Follow the feature workflow

Before implementation, read the project AGENTS instructions and follow the
spec -> plan -> progress workflow.

# No Rust cross-platform GitHub Actions

Do not add or rely on GitHub Actions for Rust cross-platform client builds. Use local scripts, documented commands, and manual validation evidence instead.

# Playable Rust port gates

For the Rust Bevy client port, every generated code change needs automated tests; all tests must pass; the app must start; fake server validation is required when relevant; review findings must be fixed while apparent; needed docs must be updated; final work should be committed and pushed.
