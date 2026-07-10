# Composition conformance rail

The composition conformance rail proves representative Animus surfaces compose safely without live host authority. It sits above isolated unit tests and below full workspace checks.

## Scope

The maintained matrix covers:

- provider hardening plus model dispatch;
- fixture-backed provider streaming plus runtime stream projection;
- model-router execution envelopes plus injected-service fail-closed behavior;
- tool contracts plus the coherent fake executor kit and provenance receipts;
- headless protocol projection plus runtime events;
- middleware denials plus runtime failure reports;
- capability absence plus no ambient fallback.

Each matrix entry has an owner, feature requirements, one positive case, one negative case, and a safe evidence label. Positive scenarios use in-memory DTOs and fake adapters. Negative scenarios cover malformed model responses, disabled model surfaces, unauthorized tools, missing executor-kit bindings, oversized output, unsafe receipts, unsafe headless records, middleware failures, and ambient fallback attempts.

## Command

```sh
scripts/cairn/check-composition-conformance --root .
nix build .#checks.x86_64-linux.composition-conformance --no-link -L
```

The script runs the `animus-runtime` composition conformance tests and writes a bounded receipt under `target/cairn/focused-rails/composition-conformance.md`.

## Evidence contract

Evidence contains scenario names, case classes, counts, safe labels, and BLAKE3 hashes. It omits raw prompts, provider payloads, credentials, private paths, and unbounded tool output.
