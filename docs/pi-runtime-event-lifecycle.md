# Pi runtime event lifecycle adaptation

Animus exposes Pi-inspired lifecycle facts as typed Rust DTOs in `animus-runtime`. The runtime owns event ordering and middleware contracts; hosts own plugin discovery, trust, UI, persistence, and transport.

## Event stream

`RuntimeEvent` is mode-neutral and can be consumed by direct SDK embedders, headless RPC streams, safe receipts, or future UI shells. Events carry typed requests/responses, safe labels, summaries, boundary classes, and terminal status classes. They do not carry terminal component state or require a TUI to interpret.

The current loop emits runtime start, context replay, compatibility loop start, turn start, loop decision, adapter request/response or denial, final response/stop/failure, and shutdown events in deterministic order. DTO variants also reserve cancellation and compaction boundaries for hosts that wire those surfaces.

## Middleware

Hosts inject middleware through `RuntimeMiddlewareStack`. Hooks run in configured order:

1. observe-only hooks may inspect events without mutating runtime state;
2. model/tool dispatch hooks may mutate requests;
3. hooks may block guarded boundaries with a typed denial;
4. hooks may replace a boundary payload before later hooks and the adapter see it.

Model and tool dispatch guards execute before adapters. Blocks and hook errors fail safe, emit typed hook and denial events, return `RuntimeFailure::Middleware`, and do not call the guarded adapter.

## Reuse contract

SDK and headless consumers should subscribe to `RunReport::events()` and use `RuntimeEvent::kind()`/`RuntimeEventKind::as_str()` for stable classification. Receipt builders can persist safe event labels and terminal status classes without embedding host UI state. Future RPC transports should stream the same lifecycle facts rather than translating them into terminal-specific events.
