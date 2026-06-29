# Proposal: Adapt Ornith offline evaluation feedback loop for Animus

## Why

Ornith's self-improvement loop optimizes scaffolds from rollout reward. Animus should adapt the feedback shape, not the live RL system: propose a reviewed scaffold strategy, evaluate it through bounded local runs, record deterministic redacted receipts, review the evidence, and only then accept a revision.

This keeps self-improvement offline, reproducible, and host-admitted while giving future batch and dataset surfaces enough metadata to compare scaffold strategies safely.

## What Changes

- Define offline scaffold evaluation plans for task sets, selected scaffold revision, model route, verifier identity, trust-boundary policy hash, resource limits, run count, and output format.
- Record evaluation receipts with model ID, scaffold hash, tool-surface hash, sandbox policy hash, verifier hash, context-window class, sampling parameters, timeout/resource labels, run count, status, and redaction state.
- Define feedback candidate records that summarize outcomes and require host review before any scaffold profile changes become active.
- Add evidence freshness rules that mark or downweight results when model route, scaffold hash, tool policy, verifier, trust boundary, or runtime version changes.
- Integrate with local batch/evaluation surfaces without adding cloud benchmark execution or provider-specific eval APIs.

## Impact

- **Files**: future evaluation plan DTOs, receipt/projection logic, freshness policy, feedback candidate fixtures, tests, docs, and Cairn artifacts.
- **Testing**: positive deterministic plan/receipt/freshness tests and negative stale-evidence, malformed usage, missing verifier hash, raw transcript leak, unreviewed feedback, remote-run, and provider-specific benchmark claim tests.
- **Non-claims**: this does not implement RL training, live self-mutation, cloud eval runners, public benchmark claims, hidden-test access, or automatic selection of Ornith or any other model family.
