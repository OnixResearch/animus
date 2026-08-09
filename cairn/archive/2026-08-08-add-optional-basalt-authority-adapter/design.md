# Design: Optional Basalt authority adapter

## Context

Animus owns agent definitions, semantic roles, guarded-action request shapes, capability ceilings, dispatch orchestration, and safe receipts. Basalt owns generic reviewed policy enforcement over contract, resource, ability, and verified UCAN facts. UCAN owns signed principals, delegated capabilities, proof chains, caveats, temporal bounds, and invocation verification.

These owners must remain separate. A convenient integration must not move Basalt types into generic Animus contracts, move agent semantics into Basalt, or make a friendly agent name the authorization principal.

The existing `basalt-reference` feature only exposes the pinned Basalt crate. The existing UCAN-aligned Animus core validates supplied grant-shaped facts, but it does not verify a raw holder-signed invocation through a live host adapter.

## Goals

- Provide one complete opt-in path from an Animus guarded action to a Basalt decision.
- Bind agent-definition identity to a UCAN DID without granting authority through a display name.
- Keep Basalt replaceable through an implementation-neutral authority port.
- Keep all external state and effects in a thin host shell.
- Fail closed for missing, stale, malformed, unsupported, or denied authority inputs.
- Preserve default, `no_std`, Wasm, and unrelated feature isolation.
- Emit safe evidence without bearer tokens, secrets, private paths, or raw effect payloads.

## Architecture

```text
agent definition + semantic role + task context
  -> pure Animus capability intersection
  -> generic HostAuthorityRequest
  -> injected HostAuthorityService
       -> optional animus-std-basalt-authority
            -> pure Animus/Basalt translation core
            -> caller-injected UCAN verification services
            -> Basalt raw UCAN enforcement
            -> pure generic decision projection
  -> existing approval, sandbox, cancellation, timeout, and budget gates
  -> effect dispatch only after every required gate allows
```

### Crate boundary

`animus-adapters` owns implementation-neutral authority DTOs and the injected service contract. These types contain only normalized action, contract, resource, ability, subject, definition, role, task, policy, freshness, and correlation identities.

`animus-std-basalt-authority` is a new optional standard-library host crate. It depends on Animus contracts and the reviewed Basalt revision. It contains a pure deterministic translation and decision-projection core plus a thin shell for Basalt and UCAN calls.

The root `animus` package exposes a new non-default `basalt-authority` feature. The existing `basalt-reference` feature remains a compatibility-only re-export unless a separate accepted change removes it.

`animus-core`, `animus-adapters`, `animus-runtime`, `animus-std`, `animus-steel`, and `animus-tui` do not gain a required Basalt dependency. Generic crates do not expose Basalt or UCAN concrete types in public signatures.

## Decisions

### Decision: Keep the provider port implementation neutral

**Choice:** Animus defines the request, decision, issue, and receipt projection contracts. The optional crate implements them with Basalt.

**Rationale:** This structure lets another host inject a different provider. It also prevents Basalt policy semantics from becoming the Animus authority model.

### Decision: Bind authority to DID and immutable definition identity

**Choice:** An `AgentAuthorityBinding` carries the UCAN audience DID, agent-definition BLAKE3 identity, binding-policy identity, freshness identity, and optional safe display label.

**Rationale:** The name helps discovery and presentation. The DID identifies the cryptographic principal. The definition identity binds the reviewed agent configuration. Changing the name does not widen authority, and changing the DID or definition requires a new admitted binding.

A binding does not prove DID ownership. The holder-signed invocation and current verification inputs establish the bounded invocation fact.

### Decision: Use Basalt raw UCAN enforcement

**Choice:** The adapter uses Basalt's raw holder-signed invocation boundary rather than accepting caller-created `CapabilityGrant` values as live authority.

**Rationale:** The raw boundary already separates policy denial, current holder verification, caveat checks, revocation, replay, and safe receipts. Reusing it avoids a second UCAN verifier in Animus.

The host injects key resolution, proof resolution, revocation state, caveat policy, replay admission, current time, and verification limits. The adapter does not discover or persist these values.

### Decision: Keep translation pure

**Choice:** A pure core converts a validated Animus request and admitted binding into an exact Basalt action, contract, resource, ability, operation profile, and request digest. A second pure core projects Basalt outcomes into generic Animus decisions.

**Rationale:** Mapping errors are authorization errors. Pure functions permit positive, negative, property, and fixture tests without keys, stores, clocks, providers, or processes.

The mapping rejects unsupported action classes, unconfigured contract mappings, wildcard authority, mismatched subjects, stale bindings, unsafe identities, digest drift, and any resource or ability substitution.

### Decision: Basalt is an additive ceiling

**Choice:** A Basalt allow satisfies only the Basalt authority gate. Every other Animus ceiling and gate remains required.

**Rationale:** Basalt policy cannot mint model, tool, filesystem, process, network, Steel, semantic role, workspace, mutation, approval, sandbox, cancellation, timeout, budget, evidence, or completion authority.

A Basalt deny always blocks the guarded effect. A Basalt allow cannot reverse another denial.

### Decision: Integrate at existing effect boundaries

**Choice:** Each live guarded-action path invokes the generic authority service before its host effect. The first implementation inventories model, tool, filesystem, process, network, Steel, and semantic agent-role paths.

**Rationale:** Checking only agent activation leaves later tool and host effects unprotected. Checking only tool execution leaves role assignment and model dispatch unbound.

When an action class has no live Basalt mapping, a host that requires the Basalt provider returns `unsupported_action`. It does not bypass the provider or use a broad fallback capability.

