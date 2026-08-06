# Change: Add an optional Basalt authority adapter

## Why

Animus already has typed agent definitions, capability-bound semantic roles, generic guarded-action requests, and UCAN-aligned authorization fixtures. Basalt already enforces reviewed contract, resource, and ability policy against verified UCAN authority.

The current Animus integration stops before a live host boundary. The root crate exposes Basalt only as an optional reference dependency, and no packaged adapter binds an agent definition to a UCAN principal, runs current Basalt enforcement, and gates host dispatch.

Animus needs one opt-in adapter that connects these existing boundaries without making Basalt a default dependency or the built-in authority model. Hosts that do not select Basalt must keep the current dependency, build, and authority posture.

## What Changes

- Add an implementation-neutral Animus host-authority port over existing guarded-action request and safe decision types.
- Add an optional `animus-std-basalt-authority` crate with a pure translation core and a thin imperative verification shell.
- Bind a stable agent-definition BLAKE3 identity to a UCAN audience DID through explicit host-admitted data. Keep the user-facing agent name non-authoritative.
- Use Basalt raw UCAN enforcement with caller-injected key, proof, revocation, caveat, replay, time, and limit services.
- Treat Basalt as an additive deny-capable authority provider. It cannot widen agent-definition, role, task, model, tool, workspace, approval, sandbox, cancellation, timeout, or budget ceilings.
- Gate each supported live guarded-action dispatch before its effect. A required but absent, stale, malformed, denied, or unsupported adapter fails closed.
- Keep Basalt and its UCAN dependency behind a non-default Cargo feature. Preserve default, `no_std`, Wasm, core, adapter, runtime, Steel, TUI, and unrelated host builds without Basalt.
- Add BLAKE3-bound metadata-only receipts, positive and negative fixtures, compatibility checks, license-boundary checks, and focused Cairn evidence.

## Non-Goals

- Adding agent definitions, role semantics, tool policy, or harness-specific resource construction to Basalt.
- Making an agent display name, role label, prompt, or manifest text an authority principal.
- Adding built-in key discovery, proof fetching, revocation storage, replay storage, ambient clocks, credential lookup, networking, or persistence.
- Replacing Animus local policy, capability intersections, approvals, sandbox admission, effect validation, or completion gates.
- Enabling Basalt by default or requiring it for hosts that inject another authority provider.
- Claiming global trust, complete authorization, sandbox confinement, provider correctness, agent safety, or production readiness.

## Impact

- **Affected spec:** `agent-harness-runtime`
- **Affected crates:** `animus-adapters`, a new optional `animus-std-basalt-authority` crate, and the root compatibility facade
- **Affected host paths:** existing guarded model, tool, filesystem, process, network, Steel, and semantic agent-role dispatch points where live host effects exist
- **Configuration:** typed optional adapter policy and deterministic generated projection, with no runtime Nickel evaluation
- **Dependencies:** the reviewed immutable Basalt revision and its reviewed vendored UCAN boundary remain optional
- **License:** Basalt remains `AGPL-3.0-or-later`; default Animus crates and features retain their current license and dependency boundaries
- **Testing:** focused adapter, feature-isolation, fail-closed, receipt, compatibility, license, workspace, and Cairn rails
