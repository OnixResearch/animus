## Phase 1: Implementation

- [x] [serial] Retain exact bounded output and failure facts in a private opt-in capture. r[agent_harness_runtime.check_capture.retention]
- [x] [serial] Preserve legacy observations, admission, hashes, and metadata-only receipts. r[agent_harness_runtime.check_capture.compatibility]
- [x] [serial] Add explicit cancellation with pre-spawn rejection and real-child controls. r[agent_harness_runtime.check_capture.cancellation]

## Evidence

`docs/selected-check-capture-evidence.md` records 15 package tests, one compile-fail
control, strict Clippy, the focused fixture rail, and full clean Octet evidence.
Policy repair `92914e2` resolves the historical `verification_contract.policy_drift`
without a dependency change. Merged source `fdb5595` passed 20 package tests,
one compile-fail control, strict Clippy, the live completion gate, the source-bound
fixture rail, and full clean Octet. No downstream runtime admission follows from
this owner-contract qualification.

## Phase 2: Verification

- [x] [serial] Record focused tests, Clippy, source-bound rail, lifecycle evidence, and non-claims before sync and archive. r[agent_harness_runtime.check_capture.retention] r[agent_harness_runtime.check_capture.compatibility] r[agent_harness_runtime.check_capture.cancellation]
