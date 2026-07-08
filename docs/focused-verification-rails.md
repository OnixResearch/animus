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
| Nickel-governed Steel runtime policy | `scripts/cairn/check-steel-runtime-policy --root .` | Nickel export drift check, generated-policy activation fixture, invalid-policy fixtures, Rust positive/negative Steel program fixtures, optional `CAIRN_POLICY_SOURCE` drift check; receipt in `target/cairn/focused-rails/steel-runtime-policy.md` | `checks.x86_64-linux.steel-runtime-policy` |
| Safe receipts and artifacts | `scripts/cairn/check-safe-receipts-artifacts --root .` | Adapter receipt leak fixtures, BLAKE3 artifact determinism, redaction-class tests | `checks.x86_64-linux.safe-receipts-artifacts` |
| Neutral ledger replay | `scripts/cairn/check-neutral-ledger --root .` | Positive replay tests, unsupported-entry tests, missing-ledger-service tests | `checks.x86_64-linux.neutral-ledger` |
| Injected services | `scripts/cairn/check-injected-services --root .` | Runtime missing-service and adapter-failure tests plus effect outcome fixtures; receipt in `target/cairn/focused-rails/injected-services.md` | `checks.x86_64-linux.injected-services` |
| Composition conformance | `scripts/cairn/check-composition-conformance --root .` | Runtime matrix, fake-backed positive composition, fail-closed negative composition, and safe receipt leak checks; receipt in `target/cairn/focused-rails/composition-conformance.md` | `checks.x86_64-linux.composition-conformance` |
| Clash/UCAN authority | `scripts/cairn/check-clash-ucan-authority --root .` | Nickel capability-policy export drift, positive UCAN authority fixtures, negative fail-closed fixtures, sandbox constraints, and receipt leak rejection | `checks.x86_64-linux.clash-ucan-authority` |
| Trellis verified-core boundaries | `scripts/cairn/check-trellis-verified-core --root .` | Candidate inventory doc, capability/effect authorization model equivalence, negative fail-closed counterexamples, and default Trellis dependency isolation | `checks.x86_64-linux.trellis-verified-core` |
| Runtime facade inventory | `scripts/cairn/check-runtime-facade-inventory --root .` | Nickel inventory fixtures for valid, unclassified, and ambient-authority exports | `checks.x86_64-linux.runtime-facade-inventory` |
| Subagent runtime facade | `cargo test -p animus-runtime subagents --locked` | In-crate positive and negative fixtures for typed capabilities, context packets, facade validation, injected coordinator service, fan-out/fan-in, lineage replay, UX descriptors, and offline ACP/MCP conformance | not wired |
| Wasm/no-std dependency isolation | `scripts/cairn/check-wasm-surface --root .` | Core, adapters, router, and no-default root facade compile plus allowed/rejected dependency-tree fixtures in `cairn/wasm-surface-fixtures/` | `checks.x86_64-linux.wasm-surface` |
| Router recipe policy conformance | `scripts/cairn/check-router-policy --root .` | Nickel router policy, generated JSON drift, positive family fixtures, and negative family/role/bounds/fallback/backend fixtures | `checks.x86_64-linux.router-policy` |
| Product-surface rejection | `scripts/cairn/check-clankers-product-surfaces --root .` | Bounded-doc positive fixture and reusable-contract negative fixture | `checks.x86_64-linux.clankers-product-surfaces` |
| Tool catalog descriptors | `scripts/cairn/check-tool-catalog --root .` | Nickel schema export plus positive and negative catalog fixtures for descriptor validation and runtime-neutrality markers | not wired |

## Closeout use

Cairn closeout evidence should name the focused rails that passed, the broad checks that ran, and any skipped rails with a blocker plus next-best validation. Evidence must separate covered contracts from non-claims such as live providers, TUI or daemon dogfood, VM checks, remote services, plugin runtime, performance claims, or production readiness.

The Steel runtime policy rail guards the Nickel-authored Steel profile export, checked generated JSON, activation receipts, invalid-policy denials, Rust adapter-boundary revalidation, and the explicit Cairn policy drift hook through `CAIRN_POLICY_SOURCE`. It does not claim ambient filesystem, shell, network, provider, credential, daemon, or session mutation authority for Steel programs.
