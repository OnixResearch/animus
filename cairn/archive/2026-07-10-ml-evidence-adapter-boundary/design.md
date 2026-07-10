## Design

1. Add a `ModelEvidenceReport` DTO family at the adapter boundary. It records evidence kind, model artifact identity, property scope, verifier/tool identity, input corpus identity, result status, diagnostics, freshness, and non-claims.
2. Treat gamma-CROWN-style bound reports, mly-style conversion reports, Kani/kernel-proof summaries, and benchmark rows as optional report kinds. Unknown kinds fail closed unless a host adapter explicitly maps them to a supported profile.
3. Keep all model execution, GPU access, file reads, verifier invocations, network calls, clocks, and report persistence in host-owned adapters.
4. Keep reusable core validation pure: it checks already-loaded report DTOs, capability labels, hash fields, freshness envelopes, and non-claim boundaries.
5. Define routing/admission semantics as advisory unless a future change promotes a specific report profile. Passing validation may influence selection policy, but it does not prove model safety or output truth.
6. Add fixtures for accepted bounded evidence and rejected stale, missing-property, mismatched-model, unsupported-verifier, benchmark-only, and weakened-non-claim reports.

## Non-Goals

- No adoption of gamma-crown, mly, PyTorch, ONNX, Metal, CUDA, or GPU runtime as required Animus dependencies.
- No claim that accepted model evidence proves arbitrary model robustness, absence of hallucination, output correctness, production readiness, or benchmark superiority.
- No host capability grant from a model-evidence report.