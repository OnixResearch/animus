# Design: Adapt Hermes ResearchSpec lifecycle for Animus

## Context

ResearchSpec is a Hermes workflow for disciplined optimization studies. Its transferable value is the lifecycle contract and anti-reward-hacking gates, not a specific benchmark runner.

Reference material inspected:

- `/home/brittonr/git/hermes/openspec/specs/researchspec/spec.md`
- `/home/brittonr/git/hermes/_global/skills/researchspec/SKILL.md`
- `/home/brittonr/git/hermes/_global/extensions/researchspec/core.ts`

## Decisions

### 1. ResearchSpec is optional workflow state

**Choice:** Define DTOs and gates for studies, baselines, hypotheses, runs, keep decisions, promotion groups, and archives outside the mandatory runtime loop.

**Rationale:** Most agent sessions do not run optimization studies, but when they do, the lifecycle should be rigorous.

### 2. Study contracts are typed and host-validated

**Choice:** Repo-owned study config should be Nickel-authored when practical, then exported or parsed into typed Animus DTOs before runtime use.

**Rationale:** Human-authored config benefits from contracts and merge semantics; core logic should not execute Nickel.

### 3. Baseline evidence is immutable

**Choice:** Baseline records include metric values, checks, config snapshot, guarded file/input fingerprints, and BLAKE3 hashes.

**Rationale:** Candidate runs need a stable comparison target and measurement-integrity guardrails.

### 4. Hypotheses and failed runs are durable learning

**Choice:** Runs link to hypotheses and must record rollback reason or next action when discarded, crashed, or check-failed.

**Rationale:** Future agents should not repeat dead ends after code is reverted.

### 5. Keep/promotion gates defend against reward hacking

**Choice:** Metric improvement alone is insufficient. Gates check required checks, threshold/noise policy, guarded-file drift, hypothesis traceability, suspicious measurement changes, and group overlap.

**Rationale:** Optimization workflows are vulnerable to benchmark gaming.

## Risks / Trade-offs

- Lifecycle overhead is too heavy for tiny experiments. Keep ResearchSpec optional and allow lightweight study setup only when the user requests optimization/research work.
- Metric noise is domain-specific. Make threshold/noise policy typed configuration.
- Host benchmark tooling differs. Animus should record commands and evidence, not own execution engines.
