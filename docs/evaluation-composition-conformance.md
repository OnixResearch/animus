# Native evaluation composition conformance

## Purpose

The conformance suite checks native evaluation boundaries together. It uses public production APIs and finite scripted services.

The suite is non-default. Select the root `evaluation-conformance` feature or run the focused rail.

```sh
scripts/cairn/check-evaluation-composition-conformance --root .
```

The suite does not contact live services. It does not load Verifiers or execute Python.

## Checked profile

`config/evaluation-composition-conformance.ncl` is the typed source. The export is `config/generated/evaluation-composition-conformance.json`.

Rust parses the export with unknown-field rejection. Rust then checks the exact schema, source cohort, families, entrypoints, services, bounds, dependencies, and evidence posture.

The profile contains 11 mechanism families and 22 paired fixtures. Each family has one positive fixture and one expected-negative fixture.

| Family | Positive mechanism | Negative mechanism |
|---|---|---|
| Taskset | exact and bounded lazy selection | generator, bound, order, and duplicate hazards |
| Recipes | five built-in recipes and pass-at-N | role widening, fan-out overflow, and shared promotion |
| Episode | ordered ledger references | stale or mismatched ledger binding |
| Ports | exact service admission and request binding | missing or mismatched service facts |
| Gateway | accepted turn, exact retry, and commit order | early terminal or changed retry evidence |
| Verifier | isolated verification and safe artifacts | links, hidden tests, collisions, or shared mutation |
| Rewards | finite attributed reduction | non-finite, duplicate, mismatched, or clamped values |
| Persistence | exact resume and explicit cleanup | stale, torn, or unresolved cleanup evidence |
| Compatibility | native Harbor and ATIF safe subset | lossy identity or unsupported ATIF version |
| Downstream | advisory feedback and admitted dataset rows | automatic promotion or private export |
| Evidence | metadata-only core and host receipts | private data or unsupported claims |

## Production-path rule

Fixtures can construct inputs and deterministic fake services. Fixtures must delegate all product decisions to public APIs.

The static checker rejects alternate planners, reducers, gateway rules, resume rules, and receipt projectors. It also rejects private test-module paths.

Assertions check typed outputs, identities, diagnostics, unchanged state, and missing success receipts. Call counts alone are not evidence.

## Fixture authoring

Add fixture metadata to both Nickel inventories:

- `fixtures/evaluation-composition-conformance/positive.ncl`
- `fixtures/evaluation-composition-conformance/negative.ncl`

Register the family and expected diagnostic in the checked profile. Add the matching production-path call in `animus-eval-conformance`.

A negative fixture must require these results:

1. The exact diagnostic family appears.
2. No prohibited service call occurs.
3. Reusable state stays unchanged.
4. No success receipt appears.
5. Evidence remains bounded and redacted.

Do not add a fixture that only lists metadata. The Rust harness must execute its public boundary.

## Evidence

The focused rail writes `target/cairn/focused-rails/evaluation-composition-conformance.md`.

The receipt contains profile, family, fixture, entrypoint, service, diagnostic, count, redaction, status, non-claim, and BLAKE3 identity metadata.

The receipt excludes fixture bodies, transcripts, hidden tests, reasoning, provider bodies, tool bodies, secrets, environment values, physical paths, and unrestricted logs.

## Live-check separation

A later live smoke can cite this suite as contract evidence. A live smoke cannot convert deterministic fixture evidence into production evidence.

Skipped live checks are not passing results. The focused rail reports only deterministic repository observations.

## Non-claims

This suite does not prove live host behavior, provider correctness, tool correctness, sandboxing, network enforcement, storage durability, or exactly-once effects.

It does not prove verifier soundness, artifact authenticity, model quality, benchmark validity, training fitness, authoritative completion, production readiness, or release eligibility.
