# Execution backend selection

Animus provides an optional `execution-backend-planner` feature. The pure core selects the weakest backend that satisfies explicit workload requirements and reviewed policy.

## Selection

The policy gives an explicit weakest-to-broadest order for these classes:

1. embedded component;
2. guarded local process;
3. isolated host;
4. external machine.

Code does not assign numeric authority ranks. A workload declares its execution format, filesystem, network, process, terminal, peripheral, persistence, isolation, and resource requirements.

The caller supplies candidate facts and exact adapter, application-host binding, runtime-authority unit, effect-permission, resource-profile, and enforcement-profile identities.

If the selected candidate is unavailable, the planner requires an explicit new plan. It does not silently use a broader backend.

## Current enforcement

A host-owned `CurrentBackendEnforcementService` observes current state. The thin std host seam rechecks all selected identities, availability, resources, and current enforcement before effects.

A valid plan is not current enforcement evidence. The caller still owns effect admission, execution, cancellation, cleanup, and result handling.

## Evidence

Receipts contain BLAKE3 identities, requirement classes, selected classes, denials, counts, and non-claims. They exclude commands, credentials, environment values, host paths, provider payloads, and raw sandbox diagnostics.

Run the focused rail:

```sh
scripts/cairn/check-execution-backend-selection --root .
nix build .#checks.x86_64-linux.execution-backend-selection --no-link -L
```

A plan does not prove backend availability, sandbox enforcement, work completion, output correctness, production readiness, or release eligibility.
