# Tasks

## C. Core contract

- [ ] [serial] C1: Record current queue mutation entrypoints and the reviewed Prime Agent expected-text behavior. r[agent_harness_runtime.coding_session_queue_cas.verification]
- [ ] [serial] C2: Add stable entry identity, checked generation, private expected-text identity, command, outcome, and failure DTOs. r[agent_harness_runtime.coding_session_queue_cas.commands]
- [ ] [depends:C2] C3: Implement the pure compare-and-swap edit and removal reducer. r[agent_harness_runtime.coding_session_queue_cas.reducer]
- [ ] [depends:C3] C4: Return bounded conflict snapshots with no queue mutation. r[agent_harness_runtime.coding_session_queue_cas.conflicts]

## I. Integration

- [ ] [depends:C4] I1: Bind accepted transitions to durable session ordering and one generation advance. r[agent_harness_runtime.coding_session_queue_cas.reducer]
- [ ] [depends:I1] I2: Update headless, SDK, daemon, and TUI command DTOs to carry explicit preconditions. r[agent_harness_runtime.coding_session_queue_cas.commands]
- [ ] [depends:I2] I3: Add the restricted single-owner compatibility wrapper. r[agent_harness_runtime.coding_session_queue_cas.compatibility]
- [ ] [depends:I3] I4: Add safe events and metadata-only receipts for success and conflict. r[agent_harness_runtime.coding_session_queue_cas.evidence]

## T. Tests and closeout

- [ ] [depends:I4] T1: Add positive tests for exact edit, exact removal, generation advance, ordering, and frontend projection. r[agent_harness_runtime.coding_session_queue_cas.verification]
- [ ] [depends:I4] T2: Add negative tests for stale generation, wrong text, wrong entry, moved position, concurrent injection, overflow, duplicate command, unsafe conflict output, and last-writer-wins fallback. r[agent_harness_runtime.coding_session_queue_cas.verification]
- [ ] [depends:T2] T3: Update docs and component metadata, add a focused rail, run session and frontend checks, and close through Cairn gates. r[agent_harness_runtime.coding_session_queue_cas.verification]
