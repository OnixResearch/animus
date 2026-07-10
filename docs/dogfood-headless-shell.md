# Packaged headless dogfood smoke

The current dogfood shell evidence rung is **packaged headless smoke**. The non-default `animus-dogfood-smoke` binary is gated behind the `animus-std/dogfood-smoke` feature and runs the reusable fake-backed dogfood core with deterministic model/tool fixtures.

Run it from the repository root:

```sh
cargo run -p animus-std --features dogfood-smoke --bin animus-dogfood-smoke -- --prompt "bounded prompt fixture" --tool fake-readonly-tool
```

The output is a safe line-oriented receipt projection containing the packaged rung, core fake-backed rung, event count, BLAKE3 prompt/receipt identities, ordered event labels, and omitted-surface non-claims. It does not print the raw prompt, raw model output, raw tool body, provider payloads, credentials, private paths, or unbounded logs.

This evidence proves only the packaged fake-backed request/admission/event/receipt path. It does **not** claim live-provider execution, credential discovery, network dispatch, default product tools, persistence, daemon attach, TUI dogfood, plugin runtime behavior, production readiness, or Clankers parity.

Focused validation:

```sh
scripts/cairn/check-packaged-headless-dogfood-smoke --root .
nix build .#checks.x86_64-linux.packaged-headless-dogfood-smoke --no-link -L
```

Next validation rungs are optional live-provider checks, optional TUI dogfood, and current-head release evidence indexing; each requires separate receipts and non-claim review.
