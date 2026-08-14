# Live terminal agent host

The `animus-live-terminal` package supplies a non-default local terminal client. The package uses `AGPL-3.0-or-later` and stays outside workspace `default-members`.

## Reference and owners

The design adapts the reviewed Prime Agent interactive cohort at revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`. The source is MIT licensed. The review covered `packages/coding-agent/src/modes/interactive/interactive-mode.ts`.

Animus retains these accepted owners:

- `animus-tui` owns reusable view reducers and render projections;
- `animus-local-daemon` owns local transport, attachments, routing, and worker lifecycle;
- the worker or injected direct service owns provider, tool, Steel, queue, child, session, and persistence effects;
- `animus-live-terminal` owns terminal input, rendering, lifecycle, and typed client commands only.

The terminal crate does not copy Prime Agent source. It rejects the reference runtime's ambient discovery, process, provider, persistence, credential, extension, and download authority.

## Configuration and authority

`config/live-terminal-agent-host.ncl` is the typed source of truth. Rust revalidates its exact JSON export before terminal or connection effects.

Connection mode is explicit:

- `daemon` requires one caller-owned endpoint root and one configured Unix endpoint name;
- `direct` requires one injected `LocalSupervisorService`;
- neither mode discovers endpoints, falls back, or switches automatically.

The live binary requires these inputs:

- `ANIMUS_TERMINAL_CONFIG_PATH`;
- `ANIMUS_TERMINAL_ENDPOINT_ROOT`;
- `ANIMUS_TERMINAL_WORKER_GENERATION`;
- `ANIMUS_TERMINAL_MAX_FRAME_BYTES`.

The `live-terminal` feature enables Crossterm, Ratatui live rendering, and signal handling. Default builds do not gain live terminal authority.

## Lifecycle and event loop

`TerminalLifecycleGuard` records each completed startup transition. It restores completed transitions in reverse order. Restoration is idempotent. A cleanup failure blocks a successful receipt.

One loop iteration processes no more than:

- one terminal input;
- one configured event batch;
- one typed command;
- one render decision.

The host handles input, tick, resize, cancellation, supported signal, disconnect, error, and normal-exit observations. Queue conflicts request a fresh snapshot through a typed session command.

## Commands and rendering

Key handling creates typed prompt, queue compare-and-swap, approval, cancellation, child, refresh, detach, and stop descriptors. The terminal does not execute the effects.

Snapshot-first state uses these existing projections:

- `CodingSessionRenderModel`;
- `ConversationRenderModel`;
- `ToolRenderModel`;
- `PromptInputState`.

`DaemonRequestDispatcher` binds the terminal wire protocol to `LocalSupervisorService`. The supervisor routes each private payload unchanged and does not parse or retain it.

## Evidence

Pure reducer, `TestBackend`, local Unix transport, direct-service, daemon-dispatch, and pseudo-terminal tests are automated. The pseudo-terminal test checks raw input, alternate-screen sequences, cursor sequences, and exact termios restoration.

The manual live run is a separate evidence class. See [live terminal manual evidence](live-terminal-manual-evidence.md). Automated evidence does not claim a successful real controlling-terminal run.

Receipts contain BLAKE3 identities, bounded counts, viewport dimensions, connection and exit classes, complete cleanup facts, and fixed non-claims. Receipts omit prompts, queue text, provider data, tool data, Steel source, terminal input, screen buffers, credentials, paths, and private command payloads.

Run the focused rail:

```text
scripts/cairn/check-live-terminal-agent-host --root .
```

The rail does not establish arbitrary terminal compatibility, remote security, crash-proof restoration, task correctness, production readiness, or release eligibility.
