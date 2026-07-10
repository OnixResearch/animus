# Model Evidence Adapters Specification

## Purpose

Define an optional model-evidence adapter boundary for ML verification and conversion reports.

## Requirements

### Requirement: Model evidence report DTO
r[model-evidence-adapters.report-dto] Animus MUST define model-evidence report DTOs that record evidence kind, model artifact identity, property scope, verifier or tool identity, input corpus identity, result status, diagnostics, freshness, capability labels, and explicit non-claims.

#### Scenario: Bounded model evidence validates
r[model-evidence-adapters.positive]
- GIVEN a host adapter provides an already-produced model bound report, conversion report, or kernel-proof summary for a known model artifact
- WHEN Animus validates the model-evidence report DTO
- THEN validation MUST accept the supported report profile only when identity, property scope, verifier identity, freshness, diagnostics, and non-claims are present and internally consistent.

### Requirement: Reusable crates do not execute verifiers
r[model-evidence-adapters.core-validation] Animus reusable crates MUST validate already-loaded model-evidence DTOs and MUST NOT execute models, access GPUs, read files, invoke verifiers, inspect environment state, access clocks, spawn processes, or perform network I/O.

#### Scenario: Host shell owns model evidence effects
r[model-evidence-adapters.host-shell]
- GIVEN model evidence is collected from a verifier, conversion tool, benchmark runner, or model runtime
- WHEN the report enters Animus
- THEN all collection, filesystem access, process execution, GPU access, persistence, and presentation MUST remain in host-owned adapters.

### Requirement: Model evidence fails closed on overclaim
r[model-evidence-adapters.negative] Animus MUST reject stale, mismatched, unsupported, benchmark-only, malformed, or overclaiming model-evidence reports.

#### Scenario: Overclaiming model evidence is rejected
- GIVEN a report has a stale freshness envelope, mismatched model hash, unsupported verifier kind, missing property scope, benchmark-only result, absent diagnostics, capability overclaim, or weakened non-claims
- WHEN model-evidence validation runs
- THEN validation MUST fail with deterministic diagnostics and MUST NOT promote the model, route, or task as verified.

### Requirement: Accepted evidence remains advisory unless promoted
r[model-evidence-adapters.validation] Animus MUST treat accepted model-evidence reports as bounded advisory inputs unless a later Cairn change promotes a specific report profile with additional gates.

#### Scenario: Accepted evidence preserves boundaries
- GIVEN a model-evidence report passes validation
- WHEN model routing or admission consumes it
- THEN the decision receipt MUST preserve non-claims for arbitrary model robustness, output correctness, hallucination prevention, verifier soundness, benchmark superiority, release eligibility, GPU correctness, and whole-system safety.