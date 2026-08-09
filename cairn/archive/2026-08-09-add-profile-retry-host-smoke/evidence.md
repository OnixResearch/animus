# Evidence: Profile-bound provider-retry host smoke

## Baseline

- `cargo test -p animus-std --features 'model-runtime-profiles provider-retry' --locked`: PASS before implementation.
- Baseline result: 68 `animus-std` tests passed.

## Focused implementation evidence

- `cargo test -p animus-std --features profile-retry-smoke profile_retry_smoke --locked`: PASS.
  - One positive exact-profile retry test passed.
  - Two negative test groups passed.
- `cargo clippy -p animus-std --features profile-retry-smoke --all-targets --locked -- -D warnings`: PASS.
- `scripts/cairn/check-model-runtime-profiles --root .`: PASS.
- `scripts/cairn/check-provider-retry --root .`: PASS.
- `scripts/cairn/check-profile-retry-host-smoke --root .`: PASS.
  - The rail exercised one successful exact retry.
  - The rail rejected missing arguments, malformed policy, missing profile, terminal failure, cancellation, request drift, and missing model service.
  - The rail wrote `target/cairn/focused-rails/profile-retry-host-smoke.md` with source and head metadata.
- `nix build .#checks.x86_64-linux.profile-retry-host-smoke --no-link -L`: PASS after the unavailable remote builder fell back locally.
- `scripts/cairn/check-dogfood-readiness --root .`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- Cairn validation and proposal, design, and tasks gates: PASS before closeout.

## Broader Tiger Style blocker

- `cargo tigerstyle check`: FAIL on 254 pre-existing findings outside the new profile-retry smoke files.
- The full Octet log contained no finding for `profile_retry_smoke`.
- The focused Clippy and Nix rails passed for the changed surface.

## Current-head receipt state

The pre-commit focused receipt correctly records `head_dirty: true`. After the completion commit, the rail must run again from the clean change head. That run supplies the clean current-head receipt.

## Portfolio outcome

The separate deterministic smoke survived the adversarial audit for profile substitution, request drift, terminal failure, cancelled waits, missing services, malformed policy, and content leaks.

The validated claim is limited to explicit profile loading and one fake-backed same-plan retry. Live-provider reliability, fallback, streaming, persistence, TUI, daemon, and production behavior remain non-claims.
