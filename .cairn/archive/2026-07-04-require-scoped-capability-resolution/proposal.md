# Proposal: Require scoped capability resolution

## Why

Animus already has a runtime capability registry, but dispatch can still admit a model or tool request by checking whether any service of that kind exists. As provider routing, scoped tools, queues, policy gates, and host services grow, service-kind presence is too broad: a request for one model, tool, scope, or policy lane must not be satisfied by an unrelated capability.

This change tightens dispatch admission to resolve the specific required capability label, service kind, and safe scope before adapter calls.

## What Changes

- Extend runtime adapter requests or decision metadata with explicit required capability labels/scopes for model and tool dispatch.
- Replace broad service-kind presence checks with registry resolution of the declared capability.
- Keep compatibility constructors by binding existing model/tool adapters to default model/tool capability labels.
- Add fail-closed diagnostics for absent label, wrong service kind, unsafe scope, denied capability, and compatibility mismatch cases.
- Add tests proving unrelated services cannot satisfy a scoped request and no ambient fallback occurs.

## Impact

- **Files**: runtime request/decision DTOs, capability registry resolution, runtime shell dispatch, compatibility constructors, tests, docs, facade inventory, and Cairn evidence.
- **Testing**: positive default compatibility and explicit scoped dispatch; negative absent/wrong-kind/wrong-scope/duplicate/denied/no-fallback cases.
- **Non-claims**: this does not add provider discovery, model routing, credential lookup, plugin loading, or concrete service clients.
