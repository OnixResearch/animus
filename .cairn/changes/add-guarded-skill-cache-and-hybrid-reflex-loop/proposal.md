# Why

Architectural review and cross-vault distillation in `onix-insights-mneme/19-external-review-onix-architecture-and-blind-spots-2026-11-26.adoc` identify two urgent gaps at the `Animus` runtime execution boundary:

1. **Silent distribution shift in compiled skill caches (Wall 2 / `noble-lang#118` / `rsi-lab#169`):** Retiring cached CPU/Wasm/Noble skills only when they crash or fail type/precondition guards catches loud failures, but misses **silent distribution shifts** where rephrased inputs or shifted environment semantics cause cached code to return confident, wrong answers with exit code `0`. Without shadow-mode probation and periodic live hit sampling against System 2, a compiled skill cache silently degrades end-to-end reliability.
2. **Missing live end-to-end hybrid loop:** Fast frozen policies (`animus-frozen-policy` / `rsi-lab` System 1), compiled procedural skills (`noble-lang#118`), and retractable external memory (`memory-lab` / `mneme`) are currently validated in separate synthetic harnesses rather than wired into one bounded, attenuated multi-turn execution loop inside `Animus`.

# What Changes

- Add a pure, deterministic **skill-cache probation & silent-shift monitor contract** in `crates/animus-adapter-continual` and `crates/animus-std-agent-guardrail/src/{core.rs, model.rs}` supporting:
  - Explicit skill lifecycle states: `ProbationShadow`, `ActiveSampled`, `SuspendedDrift`, and `Retired`.
  - Deterministic PRNG-seeded live hit sampling (comparing a bounded fraction of cache hits against System 2 fallback) with immediate fail-closed transition to `SuspendedDrift` and rollback upon output divergence.
  - Strict input-normalization applicability guards that route any guard miss or exception directly to System 2 fallback.
- Wire a bounded **hybrid System 1 / System 2 execution loop** connecting `crates/animus-frozen-policy` (sub-millisecond bounded reflex choosing `InvokeSkill`, `QueryMemory`, or `EscalateSystem2`), the guarded skill cache, and attenuated capability enforcement (`crates/animus-std-basalt-authority` / `crates/animus-std-agent-guardrail`), emitting deterministic replayable receipts with explicit non-claims.
- Add end-to-end adversarial integration tests covering rephrased/re-numbered inputs, silent distribution shift injection (exit-code-`0` wrong answer caught by sampling and suspended), capability-boundary refusal on misfiring reflexes, and clean fallback-on-miss.
