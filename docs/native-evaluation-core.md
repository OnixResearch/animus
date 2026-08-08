# Native evaluation core

Animus provides native evaluation values through the non-default `animus-eval-core` crate. The crate is `#![no_std]` plus `alloc`. It validates caller-supplied values and produces deterministic taskset, recipe, episode, reward, resume, and receipt results. It does not load tasks, call models or tools, inspect the host, schedule work, or persist results.

## Ownership

- `animus-eval-core` owns immutable evaluation descriptors and pure decisions.
- The session ledger remains the trace and replay authority. Episodes contain exact session, branch, leaf, subject, role, and run references. They do not contain transcript bodies.
- Batch retains execution order. Agent roles retain capability and separation policy. Dataset export retains training admission. Safe-receipt and Cairn policy retain claim admission.
- `animus-runtime::harbor_eval` keeps Harbor and ATIF compatibility. Its generic matrix and reward reducers now call `animus-eval-core`.
- Future adapter and host crates must implement task loading, model and tool calls, verifier execution, artifact transfer, clocks, cancellation observation, and result storage.

## Policy and identity

`config/native-evaluation-policy.ncl` is the typed source. `config/generated/native-evaluation-policy.json` is its checked runtime projection. `animus-std::parse_native_evaluation_policy_json` rejects unknown fields, stale source-cohort data, unsafe defaults, unsupported values, and invalid bounds before it calls the no-std validator.

The reviewed reference cohort is `PrimeIntellect-ai/verifiers` commit `1a997814e85a2e4f409add960aa852737388dbd8`, MIT. Its bounded source-cohort identity is `b3:069f20983a0d523d84028b76f85b3dd63a712f952e99cdef628824ef1ffe6bfe`. The reviewed v1 owners are `verifiers/v1/task.py`, `taskset.py`, `env.py`, `episode.py`, `trace.py`, `cli/eval/resume.py`, and `cli/resume.py`. Recipe references are `envs/single_agent/env.py`, `envs/best_of_n/env.py`, `envs/user_sim/env.py`, `envs/agentic_judge/env.py`, and `envs/shared_agentic_judge/__init__.py`. Animus adopts immutable task data, explicit tasksets, episode structure, content-addressed resume, and separate environment patterns. It rejects the Python runtime, decorators, package discovery, dynamic harness installation, provider traffic, containers, training services, and mutable shared state.

All stack-owned identities use domain-separated BLAKE3 framing. Resume keys bind task data, taskset, recipe, model, harness, environment, tools, verifier, rubric, reward, sampling, redaction, output, and policy identities.

## Minimal API shape

```rust
use animus_eval_core::{TasksetDescriptor, TasksetSource, TaskOrdering, plan_taskset};

// The host loads policy and task facts. The core only validates and plans them.
let plan = plan_taskset(&descriptor, &tasks, &policy)?;
assert_eq!(plan.tasks[0].order, 0);
```

Hosts construct typed values from already-admitted data. They must not pass raw task prompts, hidden tests, transcripts, provider bodies, or physical locators into these contracts.

## Recipes and rewards

The pure planner supports single-agent, bounded best-of-N, user simulation, isolated agentic judge, and shared advisory judge recipes. A simulated user cannot be trainable or receive assistant-only tools. An isolated judge requires distinct role and environment identities. A shared judge is always advisory and cannot satisfy verifier isolation.

Rewards have finite domains, explicit weights, stable dimension order, exact scorer and evidence bindings, visible missing-value policy, and contribution counts. NaN, infinity, duplicates, unknown dimensions, identity drift, out-of-domain values, silent clamping, and authority claims fail closed.

## Resume and evidence

Only one complete, terminal, successful, safe, within-budget record with the exact current resume key can produce `Keep`. Missing or stale evidence produces an explicit rerun, block, or redo-whole-task result. Torn or unsafe evidence blocks.

Receipts contain identities, statuses, counts, safe labels, redaction counts, and non-claims. They reject prompts, hidden tests, reasoning, tool or provider bodies, credentials, environment values, paths, transcripts, unbounded logs, and verifier, benchmark, training, completion, production, or release claims.

## Checks

Run `scripts/cairn/check-native-evaluation-core --root .`. The Nix check is `checks.x86_64-linux.native-evaluation-core`.

## Non-claims

This core does not prove task correctness, trace completeness, scorer validity, verifier soundness, model quality, benchmark comparability, training fitness, host execution, sandboxing, authoritative completion, production readiness, or release eligibility.
