# Evidence: adapt-clankers-safe-receipts-artifacts

## Implementation evidence

- Added no-std `ArtifactHash`, `ArtifactEnvelope`, artifact kind, redaction class, canonical envelope, dependency sorting/deduplication, BLAKE3 identity, display/parse validation, and review-output payload controls in `animus-adapters`.
- Added no-std `SafeReceipt` DTOs with schema, status, issue code, operation class, labels, bounded counts, artifact hashes, backend/interchange labels, and explicit redaction markers.
- Added typed leak validation for raw prompts, provider JSON, credential-looking values, absolute paths, raw tool input/output, raw model output, oversized metadata, and oversized counts.
- Added `scripts/cairn/check-safe-receipts-artifacts` plus a Nix `safe-receipts-artifacts` check and README verification entry for the focused rail.

## Verification evidence

- Baseline before edits: `nix develop -c cargo test -p animus-adapters` passed.
- After edits: `nix develop -c cargo test -p animus-adapters` passed.
- After edits: `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` passed.
- Focused rail: `nix develop -c scripts/cairn/check-safe-receipts-artifacts --root .` passed.
- Nix rail: `nix build .#checks.x86_64-linux.safe-receipts-artifacts --no-link -L` passed after staging the new source and rail files into the flake source snapshot.

## Non-claims

This evidence does not claim artifact storage, Toolpath export, provider session importers, full transcript archival, live provider behavior, TUI behavior, daemon behavior, or production readiness.
