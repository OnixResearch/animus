# Tasks

- [x] [serial] Define queued message, queue operation, queue state, acknowledgement, interrupt signal, redirection, cancellation, and control event DTOs. r[agent_harness_runtime.goose_message_queue.dtos]
- [x] [depends:dtos] Add pure queue reducer operations for append, edit, reorder, delete, clear, dispatch-next, acknowledgement, and cancel-pending behavior. r[agent_harness_runtime.goose_message_queue.reducer]
- [x] [depends:dtos] Add deterministic interrupt classification and explicit send-mode handling for pause, cancel, redirect, queue, and immediate-dispatch intents. r[agent_harness_runtime.goose_message_queue.interrupts]
- [x] [depends:reducer,interrupts] Add policy, budget, active-work, stale-ID, max-queue, unsafe-body, and missing-cancellation-handle denial paths. r[agent_harness_runtime.goose_message_queue.fail_closed]
- [x] [depends:fail_closed] Add safe queue and interrupt receipts plus leak checks for raw prompts, provider payloads, credentials, full paths, raw tool output, and unbounded queue bodies. r[agent_harness_runtime.goose_message_queue.receipts]
- [x] [depends:receipts] Update docs and run focused queue reducer, interrupt, fail-closed, receipt leak, Cairn validation, and Cairn gate checks with reviewable evidence. r[agent_harness_runtime.goose_message_queue.verification]
