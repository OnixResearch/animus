## Why

Animus has many explicit adapter, model, tool, Steel, runtime, std-shell, and TUI boundaries. ast-grep can quickly inventory source shapes that threaten those boundaries: ambient authority in reusable crates, host effects in pure reducers, unsafe or panic-prone code, direct provider calls outside adapters, and stale runtime facade exports. Animus should use ast-grep as a structural audit rail while preserving Rust tests, Nickel config checks, Octet linting, and Cairn conformance rails as the authoritative gates.

## What Changes

- Define ast-grep agent-boundary audit profiles for core, adapter, runtime, std-shell, Steel, and TUI surfaces.
- Require positive and negative rule fixtures before any audit rule can become blocking.
- Record ast-grep tool identity, rule bundle BLAKE3 hash, scan scope, findings, and safe non-claims in Animus receipts.
- Keep ast-grep findings as candidate evidence for boundary review, not proof of harness safety.

## Impact

- **Surfaces**: agent-harness runtime, adapter conformance, Steel runtime policy, component catalog, runtime facade inventory, release evidence index.
- **Non-claims**: ast-grep does not prove agent safety, adapter correctness, provider behavior, Steel policy enforcement, or release readiness.
- **Validation**: ast-grep rule tests, boundary fixture scans, Cairn validation/gates, and the smallest relevant Animus conformance rails.
