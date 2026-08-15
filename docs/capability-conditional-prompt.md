# Capability-conditional prompt composition

`animus_runtime::capability_conditional` owns an explicit composition invariant: a prompt fragment renders only when every tool or skill it names is available for the current turn.

## Boundary

- `AvailabilitySet` is an explicit composition input. Composition never infers availability from prompt text.
- `compose_capability_conditional` includes a fragment only when all required capabilities are in the union of the tool and skill sets, and records bounded omissions otherwise.
- An empty availability set is an unresolvable capability input and fails closed before any prompt is emitted.
- The mandatory negative fixture proves that a role naming a missing tool never produces instructions for it.
- `build_capability_composition_receipt` emits a metadata-only receipt with included and omitted fragment counts.

## Use

```rust
use animus_runtime::capability_conditional::{
    AvailabilitySet, PromptFragment, compose_capability_conditional,
};
```

Availability resolution, model dispatch, and policy enforcement stay host-owned.

## Non-claims

- This module does not discover tools or skills.
- It does not enforce tool admission or executor readiness.
- Hosts decide whether an omission blocks the turn.
