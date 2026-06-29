# Design: Nickel-governed Steel runtime policy

## Context

Clankers treats Steel as a constrained planner: Nickel declares the profile and Rust remains the authority boundary. Animus should do the same, but limit the first policy to the existing Steel loop fabric and adapter decisions.

## Decisions

### 1. Nickel is the source of truth

`policy/steel-runtime/` will define a typed Nickel contract for Steel runtime profiles. The schema includes profile name, enabled/default state, script bindings, BLAKE3 hashes, runtime budgets, allowed host actions, fallback mode, and receipt redaction fields.

### 2. Runtime consumes checked data only

Rust may consume a generated JSON export or in-test typed fixture, but it must not evaluate Nickel during turn execution. Drift checks prove generated data matches Nickel source.

### 3. Policy is enforced before Steel evaluation

Activation validates selected profile, script path, script hash, budget, host-action allowlist, and fallback mode before loading or evaluating Steel. Denial returns a typed policy failure and prevents model/tool adapter calls.

### 4. Fallback is explicit

A profile may choose `block` or a bounded Rust-native fallback. Fallback must not widen authority or ignore receipt redaction.

### 5. Receipts are redacted

Policy/activation receipts expose schema, status, profile, script hash, policy hash, denied reason, fallback decision, and receipt hash only. They omit raw prompts, provider payloads, credentials, script source, and tool bodies.

## Risks / Trade-offs

- Generated policy JSON can drift; add a focused drift check before runtime depends on it.
- Script-hash enforcement can be noisy during development; require explicit export/update commands.
- Policy shape should stay narrow until a real host-action ABI exists.