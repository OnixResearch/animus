# Background work queues

Animus treats long-running work as a host-owned queue service. Reusable crates define typed requests, responses, bounds, and failure classes; hosts decide whether the concrete backend is pueue, another scheduler, or no queue at all.

## Contract

- Runtime code sends `QueueServiceRequest` values to an injected `BackgroundQueueService`.
- Enqueue requests carry safe command identity labels, group labels, environment policy labels, timeout policy labels, and output bounds. They do not carry raw shell text as the durable contract.
- Queue controls that can alter work require explicit selectors: task, named group, explicit default group, or all groups.
- Group status reports use stable task status counts plus bounded backend labels.
- Logs are exposed as bounded viewports with visible counts, total counts, truncation state, safe references, and optional BLAKE3 full-output hashes.
- Wait, stream, poll, and send-input are explicit host-advertised capabilities. Unsupported send-input fails closed and never falls back to shelling out to a backend CLI.

## Non-claims

This contract does not make pueue mandatory, does not grant reusable runtime crates process-spawn authority, and does not embed full logs or raw task input in model-visible receipts.

## Verification

Focused verification lives in `crates/animus-adapters/src/queue.rs` tests. Run:

```sh
nix develop -c cargo test -p animus-adapters --locked queue
```

Closeout evidence should also name Cairn validation and gates, and should record any broader workspace check blocked by unrelated repository debt.
