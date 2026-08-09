# Optional Basalt authority adapter

Animus can use Basalt as an additional deny-capable authority gate for selected host effects. The adapter is non-default. Enable it with `basalt-authority-adapter` only after an `AGPL-3.0-or-later` license review.

## Ownership

`animus-adapters` owns the implementation-neutral `HostAuthorityService` request, decision, binding, gate-intersection, and receipt contracts. These contracts are `no_std` plus `alloc`. They expose no Basalt or UCAN type.

`animus-std-basalt-authority` owns the optional standard-library bridge. It uses Basalt revision `e9891e32dcdf655c6e12d0bea2a420f7cdca31c7`, vendored UCAN revision `34449ba1f1a9ef991225e4e6683b5d6aca8c07cf`, policy identity `b3:5475d5473ec98cbf052a83512836b0739f5dad3e7b9b0ed201eb3094b1cf39bf`, and UCAN catalog identity `b3:3266e7b1f2ce66f2ab5c8cbd7028f8bcf3502c7e64fe8d058c7b7e1f762dedb6`.

The host supplies verification time, key resolution, proof storage, revocation checks, caveat policies, replay admission, verification limits, holder-invocation limits, and durable safe-decision commit. The adapter does not discover or create these services.

## Agent identity binding

An agent display name is informational. It never selects a key or grants authority.

Before a request reaches Basalt, the host admits an exact binding between:

- the stable BLAKE3 agent-definition identity;
- one UCAN audience DID;
- the binding policy and freshness identities;
- the subject, task, actor, semantic role-plan, and session identities.

Stale, duplicate, substituted, or mismatched bindings fail before provider work. The Basalt shell compares the expected audience DID before raw holder verification. Basalt then verifies the token audience, holder signature, exact request, operation profile, scope, proofs, revocation facts, caveats, time, limits, and final replay admission.

## Supported actions

The mapping table is closed:

| Animus action | Basalt action | Contract | Ability |
| --- | --- | --- | --- |
| `filesystem` | `document-read` | `document-read` | `document/read` |
| `filesystem` | `document-write` | `document-write` | `document/write` |

All other action, contract, ability, resource, or operation-profile combinations return `UnsupportedAction`. A new action requires a reviewed code and policy change.

## Guarded-action inventory

The optional adapter supports only the document read and document write filesystem paths above. An adopting host must route each such effect through `dispatch_with_basalt`.

Model dispatch, tool dispatch, process execution, network access, Steel activation, semantic-role assignment, semantic-role activation, handoff, output influence, selection, and veto remain unsupported by this adapter. They keep their existing owners and gates. They cannot use a shared role grant or a filesystem grant.

## Dispatch order

`dispatch_with_basalt` performs this order:

1. Validate the generic request and current agent binding.
2. Translate through the closed action table.
3. Verify the raw holder invocation with caller-owned Basalt services.
4. Keep Basalt live authority in scope.
5. Intersect the allow decision with every existing Animus gate.
6. Commit the metadata-only safe decision through the injected commit service.
7. Call the guarded effect.

Basalt cannot widen agent-definition, semantic-role, parent, task, model, tool, workspace, approval, sandbox, cancellation, timeout, budget, or runtime-authority-seal limits. Any denial stops the effect.

## Evidence

Generic and Basalt receipts contain safe identities, action and ability labels, status, issue class, bounded counts, freshness class, replay class, and redacted principal and resource hashes.

Receipts exclude raw UCANs, signatures, proofs, nonces, keys, credentials, prompts, tool arguments, effect payloads or outputs, environment values, private paths, and unrestricted diagnostics.

Receipts do not prove global trust, key truth, global revocation freshness, replay durability, sandbox confinement, effect correctness, agent safety, completion, or release readiness. This integration does not create a license compatibility exception.

## Verification

```sh
cargo test -p animus-adapters host_authority --locked
cargo test -p animus-std-basalt-authority --locked
cargo clippy -p animus-std-basalt-authority --all-targets -- -D warnings
cargo check --no-default-features --locked
cargo check --features basalt-authority-adapter --locked
scripts/cairn/check-basalt-authority-adapter --root .
scripts/config/check-nickel-configs
```
