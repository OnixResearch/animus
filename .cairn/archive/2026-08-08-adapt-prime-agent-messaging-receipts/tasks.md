## Phase 1: Baseline and ownership

- [x] [serial] B1 Run and record current goose-message-queue-interrupts, orchestration adapter family, Iroh/Syndicate seams, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: focused agent-comms, orchestration, Syndicate, safe-receipt, no-std, and Wasm baselines passed before implementation.
- [x] [serial] B2 Inventory the exact prime-agent messaging cohort (revision, MIT license, `agent_message` skill, daemon routing docs) and map roster, modes, receipts, and limits onto Animus ownership. r[agent_harness_runtime.messaging_receipts.boundary]
  Evidence: `fixtures/messaging-receipts/prime-agent-c5991bc.md` pins revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`, MIT, and the bounded reviewed surface.
- [x] [serial] B3 Map ownership against `goose-message-queue-interrupts`, orchestration adapter family, Iroh/Syndicate seams, subagent facade, and the component catalog; record the seam where delivery decisions enter transports. r[agent_harness_runtime.messaging_receipts.boundary]
  Evidence: `docs/request-lifecycle.md`, `docs/safety-contract.md`, component catalog, and facade inventory keep classifier, transport, queue, and persistence ownership outside messaging.
- [x] [depends:agent_harness_runtime.messaging_receipts.profile] B4 Add a typed Nickel messaging policy for roster shape, admitted modes per role, size/rate/queue limits, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.messaging_receipts.profile]
  Evidence: `config/messaging-receipts-policy.ncl` and generated JSON pass Nickel parity and Rust policy identity revalidation.

## Phase 2: Roster and modes

- [x] [depends:agent_harness_runtime.messaging_receipts.roster] R1 Add the focused no-std messaging contract module with family roster DTOs, host-derived sender identity, and fail-closed target resolution against the admitted roster. r[agent_harness_runtime.messaging_receipts.roster]
  Evidence: `crates/animus-adapters/src/messaging.rs` implements `FamilyRoster`, host sender binding, and typed target failures.
- [x] [depends:agent_harness_runtime.messaging_receipts.modes] R2 Implement pure routing decisions for `auto`, `steer`, and `follow_up` over supplied target state; wire mode as a typed input to the interrupt classifier. r[agent_harness_runtime.messaging_receipts.modes]
  Evidence: `route_message` covers idle delivery, busy steering, explicit steering, and follow-up queue decisions without classifier or transport authority.
- [x] [depends:agent_harness_runtime.messaging_receipts.roster] R3 Implement family-scoped broadcast resolution and reject cross-family or unknown targets before routing. r[agent_harness_runtime.messaging_receipts.roster]
  Evidence: roster identity and broadcast tests pass, including unknown, cross-family, and stale-roster negatives.

## Phase 3: Receipts and limits

- [x] [depends:agent_harness_runtime.messaging_receipts.receipts] M1 Implement the two-variant receipt type (`delivered`, `queued`) with host attestation requirements and typed failure variants. r[agent_harness_runtime.messaging_receipts.receipts]
  Evidence: receipt tests require matching idle-entry or pending-storage attestations and reject missing or mismatched attestations.
- [x] [depends:agent_harness_runtime.messaging_receipts.limits] M2 Implement pure size, rate, and pending-queue limit checks over supplied counters that run before routing and return typed limit receipts. r[agent_harness_runtime.messaging_receipts.limits]
  Evidence: limit tests cover admitted boundaries and size, rate, and pending rejection before target routing.

## Phase 4: Evidence

- [x] [depends:agent_harness_runtime.messaging_receipts.evidence] E1 Implement metadata-only messaging receipts binding policy identity, roster identity, mode, receipt variant, counter identities, and domain-separated BLAKE3 identities; reject receipts containing message bodies. r[agent_harness_runtime.messaging_receipts.evidence]
  Evidence: `project_messaging_receipt` emits metadata and BLAKE3 identities only; body, credential, and claim-boundary negatives pass.

## Phase 5: Positive and negative conformance

- [x] [parallel] T1 Add positive fixtures for all three modes on busy and idle targets, `delivered` and `queued` receipts, roster-bound broadcast, and admitted limit edges. r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: messaging unit tests and focused Nix rail pass all positive routing, receipt, broadcast, and boundary cases.
- [x] [parallel] T2 Add negative fixtures for spoofed sender identity, unknown or cross-family targets, size/rate/queue violations, and ambient transport or delivery attempts. r[agent_harness_runtime.messaging_receipts.roster] r[agent_harness_runtime.messaging_receipts.limits]
  Evidence: fail-closed identity, roster, mode, limit, and source-authority tests pass.
- [x] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing message bodies, read/acted overclaims, exactly-once claims, and authority widening. r[agent_harness_runtime.messaging_receipts.evidence] r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: metadata-only receipt tests and focused source-shape guard reject content and overclaims.
- [x] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no transport, network, process, or persistence authority from the messaging module. r[agent_harness_runtime.messaging_receipts.boundary] r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: focused rail, component catalog, facade inventory, and Wasm check pass with no ambient authority markers.

## Phase 6: Documentation and closeout

- [x] [depends:agent_harness_runtime.messaging_receipts.evidence] D1 Document the roster contract, three modes, two receipt variants, limit classes, interrupt-classifier seam, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.messaging_receipts.boundary]
  Evidence: messaging docs and generated catalog/facade outputs are current.
- [x] [depends:agent_harness_runtime.messaging_receipts.verification] D2 Add `scripts/cairn/check-messaging-receipts` and a Nix check covering Nickel drift, fixture parity, routing determinism, receipt integrity, limit enforcement, safe receipts, and dependency isolation. r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: focused script and `checks.x86_64-linux.messaging-receipts` pass.
- [x] [depends:agent_harness_runtime.messaging_receipts.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: focused tests, Clippy, Wasm, Nickel, component, facade, and Nix checks pass; the drain-evidence guard remains blocked because `scripts/check-drain-evidence-guard.rs` is absent.
- [x] [depends:agent_harness_runtime.messaging_receipts.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening delivery or authority claims. r[agent_harness_runtime.messaging_receipts.verification]
  Evidence: requirements are present in `specs/agent-harness-runtime/spec.md`; archive remains gated on final Cairn validation and the documented missing drain-evidence guard blocker.
