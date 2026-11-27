# Design

## Pure Core, Host Shell Separation

All lifecycle transitions, sampling decisions, applicability-guard verdicts, and escalation routing live in pure, deterministic, single-threaded Rust state machines with zero ambient I/O, wall-clock reads, or hidden retries:

1. **Skill-Cache Probation & Silent-Shift Defense (`animus-adapter-continual` / `animus-std-agent-guardrail`):**
   - Newly admitted compiled skills (e.g., content-addressed Noble/Wasm or Rust CPU procedures) MUST enter `ProbationShadow`: during probation, System 2 executes authoritatively while the candidate skill runs in shadow mode and must match System 2 on `K` consecutive clean evaluations before promotion to `ActiveSampled`.
   - In `ActiveSampled`, a deterministic seeded sampler routes `p_sample` (e.g., 1–5%) of cache hits to dual execution (skill + System 2 comparison). Any semantic mismatch transitions the skill atomically to `SuspendedDrift`, records a structured drift receipt (input digest, skill digest, expected vs. observed output digests), and routes all subsequent matching calls to System 2 until an explicit operator/ratchet re-qualification occurs.
   - Applicability guards run before execution; any schema, range, or parameter-binding mismatch returns `CacheMiss(GuardRejected)` and falls back cleanly to System 2 without executing the cached skill.

2. **Hybrid Reflex Loop (`animus-frozen-policy` + `animus-std-basalt-authority`):**
   - On each turn, `animus-frozen-policy` evaluates a bounded, deterministic System 1 reflex step emitting a strictly schema-checked decision (`InvokeCachedSkill(skill_id, bound_args)`, `QueryExternalMemory(query)`, or `EscalateSystem2(reason)`).
   - Output from the probabilistic reflex is fenced behind attenuated capability checks (`animus-std-basalt-authority`): a misfiring reflex can never widen host permissions, mutate unapproved paths, or bypass applicability guards.
   - If a remote or local skill/memory worker traps or times out after a potential side effect, the loop records `Unknown` rather than blindly retrying.

## Explicit Non-Claims

- Shadow probation and live sampling bound expected silent-shift exposure probabilistically (`1 - (1 - p_sample)^n`); they do **not** mathematically prove zero silent shifts on unsampled inputs. Receipts must state the sampling rate and non-claim explicitly.
- External neural models and live remote workers remain behind trait/host boundaries so deterministic replay tests run completely offline.
