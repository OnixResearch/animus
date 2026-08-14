# Design: Generate the composition activation graph

## Context

The component catalog is a useful component inventory. The request lifecycle identifies behavioral owners. The composition conformance rail tests selected joins.

A host still needs a complete declaration of which component owns a port, which components provide and consume it, which events cross the seam, which lifecycle owner closes it, and whether activation is required.

## Success Contract

A selected composition root can run one pure preflight over catalog data and current readiness observations. Required closure errors stop startup before external effects. Generated maps show the same source data that preflight validates.

## Decisions

### 1. Extend the existing catalog instead of adding a registry

`.cairn/component-catalog.ncl` remains the source of truth. New typed records name:

- application-owned port identity and owner;
- provider and direct consumer component identities;
- capability family and safe scope class;
- required, optional, or multiple-provider activation policy;
- lifecycle owner and teardown contract;
- event identity, producer, consumer, delivery class, and durability class;
- feature, authority, and focused-check requirements.

The catalog describes composition. It does not instantiate services or grant authority.

### 2. Validate the complete graph deterministically

Validation rejects unknown references, orphan required ports, undeclared consumers, invalid multiplicity, authority mismatch, feature mismatch, missing lifecycle owners, impossible event routes, and unsafe evidence labels.

Existing dependency-cycle rules remain authoritative. Event cycles are allowed only when every edge declares a bounded delivery policy and the owning runtime already admits that cycle.

### 3. Generate three review maps

The generator produces:

1. a component dependency map;
2. a capability-seam map of port, providers, consumers, scope, and lifecycle;
3. an event-ownership map of producer, consumers, delivery, durability, and evidence class.

Generated prose is not a second source of truth.

### 4. Preflight selected compositions before effects

The pure preflight receives selected component IDs, feature facts, authority ceilings, and host-supplied readiness observations. It returns `ready`, `degraded_optional`, or `blocked` with typed issues.

A required provider or consumer may be structurally declared but operationally unavailable. Required unavailable state blocks startup. Optional unavailable state remains visible and cannot be silently substituted.

### 5. Keep boot execution in composition roots

The preflight core performs no I/O. Each product shell gathers readiness through explicit ports and runs preflight before it opens stores, resolves credentials, contacts providers, starts processes, mutates workspaces, enters terminal mode, or exposes a ready endpoint.

### 6. Provide safe inspection without control authority

A bounded activation snapshot contains component IDs, safe classes, generation, status, issue codes, counts, BLAKE3 graph identity, and non-claims. It contains no service handles, raw configuration, credentials, endpoints, private paths, prompts, payloads, or mutation commands.

A later adapter can map this value into the published Host Control inspection envelope.

## Alternatives Considered

### Add a runtime service locator

Rejected because it creates hidden dependencies and ambient lookup. Composition roots continue to pass explicit concrete services.

### Infer providers and consumers from Rust imports

Rejected because imports do not prove runtime selection, authority, scope, or event delivery.

### Make every owner a plugin

Rejected because domain policy, authority, durable ordering, and completion semantics must remain privileged core or application owners.

## Risks and Controls

- Catalog growth can become burdensome. Require records only for genuine external seams, event routes, lifecycle ownership, and product composition.
- Metadata can drift from code. Focused fixtures, public import checks, conformance cases, and generated projection checks remain mandatory.
- A green preflight can be overclaimed. It proves supplied graph closure and readiness facts only.

## Non-Claims

This design does not prove implementation correctness, runtime liveness, event delivery, provider correctness, authority, sandboxing, effect success, production readiness, or release eligibility.
