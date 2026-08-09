# Closeout evidence

## Passing checks

- `scripts/cairn/check-live-openai-provider-host --root .`: passed. Seven deterministic unit tests passed. The opt-in live smoke stayed ignored.
- `cargo test --workspace --all-targets --locked`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed after the optional shell was excluded from default-feature library builds.
- `cargo clippy -p animus-std-openai --all-targets --features reqwest-client -- -D warnings`: passed.
- `OpenAiApiKey` uses `zeroize::Zeroizing` so its owned key value is cleared on drop. The wrapper remains non-cloneable and redacted from debug output.
- `cargo fmt --all`: passed.
- `scripts/cairn/check-component-catalog --root .`: passed.
- `scripts/cairn/check-runtime-facade-inventory --root .`: passed.
- `scripts/cairn/check-dogfood-readiness --root .`: passed.
- `scripts/cairn/check-request-lifecycle --root .`: passed.
- `nix build .#checks.x86_64-linux.live-openai-provider-host --no-link -L`: passed.
- `nix run .#check-nickel-configs`: passed, including negative cases.
- `nix run .#cairn -- validate --root .`: passed before lifecycle closeout.

## Bounded evidence

The deterministic checks cover request projection, response normalization, cancellation, credential binding, endpoint admission, malformed and oversized responses, and one-attempt transport behavior. They do not prove live OpenAI availability, account authorization, model availability, latency, reliability, or production readiness.

The ignored smoke test was not run because no explicit live credential and model were supplied. No live request occurred, and no credential value entered a receipt or log.

## Lifecycle closeout

- Cairn proposal, design, and task gates passed. The final task gate receipt was `d9e3b02182c35a15da490bd96e8be5cea823fc9c313c58d5c260ed1bc57eb295`.
- The first sync dry-run rejected the generic delta heading and made no mutation. The heading was corrected to `## ADDED Requirements`, and Cairn validation and gates passed again.
- `nix run .#cairn -- sync add-live-openai-provider-host --root .` passed with an unblocked six-operation plan.
- `nix run .#cairn -- sync add-live-openai-provider-host --root . --execute` passed and changed the accepted-spec hash to `c3b258cb77484d031e5f49c4ca37924ec17b103c0142dc02e6b8ac5af55b3da6`.
- The accepted spec contains all `agent_harness_runtime.live_openai_provider_host.*` requirement and scenario IDs.
- `CAIRN_ARCHIVE_DATE=2026-08-09 nix run .#cairn -- archive add-live-openai-provider-host --root .` passed as an unblocked dry-run.
- `CAIRN_ARCHIVE_DATE=2026-08-09 nix run .#cairn -- archive add-live-openai-provider-host --root . --execute` passed with receipt `f1ee3a87ba8f06ca4617a932dc37bed8dda187cc1e56dfcbe43e1dd2c0985dfd`.
- Post-archive Cairn validation passed with zero active changes. The focused rail and evidence-safety rail also passed.

## Existing broad-check blocker

- `nix flake check -L` evaluated all 102 checks, then failed in the unrelated `host-control-extraction` check. That check requests the absent `host_control_conformance` test target instead of the available `hostcontrol` target.
- `nix build .#checks.x86_64-linux.host-control-extraction --no-link -L` fails the same way on unchanged `origin/main`. The live OpenAI focused Nix check passes on the final staged source.
