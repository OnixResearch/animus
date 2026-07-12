# Clash patterns through UCAN authority

Animus uses Clash as a bounded safety-harness reference for capability taxonomy, decision explanations, sandbox descriptors, and scenario-style fixtures. Animus does not adopt Clash's Starlark policy language, full policy engine, brush shell, plugin machinery, editor/LSP surfaces, installer flows, audit schema, or highest-precedence allow semantics.

Guarded actions are represented as UCAN-first capability requests with a contract, resource, ability, and guarded-action class. Nickel-owned fixtures under `policy/capability/` define stable resource and ability names for model, tool, filesystem, exec, network, Steel, and semantic agent-role actions. Role abilities cover separate assignment, activation, handoff, candidate/review/verification/judgment/coordination/observation emission, selection, and veto requests. Rust tests consume the checked generated JSON as review evidence; runtime authorization code does not evaluate Nickel.

Agent-role resources bind the semantic profile, subject/task, actor and parent lineage, model plan, policy/grant, separation, freshness, output contract, and action-specific identities into scoped BLAKE3-backed resource strings. Assignment and activation are distinct grants. Neither grant mints model, tool, filesystem, process, network, Steel, workspace, mutation, verifier, output, selection, veto, or completion authority; each later influence requires its own matching request and receipt.

Local user, project, session, and harness policy layers are caveats. They may allow an already-authorized request, deny it, or ask for a bounded session grant. They do not expand authority by themselves: a local allow without a matching UCAN grant still fails closed before model, provider, tool, process, network, filesystem, or Steel dispatch.

Sandbox descriptors are execution constraints attached to authorized actions. They are represented by BLAKE3 descriptor hashes in receipts and must fail closed when malformed or unavailable. A sandbox descriptor is defense-in-depth, not proof of authority.

Authorization receipts expose only safe metadata: status, issue code, action class, contract id, ability, BLAKE3 resource hash, policy hash, checked grant count, caveat counts, optional sandbox descriptor hash, and receipt hash. Raw prompts, provider payloads, credentials, full private paths, raw tool bodies, raw model outputs, and unbounded outputs remain excluded.

Run the focused rail:

```sh
scripts/cairn/check-clash-ucan-authority --root .
```

The rail validates Nickel export drift, positive authorization fixtures, negative fail-closed fixtures, sandbox constraints, and receipt leak rejection without credentials or live services.
