# Generation-fenced component scopes

Animus uses explicit component scopes for registration, resolution, and teardown. The pure core is `animus_session_core::component_scope`. `animus-runtime` re-exports this module.

<!-- r[impl agent_harness_runtime.component_scope.registration] -->
## Registration

A scope binds these values:

- one safe scope identity;
- one non-zero generation;
- one admitted activation-graph identity;
- an optional exact parent scope;
- canonical component and child-scope registrations;
- explicit dependency member identities;
- local-only or descendant visibility;
- required evidence non-claims.

A component member uses its catalog component identity as its member identity. A child-scope member uses a separate safe identity. Each child scope names its owning component.

Admission checks every component against the validated activation graph. It also checks each dependency against declared port or event relations. An owned child can depend on its owning component.

A component lease binds the scope, generation, component, registration, and lease identities. It grants no model, tool, filesystem, process, network, credential, persistence, terminal, or mutation authority.

<!-- r[impl agent_harness_runtime.component_scope.resolution] -->
## Explicit resolution

Resolution always receives the current scope and an exact target. A child can resolve one parent registration only when both conditions are true:

1. The caller supplies the exact parent scope.
2. The parent registration permits descendant visibility.

Ambient lookup fails. Name-only lookup fails. Hidden ancestor traversal fails. A stale generation fails. Resolution also fails after close starts.

A valid lease does not bypass existing runtime-authority seals. The later effect still needs its normal authority checks.

<!-- r[impl agent_harness_runtime.component_scope.teardown] -->
## Pure teardown reducer

The core owns this state sequence:

```text
open -> closing -> quiescing -> releasing -> closed
                    \-> failed_cleanup
```

The first close request revokes new resolution. It then requests delivery closure. An exact delivery-close observation causes cancellation requests and quiescence checks.

The reducer derives release order from admitted dependencies. It rejects ownership cycles. It releases child scopes and dependents before their dependencies.

A wrong-generation, unknown, duplicate, forged, or out-of-order observation cannot advance the state. Clean closure requires exact quiescence, release, and terminal receipt observations for all members.

The reducer performs no cancellation, waiting, listener, task, process, filesystem, network, clock, persistence, provider, tool, or terminal effect.

<!-- r[impl agent_harness_runtime.component_scope.concurrency] -->
## Repeated close requests

The first accepted close request fixes one BLAKE3 plan identity and release order. Later close requests return that same plan and no new effects.

A repeated request cannot restart teardown. It cannot restore visibility. It cannot change release order or hide unresolved cleanup.

<!-- r[impl agent_harness_runtime.component_scope.evidence] -->
## Host shell and evidence

`animus-local-daemon::ComponentScopeHost` executes only reducer-requested effects. The caller injects five narrow ports:

- delivery close;
- cancellation request;
- quiescence observation;
- member release;
- terminal receipt commit.

An unresolved delivery, cancellation, quiescence, or release request becomes `failed_cleanup`. The shell commits a metadata-only receipt for that outcome. It does not report clean closure.

Receipts contain scope, generation, graph, plan, member, transition, count, cleanup-class, and non-claim metadata. They omit handles, commands, paths, payloads, prompts, credentials, endpoints, service values, and raw errors.

The receipt does not prove task cancellation, process death, resource reclamation, sandboxing, external liveness, production readiness, or release eligibility.

## Local daemon mapping

One worker generation maps to one root component-scope plan. A client attachment gets a separate delivery lease for that worker generation.

Client detach removes only the client delivery lease. It does not own or close the worker scope. Authorized worker stop remains a separate operation.

The live terminal activation profile now includes `runtime.component-scope-core`. This check binds the terminal and daemon composition before terminal or connection effects.

## Reviewed source

The design review used DeepSeek Harness commit `47f943859bef60e4160492346772ded9b24f765a`. The reviewed source has an MIT license and developer-preview status.

Animus adapted explicit scoped registration and disposable lifecycle ordering. Animus did not adopt Cordis ambient contexts, hidden parent fallback, or plugin-owned policy authority.

## Baseline evidence

Before the change, the session core and local daemon tests passed. The agent-control, daemon-worker, transactional-handoff, component-catalog, runtime-facade, and evidence-safety rails also passed.

The first combined baseline run overlapped the new local path dependency. Its final locked check reported the expected stale `Cargo.lock`. Cargo regenerated the lock before post-change checks.

The repo-wide pinned Nix formatter still reports existing `flake.nix` drift. All changed Rust files pass direct `rustfmt`, focused Clippy, and workspace checks.

<!-- r[impl agent_harness_runtime.component_scope.verification] -->
## Verification

Run the focused rail:

```sh
scripts/cairn/check-component-scopes --root .
```

The rail covers registration, local and parent resolution, no-authority leases, cancellation requests, quiescence, reverse release, repeated close, worker mapping, and client detach.

Negative cases cover stale generations, ambient lookup, out-of-scope lookup, post-close lookup, late delivery, wrong observations, cycles, duplicate release, unresolved cleanup, and receipt leaks.

The local rail runs no-std Wasm compilation and the related agent-control, daemon-worker, handoff, activation, and catalog checks. The Nix check runs a sandbox-safe static subset.
