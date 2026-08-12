# Baseline evidence

Date: 2026-08-10

Published source: `526519b8b66a68a62aece40f0d5b6c6ed4e6a08e`

All commands used the pinned Nix development shell and external Cargo target directory.

- Session authority profile: 10 passed.
- Runtime authority seal: 10 passed.
- Active-turn recovery: 17 passed.
- Session branch explorer: 13 passed.
- Conversation projection: 12 passed.
- Session event replay: 4 passed with `session-event-replay` enabled.
- Interrupted tool repair: 16 passed.
- Redacted session-tree export: 8 passed.
- Session branch planner: 6 passed.
- The control filter passed 26 related runtime tests, including control-gated active-turn behavior.

The initial `runtime_authority_seal` text filter selected no Rust test names. The corrected `authorityseal::tests` filter selected and passed all 10 authority-seal tests.

This baseline proves only the observed test behavior at the stated source revision. It does not prove semantic equivalence or release readiness.
