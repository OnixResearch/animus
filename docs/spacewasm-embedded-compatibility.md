# SpaceWasm embedded compatibility

This non-default diagnostic lane asks one narrow question: can the test-only `fixtures/spacewasm/guest.rs` hostless `run` export be emitted for `wasm32v1-none`, admitted by the exact Octet `spacewasm-mvp` static profile, and observed through a caller-injected service for NASA SpaceWasm revision `e24cf09355a90497148eb5029fdb8e3400bd63e3`?

It is separate from Animus's supported generic `wasm32-unknown-unknown` compile boundary. Failure or unavailability here does not revoke that compile support.

## Ownership

- `config/spacewasm-embedded-compat.ncl` owns the reviewed profile and deterministic JSON projection.
- `animus_adapters::spacewasm` owns pure profile, surface, exact-evidence, resource, outcome, and metadata-only receipt decisions.
- `animus_std::spacewasm` owns only orchestration over a caller-provided `DiagnosticService`.
- Mantle owns the exact reference bundle; Octet owns execution-free artifact analysis. Neither transfers behavioral correctness.
- The caller owns bundle reads, artifact compilation, SpaceWasm invocation, memory transfer, cancellation, deadlines, and evidence persistence.

Static admission requires the hostless `run` export, no imports, only the `wasm1` proposal label, exact artifact identity agreement, exact Octet cohort agreement, and finite named limits. Disabled profiles, target drift, undeclared imports, stale reports, oversized inputs/outputs, runtime failures, receipt leaks, and claim promotion fail closed.

## Verification

Run:

```sh
scripts/cairn/check-spacewasm-compat --root .
```

The deterministic rail compiles and inspects the guest, runs positive and negative pure/shell tests, checks dependency isolation, and reports runtime execution as blocked unless `ANIMUS_SPACEWASM_RUNNER` names an explicitly injected arbitrary-guest runner.

## Non-claims

A pass is not general Animus Wasm runtime support, a plugin SDK or activation mechanism, WIT/Component Model/WASI support, SpaceWasm correctness, memory safety, semantic equivalence, sandbox completeness, configured capacity, flight qualification, embedded production readiness, certification, or release eligibility.
