# Proposal: Adapt Hermes ResearchSpec lifecycle for Animus

## Why

Hermes ResearchSpec captures a strong lifecycle for optimization and experiment work: typed study contracts, baselines, hypotheses, run evidence, keep/promotion gates, benchmark-gaming defenses, and archival conclusions. Animus should adapt this as an optional evaluation workflow so agents can run improvement studies with durable evidence instead of treating metric gains as self-justifying.

## What Changes

- Define ResearchSpec-inspired study, metric, workload, scope, baseline, hypothesis, run, keep, promotion, and archive DTOs.
- Prefer Nickel-authored contracts for repo-owned study configuration, while reusable runtime consumes typed data after host validation.
- Use BLAKE3 fingerprints for baseline and guarded workload/input evidence.
- Add keep/promotion gates that require metric improvement, checks, traceability, threshold/noise policy, and measurement-integrity guardrails.
- Keep benchmark execution and autoresearch tooling host-owned; Animus records lifecycle facts and gate decisions.

## Impact

- **Files**: future evaluation workflow DTOs, optional std adapters, Nickel contracts/exports if repo-owned configs are added, lifecycle gate tests, and docs.
- **Testing**: valid study lifecycle plus incomplete contract, missing baseline, malformed metric output, guarded-file drift, vague hypothesis, failed run without rollback context, suspicious improvement, overlapping promotion groups, and unresolved archive rejection.
- **Non-claims**: this change does not make ResearchSpec part of the core turn loop and does not duplicate benchmark execution, timing, metric extraction, or autorevert engines.
