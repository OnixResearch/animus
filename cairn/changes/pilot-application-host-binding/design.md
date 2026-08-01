## Context

Animus already implements the hard parts of an application/platform split. Pure logic lives in reusable cores. Adapter contracts declare component boundaries. Runtime composition uses injected services. Host shells own filesystem, process, network, credential, terminal, persistence, and provider effects.

The session runtime authority profile binds the exact policy, permission, sandbox, model, tool, extension, and context posture for a resumable lineage. It is metadata-only and does not replace current enforcement.

The pilot adds a portable view over those existing facts. It must not create a broader profile or let a portable descriptor authorize host behavior.

## Decisions

### Decision: Pilot one non-default deterministic entrypoint

**Choice:** Use the existing fake-backed headless dogfood entrypoint as the first application unit. The pilot remains non-default and uses injected deterministic services.

**Rationale:** This path exercises real runtime composition and receipts without adding provider, credential, network, process, or external tool authority.

### Decision: Project existing identities

**Choice:** The binding projection carries exact existing identities for the application artifact, entrypoint, schemas, symbolic effects, session runtime authority profile, adapter cohort, resource profile, and policy.

No policy body, prompt, tool payload, credential, endpoint, environment value, or physical path enters the projection.

**Rationale:** The pilot should make composition easier to inspect, not create another source of truth.

### Decision: Consume a frozen Kamacite contract first

**Choice:** Implementation waits for an accepted Kamacite `ApplicationHostBinding` schema. Animus then freezes the exact schema identity, producer revision, canonical fixture bytes, and expected validation receipt.

**Rationale:** Cross-repository fixture conformance avoids an early release dependency and exposes schema drift clearly.

### Decision: One host profile applies to the pilot unit

**Choice:** The pilot binding references exactly one existing session runtime authority profile. Missing, multiple, implicit, or fallback profile selection denies dispatch.

**Rationale:** The effective host contract must be known before the first dependent effect.

This rule applies to the pilot executable unit. It does not require one profile for every process or workflow in the stack.

### Decision: Symbolic effects remain separate from authority

**Choice:** The pilot exports a closed Kamacite-compatible effect summary for the fake-backed entrypoint. The summary classifies requested model and tool operations but grants no host capability.

**Rationale:** Effect visibility helps review. Authorization still comes from the exact intersection of existing Animus policy and host evidence.

### Decision: Commit binding before dependent effects

**Choice:** The host prepares and commits the application-host binding beside the existing session authority binding before model or tool dispatch in the pilot path.

**Rationale:** Later evidence can identify the exact application and host contract that preceded the observed effects.

### Decision: Recheck current enforcement

**Choice:** A matching binding never skips current authorization, capability probes, adapter-cohort checks, tool and provider readiness, credential admission, sandbox preflight, cancellation, or per-run resource checks.

**Rationale:** Binding continuity is identity evidence, not current enforcement evidence.

### Decision: Explanation is read-only and safe

**Choice:** Add a read-only explanation projection that shows safe IDs, effect classes, selected host profile, adapter/resource cohorts, compatibility status, and non-claims.

**Rationale:** Roc's main lesson is that the application/platform boundary should be visible. The explanation output provides that visibility without hidden defaults.

### Decision: Preserve functional core and shell ownership

**Choice:** Projection and comparison logic is pure over typed inputs. File loading, generated config, stdout, profile discovery, and durable writes stay in the host shell and repository check scripts.

**Rationale:** The pilot must not move ambient effects into reusable crates.

## Test Design

Positive tests cover canonical projection, one selected host profile, closed symbolic effects, stable metadata changes, pre-effect commit order, and safe explanation output.

Negative tests cover missing or multiple hosts, open or unknown effects, undeclared operations, stale Kamacite schema, stale session profile, wrong adapter or resource cohort, binding after effect dispatch, unsafe output fields, and authority overclaims.

The focused conformance rail validates the frozen Animus projection with the exact Kamacite fixture contract. It records an exact blocker when the upstream schema is unavailable.

## Risks and Trade-offs

- The pilot can duplicate session profile fields. The projection uses identities and small role labels instead of copying policy bodies.
- The binding can look authoritative. Types, receipts, tests, and docs state that current host admission remains mandatory.
- Kamacite can change before adoption. Exact frozen schema and receipt identities turn drift into a visible failure.
- One pilot may not justify a shared abstraction. Lattice and Aspen adoption stays deferred until measured value exists.

## Claim Boundary

A passing pilot proves that one Animus entrypoint can project existing composition identities into the accepted Kamacite binding shape. It does not prove effect completeness, application correctness, authorization, sandboxing, provider behavior, tool safety, runtime success, or release eligibility.
