# Proposal: Align Clash safety-harness patterns with Basalt/UCAN authority

## Why

Animus can learn from Clash's agent safety harness without adopting Clash wholesale. Clash proves useful ergonomics around capability categories, policy explanation, sandbox descriptors, and scenario-style hook tests. Animus already points toward a different authority boundary: Nickel-authored contracts, Basalt/UCAN capability checks, typed adapter DTOs, Steel revalidation, and redacted receipts.

A native Cairn change is needed so future implementation does not accidentally import Clash's Starlark policy engine, shell, plugin surface, audit format, or precedence-as-authority model. The change defines how selected Clash ideas translate into Animus terms while preserving Basalt/UCAN as the source of authorization.

## What Changes

- Define Clash as a bounded reference for selected safety-harness patterns only.
- Introduce a UCAN-first capability request shape for guarded runtime actions.
- Treat layered local policy as caveats and UX prompts, not ambient authority.
- Specify safe authorization receipts with BLAKE3 hashes and checked-capability evidence.
- Treat sandbox descriptors as execution constraints attached to authorized actions, not as proof of authorization.
- Add focused positive and negative verification rails for the capability, caveat, sandbox, and receipt behavior.

## Impact

- **Files**: New Cairn change artifacts under `cairn/changes/align-clash-patterns-with-ucan/`; later implementation may add an `animus-policy` or equivalent module/crate, Nickel policy contracts, docs, receipt DTOs, runtime dispatch hooks, and focused tests.
- **Testing**: Future implementation must include positive and negative tests for authorized actions, contract denials, missing UCAN grants, local caveat narrowing, session grant behavior, sandbox enforcement requirements, and receipt redaction.
- **Non-claims**: This change does not adopt Clash's Starlark DSL, full policy engine, brush shell, Claude plugin, npm/editor/LSP distribution, audit schema, or highest-precedence allow semantics.
