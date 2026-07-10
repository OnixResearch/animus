## Why

Animus model adapters need a future-ready way to consume model verification, bound-propagation, kernel-proof, and conversion reports without trusting ML frameworks or promoting model correctness. The gamma-crown and mly repositories are useful references for structured model verification reports, bound propagation evidence, conversion reports, and proof-certificate non-claim hygiene.

Reference sources: `https://github.com/joweeba/gamma-crown`, `https://github.com/joweeba/mly`, and `https://github.com/joweeba/dML`.

## What Changes

- Define a model-evidence adapter DTO for optional ML verification artifacts such as bound reports, conversion reports, kernel-proof summaries, benchmark/provenance rows, and unsupported-gaps inventories.
- Keep model execution, GPU/runtime access, filesystem reads, and verifier invocation outside reusable core/runtime crates.
- Require explicit evidence kind, model artifact identities, property scope, verifier identity, freshness, diagnostics, and non-claims.
- Add positive/negative fixtures so evidence can inform routing/admission without becoming a proof of arbitrary model safety.

## Impact

- **Surfaces**: model adapter contracts, model-router execution docs, observability/safe-receipt artifacts, optional std-host report ingestion.
- **Non-claims**: No mandatory ML stack, no GPU dependency, no arbitrary neural network safety proof, no benchmark superiority claim.
- **Validation**: DTO validation, malformed evidence fixtures, safe-receipt checks, model-router fixture checks, Cairn validation/gates.