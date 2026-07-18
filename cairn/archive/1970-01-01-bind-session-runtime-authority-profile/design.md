## Context

The durable-session-store change owns committed framing and lineage. The coding-session facade owns resume ordering and effect admission. The Sandlock host change owns concrete Linux enforcement and capability observations. Per-model profiles, tool catalogs, hook/plugin descriptors, permission policy, and context policy already have separate owners.

This change composes their safe identities without moving their policy or effect authority into the session ledger.

## Decisions

### 1. Bind a reviewed authority profile, not ambient host state

**Choice:** `SessionRuntimeAuthorityProfile` contains canonical safe identities for the active authorization/policy profile, permission posture, requested sandbox descriptor, model runtime profile, effective tool catalog and capability ceilings, admitted hook/plugin/agent-definition set, and context/retrieval policy. Credential reference classes may be represented by safe identities; credential values, endpoint values, environment values, host paths, prompts, and tool payloads are forbidden.

**Rationale:** Resume needs to detect authority-relevant drift without turning the session journal into a secret or configuration dump.

### 2. Keep identity construction pure and domain separated

**Choice:** A pure core validates every component, canonicalizes deterministic ordering, and computes a domain-separated BLAKE3 profile identity. Clocks, random identifiers, filesystem order, locale, process-global state, and physical paths do not participate.

**Rationale:** Separate processes must derive the same identity from the same admitted profile, and unrelated BLAKE3 domains must not be interchangeable.

### 3. Commit the profile before dependent effects

**Choice:** Session creation commits the initial authority profile before the first provider or tool effect. An accepted profile transition is a new durable transaction bound to an explicit branch or successor lineage. Existing committed profile records are never rewritten in place.

**Rationale:** A provider or tool must not observe an authority posture the durable session cannot later identify.

### 4. Treat every profile difference as an explicit transition

**Choice:** The pure resume comparator classifies exact match, changed component, missing component, malformed identity, and unavailable-required-profile outcomes. Automatic continuation is allowed only for an exact requested-profile match. Widening and narrowing both block because either can change transcript/tool semantics. A caller may abandon, fork under the prior profile where still enforceable, or request an explicitly authorized rebind that creates a new durable lineage point.

**Rationale:** Narrower policy is not necessarily replay-equivalent, and silently accepting it can produce confusing partial behavior or invalidate prior assumptions.

### 5. Separate requested profile from realized enforcement

**Choice:** The durable authority profile records requested and admitted intent. Every process launch or resumed run separately records current host capability, adapter cohort, policy authorization, and enforcement observations. Matching requested identity never skips Sandlock capability probes, tool readiness checks, credential admission, provider availability, or other host checks.

**Rationale:** Kernel, adapter, executable, credential, or service state can change while the requested profile remains identical.

### 6. Keep reauthorization in the imperative shell

**Choice:** The pure core returns a transition plan and required authorization bindings. The host shell invokes existing Animus/Basalt authorization, confirmation, capability-probe, store, and adapter services. Profile comparison cannot mint authority, approve a transition, or execute an effect.

**Rationale:** Identity continuity is evidence, not permission.

### 7. Emit bounded receipts and lifecycle events

**Choice:** Receipts and runtime events include status class, changed-component classes, session/branch/head/profile identities, authorization and enforcement observation identities, redaction markers, and a BLAKE3 receipt identity. They exclude raw policy documents, tool schemas, prompts, extension bodies, credentials, paths, environment values, provider payloads, and unbounded diagnostics.

**Rationale:** Operators need to understand why resume was blocked without leaking the values that produced the decision.

## Risks / Trade-offs

- Binding more inputs makes deliberate configuration changes visible and may require more explicit forks or reauthorization.
- Profile equality can be mistaken for current enforcement success; separate per-run observations and non-claims prevent that promotion.
- A profile can grow into a duplicate policy model. The contract therefore stores identities and safe classes only and leaves semantic ownership with existing modules.
- Historical sessions may lack a profile record. They must be classified as legacy/unbound and require explicit migration or safe non-resumable handling rather than an inferred profile.

## Non-Goals

- Storing credentials, raw policy, prompts, tool schemas, host paths, extension bodies, or provider payloads in the session ledger.
- Treating profile identity as authorization, sandbox proof, provider admission, or tool readiness.
- Rewriting historical session records when policy changes.
- Making one global authority profile apply to every host, workspace, session, or branch.
- Claiming that an exact profile match proves behavioral replay equivalence or whole-session correctness.
