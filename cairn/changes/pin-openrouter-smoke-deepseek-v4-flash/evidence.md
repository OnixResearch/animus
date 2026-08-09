# Evidence: Pin OpenRouter smoke to DeepSeek V4 Flash 0731

## Baseline

- `cargo test -p animus-std-openai --features reqwest-client --locked`: PASS before edits.
- `scripts/cairn/check-packaged-live-openrouter-smoke --root .`: PASS before edits.

## Deterministic evidence

- `cargo test -p animus-std-openai --features reqwest-client --locked`: PASS.
- `cargo clippy -p animus-std-openai --all-targets --features reqwest-client -- -D warnings`: PASS.
- `scripts/cairn/check-packaged-live-openrouter-smoke --root .`: PASS.
- `nix build .#checks.x86_64-linux.packaged-live-openrouter-smoke --no-link -L`: PASS after remote builder fallback.
- Cairn validation and proposal, design, and tasks gates: PASS.

Positive tests admit only `deepseek/deepseek-v4-flash-0731`. Unit and packaged process tests reject `openai/gpt-5.5` as a substituted model.

## Diagnostic live evidence

A diagnostic request used the Pi OpenRouter key through bounded standard input. The request selected `deepseek/deepseek-v4-flash-0731` and returned `LIVE_OPENROUTER_SMOKE status=passed`.

The metadata-only output recorded provider `openrouter`, endpoint `openrouter-responses`, canonical status `success`, one response item, and BLAKE3 request and response identities. It omitted the credential, prompt, model output, and raw response.

The diagnostic ran from a modified worktree. Final current-head evidence requires another run after commit and lifecycle closeout.

## Non-claims

The pin and smoke do not prove model quality, OpenRouter routing choice, provider reliability, billing correctness, or production readiness.
