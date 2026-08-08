## Phase 1: Baseline and ownership

- [x] [serial] B1 Run and record current host-control-protocol, session-restore-contract, std-control, background-work-queues, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.daemon_workers.verification]
- [x] [serial] B2 Inventory the exact prime-agent daemon cohort (revision, MIT license, architecture, daemon, and agent-connection docs) and map client, supervisor, and worker role authority onto Animus ownership. r[agent_harness_runtime.daemon_workers.boundary]
- [x] [serial] B3 Map ownership against `host-control-protocol`, `session-restore-contract`, `animus-std-control`, `background-work-queues`, messaging receipts, and the component catalog; record the seams where clients attach and workers report. r[agent_harness_runtime.daemon_workers.boundary]
- [x] [depends:agent_harness_runtime.daemon_workers.profile] B4 Add a typed Nickel daemon policy for role authority sets, attachment limits, recovery rules, versioned envelope identities, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.daemon_workers.profile]

## Phase 2: Role ownership and supervisor

- [x] [depends:agent_harness_runtime.daemon_workers.ownership] W1 Add the focused no-std daemon contract module with the three role types, their authority sets, and versioned command and event envelopes; add compatibility re-exports without changing default authority. r[agent_harness_runtime.daemon_workers.ownership]
- [x] [depends:agent_harness_runtime.daemon_workers.ownership] W2 Enforce single-tree worker ownership: one root session, its queue, schedules, kernel handles, and descendants per worker; reject cross-worker tree access. r[agent_harness_runtime.daemon_workers.ownership]
- [x] [depends:agent_harness_runtime.daemon_workers.supervisor] W3 Implement supervisor decision contracts for discovery, routing, attachment management, worker health classification, and catalog scans as pure decisions over supplied state. r[agent_harness_runtime.daemon_workers.supervisor]

## Phase 3: Detach, reattach, and containment

- [x] [depends:agent_harness_runtime.daemon_workers.detach] D1 Implement attach-as-subscription and detach-as-unsubscribe semantics; prove execution decisions never depend on attachment state. r[agent_harness_runtime.daemon_workers.detach]
- [x] [depends:agent_harness_runtime.daemon_workers.detach] D2 Implement reattach admission for live streams and recovery snapshots with freshness typing from the session-restore contract. r[agent_harness_runtime.daemon_workers.detach]
- [x] [depends:agent_harness_runtime.daemon_workers.containment] D3 Implement the containment contract listing lifecycle-provided properties and add receipt validation that rejects sandbox, confinement, or security claims. r[agent_harness_runtime.daemon_workers.containment]

## Phase 4: Evidence

- [x] [depends:agent_harness_runtime.daemon_workers.evidence] E1 Implement metadata-only daemon receipts binding policy identity, role, envelope identity, attachment class, recovery class, health class, and domain-separated BLAKE3 identities; reject receipts containing transcript content or sandbox claims. r[agent_harness_runtime.daemon_workers.evidence]

## Phase 5: Positive and negative conformance

- [x] [parallel] T1 Add positive fixtures for role authority, attach/detach/reattach cycles, snapshot recovery, supervisor routing, and worker failure containment to one session tree. r[agent_harness_runtime.daemon_workers.verification]
- [x] [parallel] T2 Add negative fixtures for client-side execution authority, supervisor session execution, cross-worker tree access, orphan sessions, and stale snapshot reattach. r[agent_harness_runtime.daemon_workers.ownership] r[agent_harness_runtime.daemon_workers.detach]
- [x] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing transcript content, sandbox or security claims, crash-freedom claims, and authority widening. r[agent_harness_runtime.daemon_workers.evidence] r[agent_harness_runtime.daemon_workers.verification]
- [x] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no process, socket, filesystem, or persistence authority from the daemon module. r[agent_harness_runtime.daemon_workers.boundary] r[agent_harness_runtime.daemon_workers.verification]

## Phase 6: Documentation and closeout

- [x] [depends:agent_harness_runtime.daemon_workers.evidence] D1 Document the three roles, authority sets, attach/detach semantics, recovery rules, containment boundary, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.daemon_workers.boundary]
- [x] [depends:agent_harness_runtime.daemon_workers.verification] D2 Add `scripts/cairn/check-daemon-workers` and a Nix check covering Nickel drift, fixture parity, role conformance, recovery determinism, containment non-claims, safe receipts, and dependency isolation. r[agent_harness_runtime.daemon_workers.verification]
- [x] [depends:agent_harness_runtime.daemon_workers.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.daemon_workers.verification]
- [x] [depends:agent_harness_runtime.daemon_workers.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening containment or authority claims. r[agent_harness_runtime.daemon_workers.verification]
