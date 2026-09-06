## Phase 1: Implementation

- [x] [serial] Retain exact bounded output and failure facts in a private opt-in capture. r[agent_harness_runtime.check_capture.retention]
- [x] [serial] Preserve legacy observations, admission, hashes, and metadata-only receipts. r[agent_harness_runtime.check_capture.compatibility]
- [x] [serial] Add explicit cancellation with pre-spawn rejection and real-child controls. r[agent_harness_runtime.check_capture.cancellation]

## Evidence

`docs/selected-check-capture-evidence.md` records 15 package tests, one compile-fail
control, strict Clippy, the focused fixture rail, and full clean Octet evidence.
The broader authoritative-completion gate rejects its existing positive fixture
with `verification_contract.policy_drift`. The final task remains open until the
current policy and completion cohorts align and that gate passes. No archive,
main publication, or downstream runtime admission is authorized by this evidence.

## Phase 2: Verification

- [ ] [serial] Record focused tests, Clippy, source-bound rail, lifecycle evidence, and non-claims before sync and archive. r[agent_harness_runtime.check_capture.retention] r[agent_harness_runtime.check_capture.compatibility] r[agent_harness_runtime.check_capture.cancellation]
