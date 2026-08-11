# Packaged TUI dogfood smoke

The completed TUI evidence rung is **packaged deterministic TestBackend TUI**. This rung composes the accepted headless smoke with a real Ratatui render pipeline.

The non-default `animus-tui-dogfood-smoke` binary uses a fixed viewport. It renders safe `AppShellState` metadata and emits only bounded receipt fields.

Run the smoke from the repository root:

```sh
cargo run -p animus-tui --features packaged-tui-dogfood-smoke \
  --bin animus-tui-dogfood-smoke --locked -- \
  --prompt "bounded TUI prompt fixture" \
  --tool fake-readonly-tool
```

The receipt contains the TestBackend rung, upstream headless rung, backend label, event count, viewport, cleanup status, BLAKE3 identities, and non-claims.

The receipt omits the prompt, model output, tool output, headless receipt body, rendered buffer, provider payloads, credentials, and private paths.

Run the focused checks:

```sh
scripts/cairn/check-packaged-tui-dogfood-smoke --root .
nix build .#checks.x86_64-linux.packaged-tui-dogfood-smoke --no-link -L
```

This evidence does not prove a live terminal. It does not prove raw mode, input polling, terminal restoration, daemon attach, production readiness, or release eligibility.
