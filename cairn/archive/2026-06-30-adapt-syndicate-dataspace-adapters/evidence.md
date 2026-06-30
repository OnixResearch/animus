# Evidence: adapt Syndicate dataspace adapters

## Scope

- Change: `adapt-syndicate-dataspace-adapters`
- Requirement IDs: `r[agent_harness_runtime.syndicate_dataspace.*]`
- Live Syndicate transport: unavailable by design for this change; release evidence is deterministic fake-dataspace and dependency-isolation coverage.
- Worktree note: repository had pre-existing uncommitted/staged files before this drain. Closeout evidence names only bounded command/status facts.

## Lifecycle gates

- `nix run .#cairn -- validate --root .`: PASS before closeout edits (`valid: true`, `changes: 72`, `specs_validated: 73`).
- `nix run .#cairn -- gate proposal adapt-syndicate-dataspace-adapters --root .`: PASS, no issues.
- `nix run .#cairn -- gate design adapt-syndicate-dataspace-adapters --root .`: PASS, no issues.
- `nix run .#cairn -- gate tasks adapt-syndicate-dataspace-adapters --root .`: PASS, no issues before implementation task markers were checked.
- `scripts/cairn/check-roadmap --root .`: PASS; unrelated active blockers reported for downstream changes only.
- `nix run .#cairn -- sync adapt-syndicate-dataspace-adapters --root .`: PASS dry-run, unblocked one-delta plan.
- `nix run .#cairn -- sync adapt-syndicate-dataspace-adapters --root . --execute`: completed, but the accepted spec content hash did not change and `syndicate_dataspace` IDs were absent; requirement text was manually merged into `cairn/specs/agent-harness-runtime/spec.md` per repo Cairn workflow guidance.

## Focused implementation checks

- `nix develop -c cargo test -p animus-adapters --locked dataspace::`: PASS; 9 dataspace tests passed covering bounded publish/subscribe, scoped cancellation, deterministic stale cleanup, ownership transfer, raw-payload rejection, cross-workspace rejection, unsupported query rejection, empty-redaction rejection, and missing owner rejection.
- `nix develop -c scripts/cairn/check-syndicate-dataspace-isolation --root .`: PASS; script self-tests passed and no default runtime/core/adapter dependency leak was reported.
- `nix develop -c cargo test -p animus-adapters --locked`: PASS; 86 unit tests passed and doc-tests passed.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: PASS.
- `nix build .#checks.x86_64-linux.syndicate-dataspace-isolation --no-link -L`: PASS; Nix check ran the isolation rail and completed.

## Non-claims

- No live Syndicate daemon, network transport, credential handling, or production orchestration backend is claimed.
- Dataspace assertions remain optional host-adapter projections, not durable ledger/provenance authority.
