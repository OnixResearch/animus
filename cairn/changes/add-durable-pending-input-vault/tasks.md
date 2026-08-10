## Phase 1: Baseline and contract inventory

- [ ] [serial] B1 Run the current coding-session, durable-store, message-queue, messaging-receipt, and active-turn recovery baselines. r[agent_harness_runtime.pending_input_vault.verification]
- [ ] [serial] B2 Inventory pending input classes, durable records, queue transitions, restart entrypoints, and existing cleanup behavior. r[agent_harness_runtime.pending_input_vault.boundary]
- [ ] [parallel] B3 Record the pinned Autolith revision, ISC license, adopted scenarios, rejected formats, and no-runtime-dependency boundary. r[agent_harness_runtime.pending_input_vault.evidence]

## Phase 2: Pure vault model

- [ ] [serial] C1 Add nominal conversation, vault, entry, disposition, and generation identities with domain-separated BLAKE3 framing. r[agent_harness_runtime.pending_input_vault.contents]
- [ ] [serial] [depends:C1] C2 Add validated active, steering, follow-up, command, and attachment-reference entry classes with exact ordering. r[agent_harness_runtime.pending_input_vault.contents]
- [ ] [serial] [depends:C2] C3 Add pure startup projection and review-state planning over supplied durable facts. r[agent_harness_runtime.pending_input_vault.boundary] r[agent_harness_runtime.pending_input_vault.recovery]
- [ ] [serial] [depends:C3] C4 Add pure restore, discard, repeated-request, stale-generation, and cross-conversation decisions. r[agent_harness_runtime.pending_input_vault.transactions]
- [ ] [serial] [depends:C3] C5 Add corruption classification that blocks ordinary ingress without removing bounded recovery controls. r[agent_harness_runtime.pending_input_vault.corruption]

## Phase 3: Host services and composition

- [ ] [serial] [depends:C4] [depends:C5] H1 Add implementation-free vault load, disposition commit, quarantine, cleanup, and body-access service ports. r[agent_harness_runtime.pending_input_vault.boundary] r[agent_harness_runtime.pending_input_vault.transactions]
- [ ] [serial] [depends:H1] H2 Extend the explicit durable session host with generation-fenced vault transactions and no global-store default. r[agent_harness_runtime.pending_input_vault.transactions]
- [ ] [serial] [depends:H2] H3 Compose startup recovery with active-turn recovery, coding-session queues, session authority, and messaging receipts. r[agent_harness_runtime.pending_input_vault.recovery]
- [ ] [serial] [depends:H3] H4 Require a new normal admission decision before any restored entry becomes transcript or provider input. r[agent_harness_runtime.pending_input_vault.recovery]
- [ ] [serial] [depends:H2] H5 Commit disposition evidence before vault deletion and preserve review state after failed cleanup. r[agent_harness_runtime.pending_input_vault.transactions] r[agent_harness_runtime.pending_input_vault.evidence]

## Phase 4: Adversarial evidence

- [ ] [parallel] [depends:H3] [depends:H4] [depends:H5] T1 Add positive restart tests for every pending-input class, exact ordering, selected restore, discard, and cleanup retry. r[agent_harness_runtime.pending_input_vault.verification]
- [ ] [parallel] [depends:H3] [depends:H4] [depends:H5] T2 Add negative tests for automatic submission, duplicate restore, stale generation, changed selection, cross-conversation access, and authority drift. r[agent_harness_runtime.pending_input_vault.verification]
- [ ] [parallel] [depends:H3] T3 Add malformed, missing, reordered, duplicated, mixed-generation, oversized, and uncertain-commit corruption fixtures. r[agent_harness_runtime.pending_input_vault.corruption] r[agent_harness_runtime.pending_input_vault.verification]
- [ ] [parallel] [depends:H5] T4 Add receipt leak and overclaim tests for input bodies, attachments, paths, provider data, raw errors, durability, and exactly-once claims. r[agent_harness_runtime.pending_input_vault.evidence] r[agent_harness_runtime.pending_input_vault.verification]

## Phase 5: Documentation and closeout

- [ ] [serial] [depends:T1] [depends:T2] [depends:T3] [depends:T4] D1 Document operator review, restore, discard, corruption, cleanup, ownership, and non-claims. r[agent_harness_runtime.pending_input_vault.boundary] r[agent_harness_runtime.pending_input_vault.evidence]
- [ ] [serial] [depends:D1] D2 Update request-lifecycle, session, recovery, component, facade, roadmap, and README indexes. r[agent_harness_runtime.pending_input_vault.boundary]
- [ ] [serial] [depends:D2] D3 Add a focused rail and Nix check for policy, core, host, restart, corruption, leak, and isolation fixtures. r[agent_harness_runtime.pending_input_vault.verification]
- [ ] [serial] [depends:D3] D4 Run focused tests, formatting, Clippy, no-std and Wasm checks where applicable, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.pending_input_vault.verification]
- [ ] [serial] [depends:D4] D5 Review dry-run sync and archive plans, validate accepted requirement promotion, and archive only with current evidence. r[agent_harness_runtime.pending_input_vault.verification]
