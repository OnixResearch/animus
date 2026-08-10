# Evidence

## Baseline

- `cargo test -p animus-std --features 'agent-role-profiles model-runtime-profiles' --locked` — PASS before implementation.
- `scripts/cairn/check-agent-role-profiles --root .` — PASS before implementation.

## Focused implementation evidence

- `cargo test -p animus-std --features multi-role-smoke --locked multirolesmoke` — PASS, 4 positive and negative tests.
- `cargo clippy -p animus-std --features multi-role-smoke --all-targets --locked -- -D warnings` — PASS.
- `scripts/cairn/check-multi-role-host-smoke --root .` — PASS. The rail validates explicit role and model-profile loading, four role rows, separate outputs, judge veto, reviewer mutation rejection, completion non-equivalence, safe output omission, and seven negative CLI paths.
- `nix build .#checks.x86_64-linux.multi-role-host-smoke --no-link -L` — PASS in a sandbox.
- `cargo check --workspace --all-targets --locked` — PASS with existing `animus-std-openai` unused-code warnings.

## Boundary evidence

- `scripts/cairn/check-agent-role-profiles --root .` — PASS after implementation.
- `scripts/cairn/check-authoritative-agent-completion --root .` — PASS after implementation.
- `scripts/cairn/check-dogfood-readiness --root .` — PASS after documentation updates.
- `cargo tigerstyle check` — BLOCKED by the repository's 254 existing findings. The report contains no `multirolesmoke` or multi-role smoke finding. Its final finding remains `crates/animus-runtime/src/rejected_attempt/tests.rs:127`.

## Receipt scope

The focused rail writes `target/cairn/focused-rails/multi-role-host-smoke.md`. It binds the current Git head, dirty state, and a BLAKE3 identity over the smoke source, binary, feature declaration, generated policy projections, and focused rail.

The smoke output is metadata-only. It contains role, profile, plan, assignment, separation, capability-count, output, and receipt identities. It does not contain subject text, prompts, role output bodies, verifier bodies, judge rationale, credentials, policy paths, provider payloads, or transcripts.

## Non-claims

This evidence does not claim live-provider behavior, provider quality, verifier soundness, human or organizational independence, mutation execution, authoritative completion, TUI behavior, daemon behavior, or production readiness.
