# Evidence

- Baseline focused rail before workspace core edits: `cargo test -p animus-adapters workspace --locked` — passed, 9 tests.
- Cairn gates before implementation: `nix run .#cairn -- gate proposal adapt-branchfs-speculative-workspaces --root .`, `gate design`, `gate tasks` — passed.
- Focused rail after implementation: `cargo test -p animus-adapters workspace --locked` — passed, 15 tests.
- Broader adapter rail: `cargo test -p animus-adapters --locked` — passed, 121 tests plus doc tests.
- Formatting: `cargo fmt --check` — passed after formatting.
- Lint rail: `cargo clippy -p animus-adapters --all-targets --locked -- -D warnings` — passed.
- Cairn validation and evidence safety were run before closeout; rerun after sync/archive as final lifecycle evidence.
