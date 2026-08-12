# Evidence: Packaged live OpenAI smoke

## Baseline

- `scripts/cairn/check-live-openai-provider-host --root .`: PASS before core edits.
- Existing result: 7 host tests passed and the explicit live test remained ignored.

## Focused implementation evidence

- `cargo test -p animus-std-openai --features reqwest-client --locked`: PASS.
  - 9 library tests passed.
  - 2 packaged binary tests passed.
  - 1 explicit live test remained ignored.
- `cargo clippy -p animus-std-openai --all-targets --features reqwest-client -- -D warnings`: PASS.
- `scripts/cairn/check-live-openai-provider-host --root .`: PASS after edits.
- `scripts/cairn/check-packaged-live-openai-smoke --root .`: PASS.
- `scripts/cairn/check-release-evidence-index --root .`: PASS through the packaged rail.
  - Current-head live-provider fixture passed.
  - Stale and dirty live-provider fixtures failed with the expected issue codes.
- `scripts/cairn/check-component-catalog --root .`: PASS.
- `scripts/cairn/check-dogfood-readiness --root .`: PASS.
- `nix build .#checks.x86_64-linux.packaged-live-openai-smoke --no-link -L`: PASS locally after the unavailable remote builder fell back.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- Cairn validation and proposal, design, and tasks gates: PASS before closeout.

## Corrected failures

- The first smoke plan test exposed a stale reviewed fixture line for disabled reasoning. The accepted projection omits that line. The fixed fixture now passes through the production planner.
- The first focused Nix build could not invoke the release checker by a relative program path. The rail now invokes the checked script through `sh`, and the Nix check passes.

## Live evidence blocker

No OpenAI API key or reviewed live model was supplied to this session. No live provider request ran.

The packaged binary, deterministic denial tests, and current-head evidence checks are validated. Live OpenAI availability remains a non-claim until an operator runs the packaged command from a clean indexed head.

## Portfolio outcome

The authority-scoped packaged binary survived the adversarial audit for bounded input, credential omission, response omission, one-attempt host reuse, and stale evidence rejection. The final state is validated for implementation and blocked for credential-backed live evidence.
