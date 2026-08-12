## Phase 1: Invariant and repair contract

- [x] [serial] Record the Tau behavior/provenance reference, inventory Animus canonical message, tool-call/result, session-tree, persistence, cancellation, and compaction owners, and identify the exact active-branch integration points without copying Tau source. r[agent_harness_runtime.tau_tool_repair.invariant] r[agent_harness_runtime.tau_tool_repair.verification]
- [x] [serial] Define validated active-branch entry, tool-call binding, tool-result binding, interruption evidence, diagnostic, repair action, plan, application outcome, and error DTOs with named bounds. r[agent_harness_runtime.tau_tool_repair.invariant] r[agent_harness_runtime.tau_tool_repair.plan]
- [x] [depends:agent_harness_runtime.tau_tool_repair.plan] Define evidence-bound reason reduction for user cancellation, host cancellation, unclean shutdown, and unknown interruption. r[agent_harness_runtime.tau_tool_repair.reason]

## Phase 2: Functional core and host seam

- [x] [depends:agent_harness_runtime.tau_tool_repair.invariant] Implement pure active-path diagnostics that match each call to one compatible result and reject duplicate IDs, duplicate results, result-before-call order, tool mismatch, unsupported entries, and stale leaves. r[agent_harness_runtime.tau_tool_repair.invariant]
- [x] [depends:agent_harness_runtime.tau_tool_repair.reason] Implement deterministic repair planning for tail gaps, partially completed tool batches, and historical gaps with ordered BLAKE3 plan/action identities. r[agent_harness_runtime.tau_tool_repair.plan]
- [x] [depends:agent_harness_runtime.tau_tool_repair.plan] Define append-only tail and historical-branch application requests that preserve old entries, matched results, and active-leaf provenance without raw-content duplication in evidence. r[agent_harness_runtime.tau_tool_repair.append_only]
- [x] [depends:agent_harness_runtime.tau_tool_repair.append_only] Wire the host boundary so provider continuation requires durable repair application plus replay revalidation, and prove no repair path constructs or invokes a tool executor. r[agent_harness_runtime.tau_tool_repair.provider_gate]
- [x] [depends:agent_harness_runtime.tau_tool_repair.plan] Implement metadata-only repair receipt construction and leak validation. r[agent_harness_runtime.tau_tool_repair.receipts]

## Phase 3: Positive and negative evidence

- [x] [parallel] Add positive tests for a clean branch, one tail gap, multiple missing calls, a partially completed batch, explicit user cancellation, host interruption, unclean shutdown, unknown interruption, historical suffix repair, repeated-load idempotence, and stable plan/receipt hashes. r[agent_harness_runtime.tau_tool_repair.verification]
- [x] [parallel] Add negative tests for duplicate call IDs, duplicate results, result-before-call order, mismatched tool identity, stale leaf, missing repair anchor, unsupported entries, partial durable application, store failure, attempted tool re-execution, false user-cancellation attribution, and raw prompt/argument/output/path/provider leakage. r[agent_harness_runtime.tau_tool_repair.verification]
- [x] [depends:agent_harness_runtime.tau_tool_repair.provider_gate] Add composition tests proving repaired context is accepted by canonical provider projection while unrepaired or partially repaired context is rejected before provider contact. r[agent_harness_runtime.tau_tool_repair.provider_gate] r[agent_harness_runtime.tau_tool_repair.verification]

## Phase 4: Documentation and closeout

- [x] [serial] Document repair semantics, cause non-claims, append-only branch behavior, idempotence, provider gate, effect non-reexecution, safe receipts, and the bounded Tau reference in README and contract indexes. r[agent_harness_runtime.tau_tool_repair.receipts] r[agent_harness_runtime.tau_tool_repair.verification]
- [x] [serial] Run focused runtime/session/compaction/provider-projection tests, Clippy for touched crates, neutral-ledger and safe-receipt rails, composition conformance, Cairn validation, and proposal/design/tasks gates; record unavailable checks with blockers and next-best evidence. r[agent_harness_runtime.tau_tool_repair.verification]
