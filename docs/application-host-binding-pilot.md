# Application-host binding pilot

## Status

Animus pilots one application-host binding for the non-default fake-backed headless entrypoint. The pilot is not a default runtime path.

The binding is a read-only composition projection. It does not grant execution authority.

## Frozen Kamacite cohort

The pilot consumes the Kamacite `ApplicationHostBinding` contract at revision `b21732a7458b660367f7281c717a5acc3f8c8190`.

| Artifact | BLAKE3 |
| --- | --- |
| `application-host-binding-v1.preserves` | `34fdebab3c8a274729f364af8da58691dba38809c67d2643c1f0d0354cb0c80b` |
| `application-host-binding-receipt-v1.preserves` | `3b48bf894cf40990a6d857542e9c3b26df2f6b50510999698323a7cb5cbca39e` |
| Closed-effects validation receipt | `75ac779da7b5fde841188ba326e81c253ba5b78dbf0cc632bc90f8b58a9fb303` |
| Kamacite validation core source | `4912fa0d0b81c5ac526d6fd762f4fe83d6d7dbba21cb661f3724bb499af5a1db` |

Animus freezes fixtures before it adds a direct Kamacite crate dependency. Schema drift fails the focused rail.

## Identity mapping

| Kamacite role | Animus source | Mapping |
| --- | --- | --- |
| application artifact | `animus_runtime::dogfood_shell` source artifact | exact BLAKE3 identity |
| executable entrypoint | `animus-dogfood-smoke` | exact BLAKE3 identity |
| input schema | `DogfoodRunRequest` | domain-separated schema identity |
| output schema | `DogfoodReceipt` | domain-separated schema identity |
| effect summary | fake model request plus optional fake tool request | closed symbolic summary |
| host profile | generated session runtime authority profile | exact existing profile identity |
| adapter cohort | session profile `adapter_cohort` component | exact existing identity |
| resource profile | generated process-job profile policy | exact BLAKE3 identity |
| policy | session profile `policy_authorization` component | exact existing identity |

`config/application-host-binding-pilot/profile.ncl` is the typed review source. Its generated projection is `config/application-host-binding-pilot/generated/animus-binding.json`.

The projection contains identities and closed effect classes. It does not copy prompts, policy bodies, tool payloads, credentials, endpoints, environment values, or host paths.

The Kamacite producer envelope is `config/application-host-binding-pilot/generated/animus-producer.json`. It binds implementation revision `58cab252797865df5ccb3c7364f0ad994d4c8ea7`, source-manifest identity `69fb71f312cc479e34aa91f00f01d16a12caf5253bff0cf003076f94d0a52339`, and envelope BLAKE3 `f2e16b86830265f9e64538230e6d20e3614039f72875a191fd715b45dc23981f`.

## Effect and host rules

The pilot declares two symbolic operations:

1. fake model request
2. fake tool request

The set is closed. An open, unknown, duplicate, undeclared, or stale operation fails before dispatch.

The pilot selects exactly one existing session runtime authority profile. Missing, multiple, fallback, or alternative host profiles fail closed.

Symbolic effects do not grant model, tool, provider, filesystem, process, network, credential, sandbox, or mutation authority.

## Pre-effect order

The host must complete these steps in order:

1. Commit the existing session authority binding.
2. Commit the application-host binding projection.
3. Prepare and seal the accepted runtime-authority unit.
4. Recheck current authorization, capability probes, adapter readiness, credential admission, sandbox preflight, cancellation, timeout, and resources.
5. Admit one declared dependent effect.

`prepare_pilot_dispatch` reuses `start_runtime_authority_effect`. A matching binding cannot bypass the existing seal or current recheck.

Changed profile, adapter, resource, operation, binding, or seal identities require a new unit. The active unit cannot widen.

## Explanation

`explain_application_host_binding` returns safe line records. It shows application, entrypoint, effect, host, adapter, resource, compatibility, and projection identities. It also shows all required non-claims and the next focused check.

Denied explanations contain stable issue classes. They do not echo rejected private input.

Run the focused rail:

```sh
scripts/cairn/check-application-host-binding --root .
nix build .#checks.x86_64-linux.application-host-binding --no-link -L
```

## Design reference

The inspection model takes design inspiration from [Roc platforms](https://www.roc-lang.org/platforms): application intent and platform effects are visible as separate roles.

Animus does not add Roc source, the Roc compiler, a Roc runtime, or a Roc platform ABI. The existing Animus policy, adapter, session, seal, and host-shell boundaries remain authoritative.

## Non-claims

A passing pilot proves a bounded projection over one measured Animus and Kamacite cohort. It does not prove:

- effect completeness
- application correctness
- host capability or authorization
- host behavior
- sandbox admission
- provider correctness
- tool safety
- runtime success
- release eligibility
