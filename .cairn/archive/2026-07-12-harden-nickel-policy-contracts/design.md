## Context

Animus policy files repeat primitive record contracts while leaving semantic vocabularies and relationships as unconstrained strings or numbers. Later Rust and repository checkers know more than Nickel does: routing thresholds are ordered, job resources are bounded, script hashes have a fixed identity format, one Steel profile is the default, tool descriptors use catalog vocabularies, and dangerous/secret-adjacent tools require stronger approval and redaction.

The goal is to reject invalid reviewed source at export time without moving runtime decisions into Nickel or creating one monolithic policy schema.

## Decisions

### 1. Share primitives, not one giant policy

**Choice:** Add a small shared prelude under `policy/contracts/` for generic predicates and contract constructors. Each policy family keeps its own domain vocabulary and whole-record validator.

The prelude covers non-empty identifiers, exact schema literals, non-negative and positive integers, named bounded integers, lowercase BLAKE3 hex identities, safe repository-relative paths, enum membership, uniqueness, and deterministic validation errors.

**Rationale:** Common primitives should not drift, while family-specific policy remains reviewable next to its consumer.

### 2. Close model-routing vocabulary and ordering

**Choice:** The model-routing contract validates exact schema version, unique role labels, declared downgrade targets, acyclic downgrade relationships, ordered score thresholds, integer non-negative weights, consistent role minima, closed missing-price and budget behaviors, and a well-formed BLAKE3 policy identity.

**Rationale:** A record full of numbers and strings can type-check while describing an impossible or ambiguous routing policy.

### 3. Validate process-job profiles as complete admission inputs

**Choice:** Process-job contracts validate backend membership, unique profile names, exactly one admitted execution form, confined working/writable paths, environment-key prefix policy, unique environment and identity metadata keys, positive resource values not exceeding policy maxima, and explicit notification behavior.

Path and command validation remains lexical and policy-relative. The process shell still resolves paths, opens files, measures time/resources, and executes programs.

**Rationale:** Nickel can reject malformed intent without claiming operating-system confinement.

### 4. Make Steel-runtime profile relationships explicit

**Choice:** Steel-runtime contracts require unique profile names, exactly one enabled default, positive turn budgets, safe script labels, lowercase BLAKE3 script identities, unique closed host-action sets, and closed fallback/redaction modes. Script bytes are rehashed by the shell; a declared hash is not self-authenticating.

**Rationale:** Default selection, action admission, and script identity are whole-policy invariants, not primitive field types.

### 5. Make the tool catalog schema enforce its own vocabulary

**Choice:** Tool descriptors are validated against the catalog's declared runtime-kind, capability, side-effect, approval, redaction, and schema-reference vocabularies. Descriptor names and capability entries are unique. Cross-field rules reject dangerous effects without the required approval and secret-adjacent tools without safe redaction.

**Rationale:** Publishing vocabularies beside a permissive descriptor contract does not make those vocabularies authoritative.

### 6. Projection parity is checked, not assumed

**Choice:** Explicit generation exports validated Nickel to deterministic JSON and records BLAKE3 identities for source, shared prelude, family contract, evaluator/profile, and output. Rust tests deserialize the generated artifact and compare normalized DTO meaning. Default runtime code consumes checked artifacts only and never invokes Nickel.

**Rationale:** Byte freshness catches stale files; DTO parity catches contract/consumer disagreement.

### 7. Failure behavior is fixture-backed

**Choice:** Each family keeps a valid minimal fixture and negative fixtures for each invariant class. The fixture runner evaluates Nickel and also submits malformed external JSON to the Rust boundary where applicable.

**Rationale:** Nickel source rejection and Rust fail-closed handling protect different entry paths and both need evidence.

## Risks / Trade-offs

- Stronger contracts may expose currently tolerated ambiguity and require explicit policy migrations.
- Whole-value validators can become hard to read; generic helpers stay small and family validators return stable issue classes.
- Generated JSON identity does not prove runtime enforcement or script/tool safety; receipts and runtime checks remain authoritative for those claims.
- Duplicating semantic validation in Nickel and Rust can drift. Parity fixtures and generated-artifact checks are therefore required, while runtime Rust remains the final authority for externally supplied artifacts.
