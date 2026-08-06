## Phase 1: Baseline and ownership

- [ ] [serial] B1 Run and record current goose-message-queue-interrupts, orchestration adapter family, Iroh/Syndicate seams, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.messaging_receipts.verification]
- [ ] [serial] B2 Inventory the exact prime-agent messaging cohort (revision, MIT license, `agent_message` skill, daemon routing docs) and map roster, modes, receipts, and limits onto Animus ownership. r[agent_harness_runtime.messaging_receipts.boundary]
- [ ] [serial] B3 Map ownership against `goose-message-queue-interrupts`, orchestration adapter family, Iroh/Syndicate seams, subagent facade, and the component catalog; record the seam where delivery decisions enter transports. r[agent_harness_runtime.messaging_receipts.boundary]
- [ ] [depends:agent_harness_runtime.messaging_receipts.profile] B4 Add a typed Nickel messaging policy for roster shape, admitted modes per role, size/rate/queue limits, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.messaging_receipts.profile]

## Phase 2: Roster and modes

- [ ] [depends:agent_harness_runtime.messaging_receipts.roster] R1 Add the focused no-std messaging contract module with family roster DTOs, host-derived sender identity, and fail-closed target resolution against the admitted roster. r[agent_harness_runtime.messaging_receipts.roster]
- [ ] [depends:agent_harness_runtime.messaging_receipts.modes] R2 Implement pure routing decisions for `auto`, `steer`, and `follow_up` over supplied target state; wire mode as a typed input to the interrupt classifier. r[agent_harness_runtime.messaging_receipts.modes]
- [ ] [depends:agent_harness_runtime.messaging_receipts.roster] R3 Implement family-scoped broadcast resolution and reject cross-family or unknown targets before routing. r[agent_harness_runtime.messaging_receipts.roster]

## Phase 3: Receipts and limits

- [ ] [depends:agent_harness_runtime.messaging_receipts.receipts] M1 Implement the two-variant receipt type (`delivered`, `queued`) with host attestation requirements and typed failure variants. r[agent_harness_runtime.messaging_receipts.receipts]
- [ ] [depends:agent_harness_runtime.messaging_receipts.limits] M2 Implement pure size, rate, and pending-queue limit checks over supplied counters that run before routing and return typed limit receipts. r[agent_harness_runtime.messaging_receipts.limits]

## Phase 4: Evidence

- [ ] [depends:agent_harness_runtime.messaging_receipts.evidence] E1 Implement metadata-only messaging receipts binding policy identity, roster identity, mode, receipt variant, counter identities, and domain-separated BLAKE3 identities; reject receipts containing message bodies. r[agent_harness_runtime.messaging_receipts.evidence]

## Phase 5: Positive and negative conformance

- [ ] [parallel] T1 Add positive fixtures for all three modes on busy and idle targets, `delivered` and `queued` receipts, roster-bound broadcast, and admitted limit edges. r[agent_harness_runtime.messaging_receipts.verification]
- [ ] [parallel] T2 Add negative fixtures for spoofed sender identity, unknown or cross-family targets, size/rate/queue violations, and ambient transport or delivery attempts. r[agent_harness_runtime.messaging_receipts.roster] r[agent_harness_runtime.messaging_receipts.limits]
- [ ] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing message bodies, read/acted overclaims, exactly-once claims, and authority widening. r[agent_harness_runtime.messaging_receipts.evidence] r[agent_harness_runtime.messaging_receipts.verification]
- [ ] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no transport, network, process, or persistence authority from the messaging module. r[agent_harness_runtime.messaging_receipts.boundary] r[agent_harness_runtime.messaging_receipts.verification]

## Phase 6: Documentation and closeout

- [ ] [depends:agent_harness_runtime.messaging_receipts.evidence] D1 Document the roster contract, three modes, two receipt variants, limit classes, interrupt-classifier seam, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.messaging_receipts.boundary]
- [ ] [depends:agent_harness_runtime.messaging_receipts.verification] D2 Add `scripts/cairn/check-messaging-receipts` and a Nix check covering Nickel drift, fixture parity, routing determinism, receipt integrity, limit enforcement, safe receipts, and dependency isolation. r[agent_harness_runtime.messaging_receipts.verification]
- [ ] [depends:agent_harness_runtime.messaging_receipts.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.messaging_receipts.verification]
- [ ] [depends:agent_harness_runtime.messaging_receipts.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening delivery or authority claims. r[agent_harness_runtime.messaging_receipts.verification]
