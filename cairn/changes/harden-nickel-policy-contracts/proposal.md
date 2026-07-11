## Why

Animus uses Nickel for several reviewable policies, but the contract depth is inconsistent. `policy/model-routing/policy.ncl`, `policy/process-jobs/policy.ncl`, `policy/steel-runtime/policy.ncl`, and `policy/tool-catalog/catalog.ncl` mostly constrain fields as `String` or `Number` even when the Rust consumers and repository checkers rely on closed vocabularies, positive budgets, BLAKE3 identities, confined paths, unique names, and cross-field invariants.

That leaves malformed policy source exportable until a later Rust or custom-checker stage, duplicates primitive definitions across files, and makes generated JSON freshness weaker than semantic parity. Animus already demonstrates stronger Nickel patterns in its provider registry and Cairn indexes; the runtime policy families should use the same authoring-time discipline.

## What Changes

- Add a shared Animus Nickel policy-contract prelude for exact schema literals, bounded integers, closed vocabularies, lowercase BLAKE3 identities, safe repository-relative paths, and reusable collection predicates.
- Harden model-routing, process-job, Steel-runtime, and tool-catalog contracts with family-specific whole-value invariants and deterministic diagnostics.
- Make generated policy JSON a checked projection whose schema, contract, source, and output identities are freshness-bound to the Rust DTO boundary.
- Add positive and negative Nickel fixtures for each migrated family, including malformed enums, bounds, hashes, paths, duplicates, dangling references, and invalid cross-field combinations.
- Keep Rust responsible for runtime admission, effects, clocks, provider/tool execution, and receipts; do not add runtime Nickel evaluation.

## Impact

- **Authoring surfaces**: `policy/model-routing/`, `policy/process-jobs/`, `policy/steel-runtime/`, `policy/tool-catalog/`, and shared policy contracts.
- **Validation**: existing family checkers gain contract-evaluation and projection-parity coverage rather than relying primarily on text or post-export checks.
- **Compatibility**: valid current policies retain their runtime meaning; invalid or ambiguous source begins failing before export. Generated artifact changes require an explicit reviewed regeneration.
- **Active-change boundary**: this package does not change router selection semantics, tool execution semantics, host-control extraction, or receipt-authoritative completion. It hardens only their human-authored policy inputs.
- **Claims**: a passing Nickel contract proves authoring-time shape and declared invariants only, not runtime authorization, tool safety, provider truth, process confinement, or successful execution.
