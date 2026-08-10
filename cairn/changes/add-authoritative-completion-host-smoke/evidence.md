# Evidence

## Baseline

- `cargo test -p animus-std --features 'multi-role-smoke authoritative-completion' --locked` — PASS before implementation, 69 tests.
- `scripts/cairn/check-authoritative-agent-completion --root .` — PASS before implementation.
- `scripts/cairn/check-multi-role-host-smoke --root .` — PASS before implementation.

## Search and audit

Three correlated serial design passes covered a completion-only wrapper, a rendered role-output wrapper, and a typed role-advisory seam. The typed seam survived review because it keeps role and completion authority separate without parsing CLI output.

The adversarial audit checked judge veto, role-output self-attestation, explicit dirty head state, one missing selected result, stale execution subject identity, missing Cairn evidence, malformed generated policy, output leakage, and receipt head binding.

## Focused implementation evidence

- `cargo test -p animus-std --features authoritative-completion-smoke --locked authoritative_completion_smoke` — PASS, 4 positive and negative tests.
- `cargo clippy -p animus-std --features authoritative-completion-smoke --all-targets --locked -- -D warnings` — PASS.
- `scripts/cairn/check-authoritative-completion-host-smoke --root .` — PASS. The rail covers a verified fixture and eleven bounded CLI failure paths.
- `nix build .#checks.x86_64-linux.authoritative-completion-host-smoke --no-link -L` — PASS in a sandbox.
- `cargo check --workspace --all-targets --locked` — PASS with existing `animus-std-openai` unused-code warnings.

## Boundary evidence

- `scripts/cairn/check-agent-role-profiles --root .` — PASS after implementation.
- `scripts/cairn/check-multi-role-host-smoke --root .` — PASS after the typed advisory seam.
- `scripts/cairn/check-authoritative-agent-completion --root .` — PASS after implementation.
- `scripts/cairn/check-dogfood-readiness --root .` — PASS after documentation updates.
- `cargo tigerstyle check` — BLOCKED by the repository's 254 existing findings. The report contains no `authoritative_completion_smoke` or `multirolesmoke` finding. Its final finding remains `crates/animus-runtime/src/rejected_attempt/tests.rs:127`.

## Receipt scope

The focused rail writes `target/cairn/focused-rails/authoritative-completion-host-smoke.md`. It binds current Git head, dirty state, and a BLAKE3 identity over the smoke source, binary, feature declaration, role advisory seam, generated policy projections, and focused rail.

The packaged output contains role-advisory, registry, selection, final-subject, execution-evidence, Cairn, candidate, and completion identities. It omits subject text, role output, prompts, judge rationale, check output, commands, credentials, paths, provider payloads, and transcripts.

## Non-claims

The deterministic fake services prove composition over supplied identities only. They do not prove check-oracle soundness, honest external execution, live-provider behavior, verifier soundness, human independence, mutation execution, release readiness, TUI behavior, daemon behavior, or production readiness.