### Decision: Preserve optionality and feature isolation

**Choice:** `basalt-authority` is disabled by default. Selecting it adds only the optional adapter and its explicit facade exports.

**Rationale:** Animus consumers can use no authority provider, a test provider, or another production provider without resolving or compiling Basalt.

Feature checks must prove that default, `no_std`, Wasm, core, adapters, runtime, Steel, TUI, and unrelated standard-library crates do not acquire Basalt, vendored UCAN, token parsing, key access, proof access, replay storage, or network behavior.

### Decision: Keep policy authoring explicit

**Choice:** Animus owns a typed Nickel adapter policy for action-to-contract mapping, supported adapter mode, immutable dependency identity, required non-claims, and named bounds. The runtime consumes a checked generated projection.

**Rationale:** Basalt remains the policy enforcement owner. The Animus policy only configures the adapter and maps Animus action identities to reviewed Basalt contract identities.

The adapter receives an explicit loaded Basalt policy. It does not evaluate Nickel, search for policy files, or silently use Basalt defaults in live operation.

## Request and decision model

A generic host-authority request includes:

- guarded action class, contract, resource, and ability;
- agent-definition, subject, task, actor, role-plan, and session identities when applicable;
- admitted agent-to-DID binding identity;
- policy, grant, freshness, operation-profile, and request-digest identities;
- named verification and output bounds; and
- one safe correlation identity.

The request excludes raw prompts, tool arguments, provider payloads, credentials, tokens, proofs, private keys, full paths, environment values, and effect outputs.

The generic decision contains allowed or denied status, a stable issue class, provider and policy identities, redacted principal and resource hashes, action and ability labels, checked-bound counts, freshness class, replay class, and a BLAKE3 decision identity.

## Ordering

The shell uses this order:

1. Validate the generic request and current agent binding.
2. Build the exact Basalt request through the pure translation core.
3. Run non-consuming policy and verification checks.
4. Perform replay admission as the final consuming authorization step through Basalt.
5. Project and commit the safe authority decision before the dependent effect.

A commit failure prevents dispatch. After replay succeeds, the adapter does not run a fallible widening step. Existing approval, sandbox, cancellation, timeout, budget, and runtime-authority sealing remain separate gates.

## Failure behavior

The adapter fails closed for:

- missing feature, service, policy, binding, token, key, proof, revocation, caveat, replay, time, or limits;
- stale or mismatched agent, definition, DID, subject, task, role, session, policy, grant, or freshness identities;
- malformed tokens, signatures, proofs, capabilities, requests, resources, abilities, caveats, or operation digests;
- denied policy, authority, caveat, revocation, replay, approval, sandbox, cancellation, timeout, or budget state;
- unsupported action or Basalt/UCAN cohort; and
- unsafe receipt fields or uncertain consuming-state outcomes.

No failure falls back to an ambient identity, local allow, broad wildcard, bearer token possession, default Basalt policy, direct `CapabilityGrant`, or another authority provider.

## Evidence and privacy

Receipts contain metadata only. BLAKE3 applies to public stack-owned identity and canonical request bytes. The adapter never hashes bearer tokens, private keys, proof bodies, caveat payloads that can contain secrets, or credential material for evidence.

Receipts reject raw compact tokens, signatures, proofs, public-key bodies beyond approved redacted identities, revocation entries, replay nonces, prompts, tool arguments, provider payloads, environment values, full paths, credentials, or outputs.

An allow receipt proves only that the configured adapter produced an allowed decision for the exact supplied and observed inputs. It does not prove global trust, key truth, revocation freshness beyond the supplied observation, replay durability, sandbox confinement, effect correctness, agent safety, or completion.

## Compatibility and license

The adapter pins one reviewed Basalt revision and checks its vendored UCAN revision, policy schema, raw-enforcement API, receipt schema, and license metadata. An unreviewed Basalt or UCAN cohort is unsupported until compatibility fixtures pass.

Basalt and its UCAN standard-library shell are `AGPL-3.0-or-later`. The optional feature and resulting distribution require explicit downstream license review. The default Animus package graph must remain free of the optional dependency.

## Test design

Positive tests cover deterministic agent binding, exact translation, holder-signed invocation admission, direct and delegated capabilities, every supported action mapping, additive gate composition, safe receipts, and one fake-host end-to-end dispatch.

Negative tests cover name-only authority, DID or definition drift, stale bindings, missing services, malformed tokens, wrong holder, wrong audience, expired tokens, proof failure, revocation, caveat denial, replay denial, policy denial, resource or ability mismatch, wildcards, unsupported actions, another Animus gate denial, receipt leaks, and disabled-feature isolation.

Property tests cover deterministic mapping, order independence where contracts permit it, non-widening projection, exact request binding, and the rule that no denied input can produce an allowed generic decision.

## Risks and trade-offs

- **Dependency license:** Enabling the adapter introduces an AGPL dependency. Feature isolation and explicit license evidence make this choice visible.
- **False authority confidence:** A Basalt allow can look final. Types, gate composition, receipts, and docs mark it as one required decision only.
- **Identity aliasing:** Friendly names can change or collide. Authority uses DID, manifest identity, and binding identity instead.
- **Backend freshness:** Keys, proofs, revocation, replay, and clocks are caller-owned. Missing or uncertain inputs fail closed.
- **Integration drift:** New dispatch paths can bypass the port. A focused structural and conformance rail inventories guarded effects and rejects ungated supported paths.
- **Version drift:** Basalt vendors UCAN. Exact cohort identities and compatibility fixtures block silent upgrades.
