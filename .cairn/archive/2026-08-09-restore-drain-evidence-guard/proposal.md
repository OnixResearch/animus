## Why

The Cairn drain workflow requires `scripts/check-drain-evidence-guard.rs` before a completed worktree is removed. Animus does not contain that guard, so verified worktrees accumulate or operators risk deleting ignored owner-only `.pi/` evidence without a checked preflight.

## What Changes

- Add a bounded Rust guard that scans explicit or registered linked Git worktrees without reading evidence contents.
- Fail closed when a disposable worktree contains `.pi/` entries or cannot be classified safely.
- Add positive and negative self-tests, a Nix check, and operator documentation.

## Impact

- **Files**: `scripts/check-drain-evidence-guard.rs`, `flake.nix`, lifecycle documentation, and focused verification documentation
- **Testing**: guard self-test, positive and negative temporary-worktree fixtures, Cairn gates, and the focused Nix check
