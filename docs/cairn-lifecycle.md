# Cairn lifecycle in Animus

Animus uses native Cairn artifacts as durable lifecycle evidence.

## Layout

| Path | Meaning |
| --- | --- |
| `.cairn/specs/` | Accepted requirements baseline. Review this tree to understand current lifecycle commitments. |
| `.cairn/changes/` | Active change packages. Each package should contain `proposal.md`, `design.md`, `tasks.md`, and spec deltas when behavior or contracts change. |
| `.cairn/archive/` | Completed lifecycle history. Archive packages only after accepted specs are synced or manually repaired with evidence. |

## When a Cairn change is required

Create a Cairn change package before work that affects any of these surfaces:

- public API, CLI, SDK, protocol, TUI, or compatibility behavior;
- architecture, crate boundaries, functional-core / imperative-shell split, or dependency posture;
- mutation behavior, filesystem/process/network authority, sandboxing, credentials, or provider dispatch;
- receipt, evidence, provenance, session-ledger, schema, policy, or release-readiness contracts;
- roadmap scope or the ordering/ownership of active changes.

Direct edits are acceptable for typo fixes, fixture hardening, narrow documentation cleanup, or local test repairs when they do not affect public behavior, architecture, compatibility, mutation behavior, schema/receipt contracts, release/evidence contracts, or roadmap scope. Direct edits still need the smallest relevant repository checks.

## Change workflow

Run commands from the repository root through the pinned Cairn wrapper:

```sh
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal <change> --root .
nix run .#cairn -- gate design <change> --root .
nix run .#cairn -- gate tasks <change> --root .
```

Before closeout, review dry-run mutation plans:

```sh
nix run .#cairn -- sync <change> --root .
CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive <change> --root .
```

Execute only after the dry-run is non-blocking and the accepted-spec result is reviewed:

```sh
nix run .#cairn -- sync <change> --root . --execute
CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive <change> --root . --execute
```

If sync reports success but the accepted spec content does not gain the expected `r[...]` requirement IDs, manually merge the requirement text, record the repair in evidence, and rerun validation before archive.

## Evidence contract

Durable evidence should be a bounded summary, not a transcript dump. Prefer:

- command identity and status;
- change IDs, requirement IDs, issue codes, and counts;
- policy source identity and policy hash when Cairn reports one;
- sync/archive receipt hashes, plan hashes, and safe relative paths;
- output artifact hashes and redaction markers;
- unavailable checks with a blocker and next best validation path.

Durable evidence must omit raw prompts, credentials, provider payloads, raw private file bodies, full private paths, and unbounded command output by default. Store bulky logs under ignored local paths such as `target/` and summarize only bounded facts in committed evidence.

## Safe-evidence rail

Run the local evidence-safety check before archiving lifecycle changes:

```sh
scripts/cairn/check-evidence-safety --root .
```

The rail scans committed `evidence.md` summaries and negative fixtures for sentinel leak classes without printing matched private material.
