# Tasks

## Phase 1 — Guarded Skill-Cache Probation & Silent-Shift State Machine
- [ ] Add pure deterministic skill-cache lifecycle states (`ProbationShadow`, `ActiveSampled`, `SuspendedDrift`, `Retired`), applicability-guard verdict types, and seeded live-hit drift sampling in `crates/animus-adapter-continual/src/lib.rs` and `crates/animus-std-agent-guardrail/src/{core.rs, model.rs}`.
- [ ] Emit canonical JSON/Preserves receipts recording skill digest, guard verdict, sampling outcome, and explicit probabilistic non-claims.

## Phase 2 — Hybrid System 1 / System 2 Loop Wiring
- [ ] Wire `crates/animus-frozen-policy` bounded reflex outputs (`InvokeCachedSkill`, `QueryExternalMemory`, `EscalateSystem2`) through `crates/animus-std-basalt-authority` attenuated capability checks and fallback-on-miss routing.
- [ ] Enforce explicit `Unknown` outcome handling (no blind retries) when a skill or memory invocation traps mid-effect.

## Phase 3 — Adversarial Silent-Shift & End-to-End Verification
- [ ] Add regression tests covering (a) `ProbationShadow` promotion and rejection, (b) exit-code-`0` silent distribution shift caught by live sampling -> `SuspendedDrift`, (c) guard-miss clean fallback to System 2 on rephrased/out-of-range inputs, (d) misfiring reflex capability refusal, and (e) byte-identical deterministic replay.
- [ ] Run workspace tests and clippy for all touched crates (`animus-frozen-policy`, `animus-adapter-continual`, `animus-std-agent-guardrail`, `animus-std-basalt-authority`).
