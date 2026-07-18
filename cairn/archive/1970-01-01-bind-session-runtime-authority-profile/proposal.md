## Why

Animus can validate durable session history, rebuild a coding-session reducer, and enforce a Sandlock launch plan, but those surfaces do not yet bind one session lineage to the authority-bearing configuration under which it was created. A resumed session could otherwise observe a changed permission policy, sandbox descriptor, model profile, tool catalog, hook or plugin set, or context policy without an explicit transition.

Grok Build's sandbox-profile pinning demonstrates the value of rejecting silent profile drift across resume. Animus should adapt that mechanism more broadly: commit one metadata-only runtime authority profile to durable session history, compare it through a pure core on resume, and require explicit reauthorization plus a new durable branch/profile binding when the requested authority changes.

## What Changes

- Define a typed session runtime authority profile covering the safe identities of policy, permission, sandbox intent, model runtime profile, tool catalog and capability ceilings, extension posture, and context/retrieval policy. r[agent_harness_runtime.session_authority_profile.contract]
- Compute a domain-separated BLAKE3 profile identity from canonical validated metadata without raw secrets, host paths, prompts, tool bodies, or ambient state. r[agent_harness_runtime.session_authority_profile.identity]
- Commit the initial profile and every explicit rebind as part of durable session lineage before dependent provider or tool effects. r[agent_harness_runtime.session_authority_profile.durable_binding]
- Compare the durable profile with current admitted inputs before resume and block silent widening, narrowing, substitution, missing inputs, or malformed identities. r[agent_harness_runtime.session_authority_profile.resume_admission]
- Keep requested authority identity separate from per-run host capability and enforcement observations, which must be re-probed and re-admitted. r[agent_harness_runtime.session_authority_profile.realized_enforcement]
- Emit metadata-only profile admission and rebind receipts with explicit non-claims. r[agent_harness_runtime.session_authority_profile.receipts]
- Add positive and negative conformance scenarios across the durable store, coding-session facade, model profiles, tool admission, hooks/extensions, context policy, and Sandlock host adapter. r[agent_harness_runtime.session_authority_profile.verification]

## Impact

- **Runtime core**: gains pure authority-profile validation, canonical identity, comparison, and transition planning over in-memory inputs only.
- **Durable store**: records profile bindings as committed metadata linked to the session branch; it does not store credential values or ambient host configuration.
- **Host shell**: gathers already-admitted identities, re-runs required capability probes, executes explicit reauthorization, and persists accepted transitions.
- **Frontends and protocols**: receive blocked, exact-match, or explicit-rebind outcomes and cannot silently rewrite the session profile.
- **Claims**: a matching profile proves identity continuity for supplied metadata only. It does not prove current authorization, sandbox correctness, host capability availability, provider behavior, or whole-session correctness.
