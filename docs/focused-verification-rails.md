# Focused verification rails

Focused rails are small, credential-free checks that validate one Animus boundary and emit reviewable evidence. They complement broad workspace checks; they do not replace Rust tests, Clippy, Nickel drift checks, Cairn validation, or Cairn gates.

## Convention

Each rail MUST:

- validate one named boundary;
- use deterministic positive and negative fixtures where the boundary accepts data or protects a safety property;
- fail closed with an actionable diagnostic when the boundary drifts;
- keep evidence safe and versioned with `schema: animus.focused-rail.v1`;
- write durable local receipts under `target/cairn/focused-rails/<rail>.md` when a receipt is useful;
- keep committed closeout evidence to bounded facts: rail name, status, fixture names, issue classes, safe labels, and BLAKE3 hashes when content identity matters;
- omit raw prompts, credentials, provider payloads, private full paths, raw script bodies, raw tool input/output, environment values, live host state, and unbounded logs.

A focused rail MAY print bounded stdout instead of writing a receipt when it is an existing static marker rail. If a rail depends on optional local inputs, it MUST either require the input explicitly or record the skipped mode and next-best check.

## Current rails

| Boundary | Command | Fixtures/evidence | Nix check |
| --- | --- | --- | --- |
| Lifecycle ownership map | `scripts/cairn/check-request-lifecycle --root .` | Positive and negative fixtures in `cairn/request-lifecycle-fixtures/`; marker diagnostics only | `checks.x86_64-linux.request-lifecycle` |
| Steel runtime activation and policy-drift hook | `scripts/cairn/check-steel-runtime-policy --root .` | Rust positive/negative Steel program fixtures; optional `CAIRN_POLICY_SOURCE` drift check; receipt in `target/cairn/focused-rails/steel-runtime-policy.md` | `checks.x86_64-linux.steel-runtime-policy` |
| Safe receipts and artifacts | `scripts/cairn/check-safe-receipts-artifacts --root .` | Adapter receipt leak fixtures, BLAKE3 artifact determinism, redaction-class tests | `checks.x86_64-linux.safe-receipts-artifacts` |
| Neutral ledger replay | `scripts/cairn/check-neutral-ledger --root .` | Positive replay tests, unsupported-entry tests, missing-ledger-service tests | `checks.x86_64-linux.neutral-ledger` |
| Injected services | `scripts/cairn/check-injected-services --root .` | Runtime missing-service and adapter-failure tests plus effect outcome fixtures; receipt in `target/cairn/focused-rails/injected-services.md` | `checks.x86_64-linux.injected-services` |
| Runtime facade inventory | `scripts/cairn/check-runtime-facade-inventory --root .` | Nickel inventory fixtures for valid, unclassified, and ambient-authority exports | `checks.x86_64-linux.runtime-facade-inventory` |
| Wasm/no-std dependency isolation | `scripts/cairn/check-wasm-surface --root .` | Allowed and rejected dependency-tree fixtures in `cairn/wasm-surface-fixtures/` | `checks.x86_64-linux.wasm-surface` |
| Product-surface rejection | `scripts/cairn/check-clankers-product-surfaces --root .` | Bounded-doc positive fixture and reusable-contract negative fixture | `checks.x86_64-linux.clankers-product-surfaces` |

## Closeout use

Cairn closeout evidence should name the focused rails that passed, the broad checks that ran, and any skipped rails with a blocker plus next-best validation. Evidence must separate covered contracts from non-claims such as live providers, TUI or daemon dogfood, VM checks, remote services, plugin runtime, performance claims, or production readiness.

The Steel runtime policy rail currently guards the existing Steel activation boundary and exposes an explicit hook for policy drift through `CAIRN_POLICY_SOURCE`. The full Nickel-governed Steel runtime policy is owned by the separate `adapt-clankers-steel-runtime-policy` change; this convention prevents that future work from claiming completion without its own focused fixtures and receipts.
