# Execution-backend sandbox scope

`animus_runtime::execution_backend_planner` gains a per-component sandbox-scope planner. It names exactly one sandbox descriptor per spawned component under the weakest-admitted rule.

## Boundary

- `plan_sandbox_scope` selects the first descriptor, in caller-supplied weakest-to-broadest order, that satisfies every declared requirement and capability label.
- A component with no declared need receives a minimal admitted scope, never a broad whole-host scope.
- The planner reuses current sandlock descriptor and bubblewrap-guardrail contracts. No new sandbox mechanism is introduced.
- Unknown catalogs, unavailable backends, unsupported requirements, unsatisfiable capability requests, and duplicated descriptors fail closed before effects.
- `project_sandbox_scope_receipt` emits a metadata-only receipt with the selected profile kind and denial class.

## Use

```rust
use animus_runtime::execution_backend_planner::{
    ComponentSandboxRequest, SandboxDescriptor, plan_sandbox_scope,
};
```

The planner names scopes. Live enforcement, host rechecks, and backend availability stay host-owned.

## Non-claims

- Planning only. This does not enforce sandboxing.
- Current-enforcement rechecks remain a separate host behavior.
- Per-component scopes multiply configuration surface; the admitted descriptor catalog keeps it declarative.
