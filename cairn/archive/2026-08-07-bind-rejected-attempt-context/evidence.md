# Rejected-attempt context evidence

## Outcome

Animus now binds one externally reviewed rejected attempt to exact nominal identities. It projects a bounded redacted summary for a later fresh-context Ralph iteration.

The reusable core is VCS-neutral and authority-free. The standard host uses injected review, VCS, redaction, and classification services.

## Reference admission

The reviewed reference is Ralph the Riveter revision `9615b634df5498682cf3f9f3295df6eb6585d5cb`.

Its workspace manifest declares the MIT license. Animus adapts retained revision identity, rejection-reason binding, and bounded later context.

Animus does not adopt Riveter's task file, commit format, raw-diff prompt behavior, agent-owned pass flag, or mandatory Jujutsu workflow.

## Passing evidence

- Six pure runtime tests passed.
- Two injected standard-host tests passed.
- Four existing fresh-context tests passed.
- Two existing Ralph policy tests passed.
- Positive and negative Nickel policy fixtures passed.
- Rust formatting and focused Clippy checks passed with warnings denied.
- `scripts/config/check-nickel-configs` passed.
- `cargo check --workspace --all-targets --locked` passed.
- The component catalog and all positive and negative catalog fixtures passed.
- The runtime facade inventory and all positive and negative inventory fixtures passed.
- Proposal, design, repository validation, and evidence-safety checks passed.
- `nix build .#checks.x86_64-linux.rejected-attempt-context --no-link -L` passed on the local builder.

The configured remote builder was unavailable. Nix continued on the local builder and completed the requested check.

The focused receipt source identity is `86665f75ecc3460f621eb7e405419f4c3035fc6a8c0c5e97c834b85a34d67415`.

## Negative coverage

The tests and fixtures deny these cases:

- missing or invalid review evidence;
- wrong task or run bindings;
- ambiguous attempts;
- stale revisions, workspaces, or content;
- unknown VCS providers;
- unsafe or oversized summaries;
- raw diffs without policy, capability, classification, and byte admission;
- receipt completion or retry overclaims;
- weakened receipt non-claims;
- missing injected host services.

## Non-claims

This evidence does not prove review correctness. It does not grant lifecycle authority, authorize retry, create a run, mutate VCS state, or prove that a later attempt fixes the findings.
