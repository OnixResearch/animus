## Why

Animus binds resumable sessions to exact authority-profile identities. It also rechecks current host enforcement before effects. However, one admitted dispatch can still lack an explicit point after which its effective authority cannot widen.

A sealed runtime-authority unit closes this gap. The host prepares one exact dispatch view, records the observed scope, seals the unit, and only then performs the effect. A later grant or profile change creates a new unit instead of changing the active unit.

Redox namespace sealing informed this mechanism. Animus will adapt the bounded lifecycle rule without adding Redox code, syscalls, schemes, or runtime dependencies.

## What Changes

- Add a pure state machine for preparing, sealing, running, and terminating one authority unit.
- Bind the unit to the current session authority profile, durable binding, enforcement observation, operation set, adapter cohort, and safe runtime-view identity.
- Require a matching host observation before the seal commits.
- Reject added, replaced, implicit, or ambient authority after sealing.
- Require reauthorization or profile changes to create a new unit identity.
- Emit a BLAKE3-bound metadata-only seal receipt before the dependent effect.
- Pilot the contract on one deterministic fake-backed coding-session dispatch.
- Add positive and negative tests plus one focused repository check.

## Impact

- **Runtime core**: pure authority-unit types, transition logic, comparison, and identity construction.
- **Standard host**: a thin shell that prepares injected services, observes the bounded view, commits the seal, and dispatches.
- **Existing contracts**: session authority profiles and current enforcement remain authoritative.
- **Evidence**: safe unit, profile, binding, enforcement, view, operation, and receipt identities.

## Dependencies and Coordination

The change consumes the accepted session runtime authority profile. It does not redefine authorization, sandbox, tool, model, or adapter policy.

The active application-host binding pilot should consume this seal after this change is accepted. It must not add a second seal model.

## Estimate

Expected implementation is five to eight engineer-days after baseline checks pass.

## Out of Scope

- Creating an operating-system namespace or claiming complete host-resource isolation.
- Adding Redox schemes, Redox syscalls, or a Redox runtime dependency.
- Moving filesystem, process, network, credential, clock, or provider authority into reusable cores.
- Preventing a host with equivalent authority from bypassing Animus outside the admitted shell.
- Replacing current authorization, sandbox preflight, capability probes, or per-effect enforcement.
