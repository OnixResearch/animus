# Design: Require scoped capability resolution

## Context

The archived capability-registry change introduced typed descriptors and a host-supplied registry. The next hardening step is making dispatch depend on a particular required capability instead of the presence of any model or tool service. This matters once a host registers multiple model lanes, tool families, policy scopes, or tenant/workspace-specific adapters.

## Decisions

### 1. Dispatch requests carry required capability identity

**Choice:** Model and tool dispatch requests should carry, or be paired with, a required capability descriptor containing a stable label, service kind, and optional safe scope.

**Rationale:** Runtime admission can then prove it is calling the intended injected service rather than any service with the same broad kind.

### 2. Registry resolution is exact and typed

**Choice:** Dispatch uses `resolve(label, service_kind)` and safe scope checks instead of `contains_service(service_kind)`. Service-kind presence remains useful for diagnostics but not as authorization to dispatch.

**Rationale:** Exact resolution prevents accidental authority widening.

### 3. Compatibility defaults are explicit

**Choice:** Existing `RuntimeAdapters::new(model, tool)` binds model/tool adapters to default labels and no scope. Decisions that omit explicit requirements use those defaults only through documented compatibility helpers.

**Rationale:** Compatibility remains simple but reviewable.

### 4. Scope text remains safe evidence only

**Choice:** Scope values are safe labels, not raw paths, credentials, provider payloads, tenant secrets, or private prompt fragments. Unsafe scope text fails before registry insertion or dispatch.

**Rationale:** Scope is useful for receipts and policy, but it must not leak authority-bearing data.

## Risks / Trade-offs

- Adding capability metadata to requests can touch public APIs. Use compatibility constructors and migration docs.
- Multiple scopes can become policy logic. Keep registry resolution pure and leave policy decisions to injected policy gates.
- Hosts may need to name capabilities explicitly. Provide stable default labels and examples.
