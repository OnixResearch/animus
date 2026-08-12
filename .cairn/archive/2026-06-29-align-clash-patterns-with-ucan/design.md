# Design: UCAN-aligned adaptation of Clash safety patterns

## Context

Clash is a command-line agent safety harness. The parts worth studying for Animus are ergonomic safety patterns: capability categories, explanations for allow/deny decisions, sandbox descriptors, layered policy affordances, and scenario tests around hook inputs. Animus should not adopt Clash's product architecture because Animus is meant to stay embeddable and aligned with Nickel, Basalt/UCAN, typed adapter boundaries, and the Steel runtime fabric.

Basalt's contract model is the intended authority shape: Nickel exports reviewed policy contracts, Rust enforces contract/resource/ability checks, UCAN grants authorize the same resource and ability, and receipts carry deterministic evidence. The Clash-inspired pieces must therefore be translated into caveats and execution constraints around that authority model.

## Decisions

### 1. Clash is a bounded reference, not a dependency plan

**Choice:** Use Clash only as a source of patterns for capability taxonomy, explain traces, sandbox descriptors, and scenario tests. Do not import the Starlark DSL, `clash-policy`, brush shell, hook plugins, editor integrations, installation flows, or audit schema into Animus.

**Rationale:** Animus already has Nickel, Steel, Preserves, adapter contracts, and Basalt/UCAN references. Pulling in Clash architecture would duplicate those boundaries and would make authority depend on precedence rules that are not UCAN delegation.

### 2. Guarded actions are UCAN capability requests

**Choice:** Represent each guarded runtime action as a typed capability request with `contract`, `resource`, and `ability` fields. Examples include model calls, tool invocations, filesystem reads/writes, process execution, network access, and Steel evaluation.

**Rationale:** Clash's `exec/fs/net/tool` categories are useful vocabulary, but Basalt/UCAN needs resource/ability authorization. A typed request lets local policy and runtime dispatch talk about the same action without granting ambient authority.

### 3. Local layers are caveats, not authority

**Choice:** User, project, session, and harness/default policy layers may narrow, deny, or prompt for actions. They may not expand authority unless a verified UCAN grant is attached to the session/request.

**Rationale:** Clash uses layer precedence for policy ergonomics. UCAN uses delegated authority and attenuation. Animus should preserve the ergonomics while making the security rule monotonic: local policy can reduce authority, but only UCAN can prove authority.

### 4. Sandbox descriptors are execution constraints

**Choice:** Sandbox descriptors may be attached to authorized actions as required execution constraints. Runtime dispatch must fail closed when a required sandbox cannot be enforced or when the sandbox descriptor is malformed.

**Rationale:** Sandboxes are valuable defense-in-depth, especially for process execution, filesystem, and network effects. They are not authorization by themselves and must not substitute for contract and UCAN checks.

### 5. Receipts carry safe authority evidence

**Choice:** Authorization and dispatch receipts must use safe metadata: status, issue code, contract id, resource/ability labels or redacted hashes, policy hash, checked UCAN grants, caveat decisions, sandbox descriptor hash, and BLAKE3 receipt hash. Raw prompts, provider payloads, credentials, full paths, raw tool inputs, and unbounded outputs remain excluded.

**Rationale:** Clash's decision traces are useful for explainability, but Animus receipts must align with Basalt's evidence posture and the existing Animus receipt-redaction direction.

### 6. Basalt remains a compatibility reference unless licensing is intentionally revisited

**Choice:** Keep the runtime contract shape compatible with Basalt/UCAN semantics while avoiding a mandatory Basalt dependency in core crates. Optional feature smoke checks may continue to prove compatibility.

**Rationale:** Basalt is AGPL-licensed and already optional in the root package. Animus can implement compatible DTOs and receipts without turning the core runtime into an AGPL-coupled dependency graph by accident.

## Risks / Trade-offs

- A Clash-inspired policy engine can drift into precedence-as-authority. The UCAN requirement must stay explicit in tests and docs.
- Too much sandbox detail can pull OS-specific enforcement into reusable runtime crates. Keep kernel enforcement in the standard shell or optional platform adapter.
- Receipt fields can leak data if copied from Clash audit summaries. Start with hashes, counts, labels, and explicit redaction markers.
- Basalt compatibility may be mistaken for a required dependency. Keep dependency decisions explicit and feature-gated.
- Capability naming can ossify early. Use stable contracts for core actions and add new abilities only with positive and negative tests.
