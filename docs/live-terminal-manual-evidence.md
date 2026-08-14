# Live terminal manual evidence

## Current observation

Status: `NOT_RUN_NO_CONTROLLING_TERMINAL`

The automated drain environment does not provide a separately observed controlling terminal. Therefore, this change does not claim a successful manual live run.

The automated evidence covers pure reducers, Ratatui `TestBackend`, a local Unix protocol, direct and daemon dispatch, and a pseudo-terminal lifecycle. This evidence does not replace a human-observed live terminal run.

## Manual procedure

Use a reviewed local daemon and a disposable terminal. Set all required inputs explicitly:

```text
ANIMUS_TERMINAL_CONFIG_PATH=config/generated/live-terminal-agent-host.json
ANIMUS_TERMINAL_ENDPOINT_ROOT=<caller-owned-runtime-directory>
ANIMUS_TERMINAL_WORKER_GENERATION=<current-positive-generation>
ANIMUS_TERMINAL_MAX_FRAME_BYTES=<reviewed-positive-frame-limit>
```

Run `cargo run -p animus-live-terminal --features live-terminal --locked` from the repository root.

Check these facts:

1. The client uses only the configured Unix endpoint.
2. Input and resize update the display.
3. Prompt and control keys create expected worker commands.
4. Detach does not stop worker execution.
5. Normal exit restores raw mode, the alternate screen, and cursor state.
6. A supported signal restores terminal state.
7. The printed receipt contains metadata only.

Do not record prompt text, terminal buffers, credentials, provider content, tool content, or private command payloads.

## Non-claims

This record does not prove arbitrary terminal compatibility, crash-proof restoration, remote security, sandboxing, task correctness, production readiness, or release eligibility.
