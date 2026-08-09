# Evidence: Packaged live OpenRouter smoke

## Baseline

- `cargo test -p animus-std-openai --features reqwest-client --locked`: PASS before core edits.
- `scripts/cairn/check-packaged-live-openai-smoke --root .`: PASS before core edits.
- `cargo test -p animus-std providers --locked`: PASS before provider model-identity edits.

## Deterministic implementation evidence

- `cargo test -p animus-std providers --locked`: PASS with 29 focused provider tests.
- `cargo test -p animus-std-openai --features reqwest-client --locked`: PASS.
  - 9 library tests passed.
  - 2 OpenAI binary tests passed.
  - 2 OpenRouter binary tests passed.
  - 1 explicit OpenAI live test remained ignored.
- `cargo clippy -p animus-std --all-targets -- -D warnings`: PASS.
- `cargo clippy -p animus-std-openai --all-targets --features reqwest-client -- -D warnings`: PASS.
- `scripts/cairn/check-live-openai-provider-host --root .`: PASS.
- `scripts/cairn/check-packaged-live-openrouter-smoke --root .`: PASS.
- `scripts/cairn/check-component-catalog --root .`: PASS.
- `scripts/cairn/check-dogfood-readiness --root .`: PASS.
- `nix build .#checks.x86_64-linux.packaged-live-openrouter-smoke --no-link -L`: PASS locally after remote builder fallback.
- Cairn validation and proposal, design, and tasks gates: PASS.

## Diagnostic live evidence

A diagnostic request used the `openrouter` key from `/home/brittonr/.pi/agent/auth.json` through standard input and model `openai/gpt-5.5`.

The smoke returned `LIVE_OPENROUTER_SMOKE status=passed`, provider `openrouter`, endpoint `openrouter-responses`, status `success`, one response item, and BLAKE3 request and response identities. It omitted the prompt, credential, raw response, and model output.

This diagnostic ran from a modified worktree. It proves wire compatibility for the tested request, but it is not current-head release evidence. Closeout requires another run from a clean committed revision.

## Corrected failures

- Provider model identifiers initially rejected `/`. The identity core now admits `/` only for model identifiers and retains provider and credential denial tests.
- The first focused Nix attempt omitted the new untracked wrapper from the flake source. Staging the reviewed source fixed the source boundary.
- The second focused Nix attempt exposed a direct `/usr/bin/env` wrapper invocation. The wrapper now invokes the existing rail through `sh`, and the focused Nix check passes.

## Portfolio outcome

The exact second endpoint cohort survived the adversarial audit for crossed endpoint pairs, arbitrary URLs, credential omission, response omission, one-attempt host reuse, provider labeling, and OpenAI regression coverage.

Replacing OpenAI, adding an arbitrary URL, duplicating the host crate, and using raw `curl` were rejected. Reliability, model quality, provider routing choice, billing, production readiness, retry, fallback, streaming, and tool use remain non-claims.
