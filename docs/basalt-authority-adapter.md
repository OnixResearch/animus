# Optional Basalt authority adapter

Animus can use Basalt as an additional deny-capable authority gate for selected host effects. The adapter is non-default. Enable it with `basalt-authority-adapter` only after an `AGPL-3.0-or-later` license review.

## Ownership

`animus-adapters` owns the implementation-neutral `HostAuthorityService` request, decision, binding, gate-intersection, and receipt contracts. These contracts are `no_std` plus `alloc`. They expose no Basalt or UCAN type.

`animus-std-basalt-authority` owns the optional standard-library bridge. It uses Basalt revision `3f1fd1ed63f2b55f07a27f662e5a720518662b7e` with Basalt's default features disabled (the minimal enforcement profile), vendored UCAN revision `927dc32641af6cd1b17202aff44c35acef4540fa`, policy identity `b3:5475d5473ec98cbf052a83512836b0739f5dad3e7b9b0ed201eb3094b1cf39bf`, execution policy identity `b3:c90838cfc1f3c8f5f926f9ee98a05d203d2c0dc2199657514537c37bff3d8c7d`, and UCAN catalog identity `b3:fe77fd63076eaef555b74e702e4c14e483bdba70151ccc955c75b17d97412427`.

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
| `exec` | `process-execute` | `process-execute` | `process/execute` |

All other action, contract, ability, resource, or operation-profile combinations return `UnsupportedAction`. A new action requires a reviewed code and policy change.

`exec` admission additionally requires the request's policy identity to equal Basalt's reviewed process execution policy identity; the packaged default policy cannot authorize execution. The execution resource is `onix://process-execution/` followed by exactly 64 lowercase hexadecimal digits naming one complete consumer-admitted invocation context. The adapter binds the supplied reference through Basalt's exact binder; it does not authenticate the context preimage, executable, argv, environment, inputs, services, resource limits, or their custody. The adopting host owns complete context custody, currentness, sandboxing, and output meaning.

## Guarded-action inventory

The optional adapter supports the document read and document write filesystem paths and the exact `exec` process execution path above. An adopting host must route each such effect through `dispatch_with_basalt` or `dispatch_with_basalt_scoped`.

Model dispatch, tool dispatch, network access, Steel activation, semantic-role assignment, semantic-role activation, handoff, output influence, selection, and veto remain unsupported by this adapter. They keep their existing owners and gates. They cannot use a shared role grant, a filesystem grant, or an execution grant.

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

## Scoped live callback

`dispatch_with_basalt_scoped` adds a borrowed `ScopedAuthority` callback view.
The view exposes the actual verified holder authorization, generic decision,
committed receipt, and validated commit identity. Holder access includes the exact
operation and effective token time bounds. These values do not come from a receipt
parser or a second verification call.

The original API delegates to the same implementation and ignores the view.
Both APIs retain one replay admission before gate intersection and commit.
Neither denied path calls the effect. Effect errors retain the committed receipt.

The view has private construction and no Clone, Debug, or serde surface.
The higher-ranked callback prevents its view and borrowed authority from escaping
through the result. Consumers can copy ordinary metadata, but those copies do not
grant authority. Raw holder details require deliberate access and must not enter
safe receipts by default.

This authority reflects verification time. The callback does not guarantee later
non-revocation, continuous resource custody, or safe execution. Hosts retain later
recheck obligations. Repeating raw holder verification with the same nonce is not
a continuing-authority recheck.

## Evidence

Generic and Basalt receipts contain safe identities, action and ability labels, status, issue class, bounded counts, freshness class, replay class, and redacted principal and resource hashes.

Receipts exclude raw UCANs, signatures, proofs, nonces, keys, credentials, prompts, tool arguments, effect payloads or outputs, environment values, private paths, and unrestricted diagnostics.

Receipts do not prove global trust, key truth, global revocation freshness, replay durability, sandbox confinement, effect correctness, agent safety, completion, or release readiness. Execution-purpose receipts bind only the supplied context reference and policy identity; they never authenticate the invocation context or prove that any process ran. This integration does not create a license compatibility exception.

## Verification

```sh
cargo test -p animus-adapters effect::gate::tests --locked
cargo test -p animus-std-basalt-authority --locked
cargo clippy -p animus-std-basalt-authority --all-targets -- -D warnings
cargo check --no-default-features --locked
cargo check --features basalt-authority-adapter --locked
scripts/cairn/check-basalt-authority-adapter --root .
scripts/config/check-nickel-configs
```
