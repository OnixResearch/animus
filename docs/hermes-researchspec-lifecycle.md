# Hermes ResearchSpec lifecycle adaptation

Animus adapts Hermes ResearchSpec as an optional optimization-study lifecycle. The reusable runtime records and validates typed lifecycle facts, while hosts continue to own benchmark execution, clocks, filesystem access, process control, metric extraction, and archive storage.

The pure runtime surface lives in `animus-runtime::researchspec` and is re-exported by `animus-runtime`. It provides DTOs and deterministic functions for:

- typed study contracts with objective, primary metric, metric direction, threshold or noise policy, workload identity, checks, scope, off-limits files, guarded measurement files, promotion thresholds, and provenance;
- immutable baseline evidence with metric values, check status, config/workload/guarded fingerprints, and BLAKE3 evidence hashes;
- first-class hypotheses with claims, interventions, expected signals, falsification criteria, risk notes, linked runs, and decisions;
- candidate run evidence with study/hypothesis IDs, sequence metadata, diff fingerprints, metrics, status, checks, changed-file summaries, guarded fingerprints, measurement risks, rollback reasons, and next actions;
- keep gates that combine metric comparison, checks, threshold/noise policy, hypothesis traceability, guarded-drift detection, and measurement-integrity risks;
- promotion-group gates that distinguish independent, dependent, and overlapping kept-run groups;
- archive validation that rejects unresolved hypotheses, undecided runs, unknown run references, and unresolved in-scope ideas.

## Boundary

Animus does not execute benchmarks or infer metrics inside the reusable runtime. A host may run benchmark commands, parse outputs, read files, or collect timestamps, but it must pass already-loaded DTOs into pure validation and gate functions. This keeps the lifecycle deterministic and testable without credentials, providers, or ambient authority.

## Fingerprints

New lifecycle hashes use BLAKE3 by default. The helper `blake3_researchspec_fingerprint` domain-separates hashes under `animus.researchspec.v1` so study, baseline, workload, guarded-input, and diff fingerprints can be compared without exposing raw inputs.

## Focused verification

The focused runtime tests cover both positive and negative paths:

```sh
nix develop -c cargo test -p animus-runtime researchspec
```

Coverage includes valid and incomplete study contracts, missing primary metrics, guarded fingerprints, vague hypotheses, failed-run rollback context, suspicious metric improvements, metric-regression rejection, independent/dependent/overlapping promotion groups, and unresolved archive decisions.
