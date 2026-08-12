## Why

Animus already separates reusable application logic from host effects through `animus-core`, adapter contracts, runtime orchestration, and authority-scoped host shells. It also binds resumable sessions to exact metadata-only runtime authority profiles.

These mechanisms are strong but product-local. Animus does not yet project one application entrypoint and its declared effects into the portable Kamacite `ApplicationHostBinding` shape.

A bounded pilot can test whether the Roc-inspired application/platform presentation improves inspection without creating new runtime authority. The pilot must reuse existing Animus identities and enforcement rather than create a second policy path.

## What Changes

- Add an authority-free Animus projection for one non-default deterministic fake-backed headless entrypoint.
- Bind the application artifact, entrypoint, input/output schemas, closed symbolic effect summary, one existing session runtime authority profile, adapter cohort, resource profile, and non-claims.
- Consume one exact accepted Kamacite binding schema and producer cohort through frozen fixtures before considering a direct dependency.
- Require the binding to be committed before dependent model or tool effects in the pilot path.
- Consume the accepted runtime-authority-unit seal before those effects instead of defining a second seal model.
- Keep current authorization, capability probes, sandbox checks, adapter readiness, and per-run observations authoritative.
- Add a read-only explanation surface that shows the full effective binding without exposing prompts, credentials, provider payloads, or host paths.
- Add positive and negative conformance fixtures and one focused repository check.

## Impact

- **Core and adapters**: authority-free projection DTOs and pure validation helpers.
- **Runtime**: pilot composition and pre-effect binding checks for one non-default fake-backed path only.
- **Host shell**: bounded profile loading and safe explanation output.
- **Fixtures**: exact Kamacite conformance materials plus stale, ambiguous, unknown-effect, and overclaim cases.
- **Documentation**: application/host ownership map, Roc design reference, and explicit non-claims.

## Out of Scope

- Adding the Roc compiler, runtime, platform ABI, or Roc source execution.
- Replacing session authority profiles, model runtime profiles, tool admission, Basalt, UCAN, sandbox, or host capability checks.
- Enabling a new provider, tool executor, network path, persistence backend, or default feature.
- Generalizing the binding to Lattice, Aspen, or every Animus entrypoint before the pilot passes.
