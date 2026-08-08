## Why

Animus has process-job profiles, Wasm boundaries, sandbox descriptors, application-host binding work, runtime authority units, and injected executors. It does not define one pure planner that chooses the weakest admitted execution backend for declared workload requirements.

AgentOS provides a useful reference for lightweight in-process execution and explicit escalation to a full sandbox when a workload needs broader operating-system functions. Animus needs a backend-neutral planner without adopting agentOS, V8, a sandbox provider, or hidden fallback.

## What Changes

- Add closed workload requirement and execution backend class vocabularies.
- Add a pure planner that selects among embedded component, guarded local process, isolated host, and external machine classes.
- Require exact backend, adapter, application-host binding, runtime-authority unit, resource profile, effect permission, and evidence identities before execution.
- Deny unsupported combinations and silent downgrade or escalation.
- Require the host to recheck current enforcement and backend availability before effects.
- Expose safe explanation and metadata-only plan receipts for Lattice and other host consumers.

## Impact

- **Files**: adapter DTOs, runtime planning core, Nickel policy, host service contracts, safe receipts, fixtures, docs, and the agent runtime specification.
- **Dependencies**: Reuse application-host binding, runtime authority seals, and bounded execution evaluation instead of defining replacements.
- **Testing**: Positive selection cases plus negative ambiguity, missing capability, unavailable backend, authority drift, silent fallback, and overclaim cases.
- **Boundary**: A selected class is a plan, not evidence that the backend exists, contains hostile code, completed work, or produced a correct result.
