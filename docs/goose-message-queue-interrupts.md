# Goose message queue and interrupt controls

Animus models Goose-style follow-up queues and interrupt controls as pure runtime data. Hosts own UI controls, persistence, transport cancellation, provider abort APIs, and model dispatch; `animus-runtime` owns deterministic queue state transitions, interrupt classification, and safe receipts.

## Runtime boundary

Use `animus_runtime::apply_message_queue_operation` to reduce an in-memory `MessageQueueState` with a `MessageQueueOperation` under a `MessageQueuePolicy`. The reducer supports append, edit, reorder, delete, clear, dispatch-next, dispatch acknowledgement, and cancel-pending operations. It rejects stale active-work IDs, duplicate queue item IDs, unauthorized source labels, max-queue overflow, unsafe bodies, over-budget bodies, policy denial, unknown items, mismatched acknowledgements, and duplicate pending dispatch.

Use `animus_runtime::classify_interrupt_signal` to classify `InterruptSignalDraft` values into explicit `UserControlEvent` variants. The classifier handles policy-owned keyword classes and explicit send modes for queue, immediate dispatch, pause, cancel, and redirect. Cancel events require an available host cancellation handle; missing or unsupported handles fail closed without claiming work was aborted.

## Receipt safety

`MessageQueueReceipt` stores operation kind, source label, active-work hash, queue-item hash, decision class, cancellation handle class, budget class, policy hash, optional denial class, redaction state, and a deterministic BLAKE3 receipt hash. Receipt validation rejects raw prompts, provider payloads, credentials, full paths, raw tool or command output, and unbounded queue-body markers.

## Non-claims

This contract does not implement Desktop cards, terminal key handling, live provider abort APIs, storage-backed queue persistence, process cancellation, or UI rendering. Queue and interrupt events are plans for host shells to interpret after their own policy and capability checks.

## Focused checks

```sh
nix develop -c cargo test -p animus-runtime message_queue --locked
nix develop -c cargo clippy -p animus-runtime --lib --tests --locked --no-deps -- -D warnings
```

