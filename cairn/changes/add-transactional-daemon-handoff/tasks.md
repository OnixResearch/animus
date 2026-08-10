## Phase 1: Baseline and boundary inventory

- [ ] [serial] B1 Run daemon-worker, session-restore, active-turn recovery, pending-input, host-control, and agent-control baselines. r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [serial] B2 Inventory worker lease, queue, child, confirmation, repair, cancellation, draft, restart, and process ownership boundaries. r[agent_harness_runtime.daemon_handoff.idle]
- [ ] [parallel] B3 Record the pinned Autolith revision, ISC license, selected handoff scenarios, rejected Bash supervisor, and no-runtime-dependency boundary. r[agent_harness_runtime.daemon_handoff.evidence]

## Phase 2: Pure transaction core

- [ ] [serial] C1 Add nominal handoff, worker, lease-generation, claimant, endpoint, process-attempt, readiness, and cancellation identities. r[agent_harness_runtime.daemon_handoff.transaction]
- [ ] [serial] [depends:C1] C2 Add strict-idle validation over active effects, queues, children, commits, repairs, confirmations, cancellation, and terminal state. r[agent_harness_runtime.daemon_handoff.idle]
- [ ] [serial] [depends:C2] C3 Add monotonic prepared-through-complete and cancellation-through-restored transitions. r[agent_harness_runtime.daemon_handoff.transaction]
- [ ] [serial] [depends:C3] C4 Add single-claimant admission with exact session, lease, endpoint, process, protocol, and freshness binding. r[agent_harness_runtime.daemon_handoff.claim]
- [ ] [serial] [depends:C3] C5 Add cancellation planning that blocks lease restoration until complete replacement-tree termination evidence. r[agent_harness_runtime.daemon_handoff.cancellation]

## Phase 3: Adapter and host shell

- [ ] [serial] [depends:C3] [depends:C4] [depends:C5] H1 Extend `animus-adapter-daemon` with versioned handoff DTOs and implementation-free lease, process, endpoint, and readiness ports. r[agent_harness_runtime.daemon_handoff.transaction] r[agent_harness_runtime.daemon_handoff.claim]
- [ ] [serial] [depends:H1] H2 Add a non-default host shell for ingress pause, durable handoff commit, lease release, replacement spawn, local authentication, and readiness waits. r[agent_harness_runtime.daemon_handoff.transaction] r[agent_harness_runtime.daemon_handoff.claim]
- [ ] [serial] [depends:H2] H3 Persist composer draft identity, pending-input vault generation, session head, active-turn generation, and authority profile before lease release. r[agent_harness_runtime.daemon_handoff.session_state]
- [ ] [serial] [depends:H2] H4 Add owned process-tree cancellation and terminal observation before rollback lease acquisition. r[agent_harness_runtime.daemon_handoff.cancellation]
- [ ] [serial] [depends:H3] [depends:H4] H5 Compose replacement readiness with durable session load, vault visibility, authority admission, and active-turn recovery. r[agent_harness_runtime.daemon_handoff.session_state]
- [ ] [serial] [depends:H5] H6 Add transport-neutral status and wait mappings only where Host Control already owns the semantic shape. r[agent_harness_runtime.daemon_handoff.claim]

## Phase 4: Adversarial handoff evidence

- [ ] [parallel] [depends:H5] [depends:H6] T1 Add positive tests for strict idle, lease release, authenticated claim, replacement readiness, old-worker exit, and successful completion. r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [parallel] [depends:H5] T2 Add busy provider, active tool, non-empty queue, child work, pending commit, unresolved confirmation, repair, and cancellation denials. r[agent_harness_runtime.daemon_handoff.idle] r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [parallel] [depends:H2] [depends:H5] T3 Add stale generation, duplicate claimant, wrong endpoint, wrong process, wrong protocol, cancelled handoff, and late-claim fixtures. r[agent_harness_runtime.daemon_handoff.claim] r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [parallel] [depends:H4] T4 Add spawn failure, readiness timeout, partial teardown, surviving descendant, rollback drift, and second-owner denial fixtures. r[agent_harness_runtime.daemon_handoff.cancellation] r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [parallel] [depends:H3] T5 Add receipt leak and overclaim tests for drafts, pending input, paths, process arguments, credentials, PTY transfer, sandboxing, and migration claims. r[agent_harness_runtime.daemon_handoff.evidence] r[agent_harness_runtime.daemon_handoff.verification]

## Phase 5: Smoke, downstream contract, and closeout

- [ ] [serial] [depends:T1] [depends:T2] [depends:T3] [depends:T4] [depends:T5] D1 Add a non-default local fake-backed daemon handoff smoke with bounded deadlines and no remote transport. r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [serial] [depends:D1] D2 Document strict idle, transition order, rollback, local authentication evidence, Tile ownership, and non-claims. r[agent_harness_runtime.daemon_handoff.transaction] r[agent_harness_runtime.daemon_handoff.evidence]
- [ ] [serial] [depends:D2] D3 Record a future Tile-owned integration change without editing Tile during this Cairn. r[agent_harness_runtime.daemon_handoff.session_state]
- [ ] [serial] [depends:D3] D4 Update daemon, restore, session, request-lifecycle, dogfood-readiness, component, facade, roadmap, and README indexes. r[agent_harness_runtime.daemon_handoff.transaction]
- [ ] [serial] [depends:D4] D5 Add focused Cairn and Nix rails for state, process-tree, claim, cancellation, isolation, and receipt fixtures. r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [serial] [depends:D5] D6 Run focused tests, formatting, Clippy, no-std and dependency isolation, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.daemon_handoff.verification]
- [ ] [serial] [depends:D6] D7 Review dry-run sync and archive plans, validate accepted requirement promotion, and archive only with current evidence. r[agent_harness_runtime.daemon_handoff.verification]
