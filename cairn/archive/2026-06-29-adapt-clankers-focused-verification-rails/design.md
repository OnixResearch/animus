# Design: Focused verification rails

## Context

Clankers' checked-brick pattern is valuable because each rail validates a small contract and emits evidence that a reviewer can inspect. Animus should not copy the full harness or dogfood suite, but it should adopt the small-rail discipline.

## Decisions

### 1. Rails validate one boundary

Each focused rail should cover one boundary: lifecycle docs, Steel policy, receipt redaction, artifact identity, ledger replay, injected services, provider parity, plugin manifests, or release evidence. A failing rail should point at one contract.

### 2. Rails emit deterministic evidence

Rails should write JSON or Markdown receipts under `target/` with schema, status, fixture names, hashed artifacts, and safe metadata. They must not include raw prompts, credentials, provider payloads, full paths, raw tool output, or live host state.

### 3. Positive and negative fixtures are required

Rails that validate data or security boundaries need at least one positive fixture and one negative fixture. Negative fixtures should prove fail-closed behavior.

### 4. Stable rails join Nix checks

A rail starts as a focused local script. Once deterministic and bounded, it can be added to `nix flake check` or a named flake check.

### 5. Closeout evidence is an index, not a claim inflater

Closeout should list successful rails, workspace tests, clippy, Nickel drift checks when applicable, and Cairn gates. Skipped rails must include blockers and next-best validation. Evidence must not imply production readiness beyond the covered contracts.

## Risks / Trade-offs

- Too many rails can slow routine development. Keep each rail narrow and wire only stable rails into the default flake.
- Receipt schemas can churn. Start simple and version them.
- Avoid hidden live dependencies; focused rails must be credential-free by default.