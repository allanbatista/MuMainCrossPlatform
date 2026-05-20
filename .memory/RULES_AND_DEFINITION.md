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
