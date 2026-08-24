## Context

Animus already owns model-runtime profiles, provider-neutral request snapshots, provider retry, and separate live provider hosts. The missing seam is an explicit independent identity for the model-facing dialect when one transport serves several families.

`digitallyinduced/haskell-agent` uses provider packages for wire effects and dialect packages for prompt and tool semantics. OpenRouter can therefore transport Codex, Grok Build, or generic Responses models without flattening their model-facing behavior.

## Goals

- Resolve transport and dialect independently for each model target.
- Keep provider effects and dialect projection in separate owners.
- Bind both identities before provider contact.
- Reject incompatible transitions and cross-family leakage.
- Preserve exact per-model prompt, tool, skill, and memory composition.

## Non-goals

- Dynamic dialect inference from responses.
- Provider or model-family semantic equivalence.
- One universal prompt or tool schema.
- Moving authentication or billing authority into reusable runtime code.

## Decisions

### Decision: A model target binds two independent identities

**Choice:** A resolved target contains one provider-transport identity and one model-dialect identity. Both identities are required and versioned. Unknown identities fail closed.

**Rationale:** The same transport can carry distinct model-facing protocols. Combining the identities makes provider routing alter prompt semantics accidentally.

### Decision: Providers own effects and dialects own projections

**Choice:** Provider hosts own authentication, billing, transport, streamed wire decoding, and provider-native continuation. Dialect profiles own prompt assembly, exposed tool descriptors, schema conventions, project-instruction formatting, and subagent protocol.

**Rationale:** This boundary keeps framework and vendor effects outside pure request composition.

### Decision: Request snapshots bind both identities

**Choice:** The complete provider-neutral request snapshot records the provider identity, dialect identity, exact profile inputs, and compatibility decision before provider contact.

**Rationale:** Later evidence must show which model-facing contract and transport produced the request.

### Decision: Provider transitions preserve dialect compatibility

**Choice:** A provider transition can retain a dialect only when the new transport explicitly admits it. An incompatible transition fails closed instead of substituting a generic dialect silently.

**Rationale:** Transport availability must not rewrite the model-facing action language.

## Dependency Direction

```text
provider hosts       dialect profiles
      ^                     ^
      |                     |
      +-- model-target resolver -- request snapshot -- runtime loop
```

Dialect projection remains deterministic. Provider effects stay in host crates.

## Adversarial Audit

The audit must try to show that Animus:

- derives dialect only from the provider name;
- lets a provider host rewrite prompt or tool semantics;
- contacts a provider before both identities are bound;
- silently substitutes a generic dialect during failover;
- leaks Codex, Grok Build, or another family surface into a different target;
- claims semantic equivalence from a shared transport.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Two identities add catalog and fixture surface. They remove ambiguous ownership.
- Compatibility tables can drift as providers add models. Current profile generation and freshness rails must bind the selected cohort.
- Some model families share most schema rules. Reuse must stay explicit and must not erase family-specific behavior.

## Non-Claims

This change does not prove model quality, provider equivalence, prompt effectiveness, tool correctness, wire compatibility beyond tested fixtures, or provider adherence to declared model identity.
