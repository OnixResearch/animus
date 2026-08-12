# Proposal: Negotiate router capability eligibility

## Why

`select_entries` decides whether a catalog entry may serve a recipe role via `entry_allowed`, which today checks only `role == target`, `budget.allows_backend`, and `privacy`-vs-`local`. That is too coarse for collaborative routing: a model may be local and on the right backend but lack the role's required output contract (e.g. a text-only model assigned the `Synthesizer` role for a `Json` output), or may not support the tool-call contract a `Worker` role needs. Meanwhile the accepted `model_capability_matrix.negotiation` requirement already defines pure capability negotiation over canonical request requirements. Animus should connect router role eligibility to that capability matrix so unsupported required features fail closed instead of silently dispatching.

## What Changes

- Add a pure `resolve_role_eligibility(role: Role, output_contract: OutputContract, entry: &CatalogEntry, capability: &ModelCapabilityMatrix) -> EligibilityDecision` returning `Admitted`, `Denied(reason)`, or `Downgraded(to)`.
- Extend `CatalogEntry` (or its projection) with a capability-matrix identity so the router can look up negotiated support for the role's required output contract, tool-call behavior, structured-output mode, privacy posture, and budget class.
- Wire `select_entries` to call `resolve_role_eligibility` so a role with no eligible model returns `SelectionDenial::NoEligibleModel` and a policy-authorized downgrade is recorded in the selected plan.
- Add positive tests for fully supported roles and authorized downgrades; negative tests for unsupported output contract, missing tool-call support, privacy mismatch, missing capability metadata, and denied fallback.

## Impact

- **Files**: `crates/animus-router` eligibility module, `CatalogEntry`/catalog projection, router selection wiring, router tests, facade inventory, and Cairn evidence.
- **Testing**: positive eligibility and downgrade fixtures; negative tests for unsupported contract, missing tool support, privacy mismatch, unknown capability, and denied fallback.
- **Non-claims**: this does not add live provider calls, capability discovery, price scraping, credentials, or new recipe families; it consumes the already-accepted capability matrix.
