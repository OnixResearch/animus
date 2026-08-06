## Phase 1: Baseline and ownership

- [x] [serial] B1 Run and record the current session persistence, interrupted tool repair, provider retry, headless steering, authority-profile, event, and completion baselines. r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [serial] B2 Inventory every current acceptance, ledger commit, steering, provider dispatch, repair, terminal, and cleanup boundary. r[agent_harness_runtime.active_turn_recovery.boundary]
- [x] [parallel] B3 Record the exact camelAI reference revision, adopt and reject decisions, MIT provenance, and no-runtime-dependency boundary. r[agent_harness_runtime.active_turn_recovery.boundary] r[agent_harness_runtime.active_turn_recovery.evidence]

## Phase 2: Policy and pure core

- [x] [serial] P1 Add typed Nickel policy for schema, journal entry, aggregate byte, generation, steering, retention, replacement, corruption, terminal, and receipt limits. r[agent_harness_runtime.active_turn_recovery.policy]
- [x] [serial] P2 Export generated policy data and add strict Rust revalidation with domain-separated BLAKE3 policy identity. r[agent_harness_runtime.active_turn_recovery.policy]
- [x] [serial] C1 Add pure validated active-turn, accepted-message, journal-generation, checkpoint, steering, terminal, recovery-plan, and diagnostic types. r[agent_harness_runtime.active_turn_recovery.boundary] r[agent_harness_runtime.active_turn_recovery.identity]
- [x] [serial] C2 Add canonical BLAKE3 turn, checkpoint, steering, plan, and receipt identities over typed fields and explicit monotonic sequences. r[agent_harness_runtime.active_turn_recovery.identity]
- [x] [serial] C3 Add the pure recovery state machine for acceptance, checkpoint replacement, replay, repair delegation, continuation, terminal observation, and cleanup planning. r[agent_harness_runtime.active_turn_recovery.journal] r[agent_harness_runtime.active_turn_recovery.recovery]

## Phase 3: Host store and composition

- [x] [serial] H1 Add implementation-free durable acceptance, compare-and-swap checkpoint transaction, steering append, terminal commit, and cleanup service ports. r[agent_harness_runtime.active_turn_recovery.journal]
- [x] [serial] H2 Add a non-default std host adapter that prepares all bytes before mutation and returns typed transaction evidence. r[agent_harness_runtime.active_turn_recovery.journal] r[agent_harness_runtime.active_turn_recovery.evidence]
- [x] [serial] H3 Gate provider dispatch on durable accepted-work evidence and gate steering delivery on the exact durable append observation. r[agent_harness_runtime.active_turn_recovery.journal] r[agent_harness_runtime.active_turn_recovery.recovery]
- [x] [serial] H4 Compose exact active-turn recovery with session authority, interrupted tool repair, cancellation, provider retry, and terminal completion without constructing tool reruns. r[agent_harness_runtime.active_turn_recovery.recovery]
- [x] [serial] H5 Keep cleanup after the durable terminal commit and retain explicit cleanup-required state after failed cleanup. r[agent_harness_runtime.active_turn_recovery.recovery]

## Phase 4: Evidence and adversarial tests

- [x] [parallel] T1 Add positive tests for accepted-message recovery, complete checkpoint replacement, preserved real tool results, one synthetic interrupted result, steering recovery, and terminal-before-cleanup recovery. r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [parallel] T2 Add negative tests for pre-acceptance dispatch, partial preparation, partial transaction evidence, stale generation, duplicate sequence, identity mismatch, over-budget state, and conflicting terminal facts. r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [parallel] T3 Add corruption tests for malformed, missing, unknown, reordered, duplicated, truncated, and mixed-generation records. r[agent_harness_runtime.active_turn_recovery.recovery] r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [parallel] T4 Add no-rerun tests that fail if recovery constructs a tool request, calls an executor, replaces a real result, or treats provider retry as tool retry. r[agent_harness_runtime.active_turn_recovery.recovery] r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [parallel] T5 Add receipt leak and overclaim tests for transcript text, tool bodies, provider data, credentials, paths, raw errors, timestamp identity, durability, exactly-once, completion, and production claims. r[agent_harness_runtime.active_turn_recovery.evidence] r[agent_harness_runtime.active_turn_recovery.verification]

## Phase 5: Documentation and closeout

- [x] [serial] D1 Document the recovery state machine, crash-window matrix, host transaction contract, integration order, operator cleanup, and non-claims. r[agent_harness_runtime.active_turn_recovery.boundary] r[agent_harness_runtime.active_turn_recovery.evidence]
- [x] [serial] D2 Update contracts, component, facade, request-lifecycle, session, repair, retry, headless, safety, roadmap, and README indexes. r[agent_harness_runtime.active_turn_recovery.boundary]
- [x] [serial] D3 Add a focused recovery rail and Nix check for Nickel drift, pure core behavior, host fakes, crash windows, dependency isolation, safe receipts, and composition. r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [serial] D4 Run focused tests, formatting, Clippy, Wasm and no-std checks where applicable, Cairn validation and gates, and the relevant Nix checks. r[agent_harness_runtime.active_turn_recovery.verification]
- [x] [serial] D5 Review dry-run sync and archive plans, make sure that every requirement reaches the accepted spec, and archive only after current evidence passes. r[agent_harness_runtime.active_turn_recovery.verification]
