# Serving readiness and request admission

Animus screens named model routes against host-supplied serving readiness and capacity, then dispatches only with a serving-owner grant confirmed by a current recheck. Workers and serving deployments keep separate lifecycles. A CPU-hosted agent can call a remote model without a device allocation, many sessions can share one deployment, and one agent can bind several named model services. This resolves the deterministic part of [Animus #17](https://github.com/OnixResearch/animus/issues/17). Lattice #24 and #13 consume it.

The change package is `.cairn/changes/consume-serving-readiness-admission/`.

## Pure core: `animus_adapters::serving`

The core is `no_std`. It reads no clock, network, credential, or health probe. Every time value is a logical tick that the host supplies.

| Step | API | Contract |
|---|---|---|
| Snapshot | `AvailabilitySnapshot::new(&SnapshotDraft)` | `animus.serving-availability-snapshot.v1`. A snapshot binds the source id, source generation, sequence, and observation window. Each service carries its endpoint, deployment, and generation. It also carries the exact model and configuration, attested as `claimed` or `observed`. Readiness is one of `absent`, `unknown`, `loading`, `ready`, `draining`, `unavailable`, or `stale`. Each service also lists capabilities, its request envelope, and capacity observations with kind, unit, scope, origin, and freshness. Limits: at most 64 services, 16 capacity observations each, and tokens of at most 256 bytes. References are `blake3:` plus 64 lowercase hex characters. |
| Profile | `Profile::new(ProfileDraft)` | `animus.serving-capacity-profile.v1`. The profile selects the identity rule (`claimed-accepted` or `observed-required`), the capacity rule (`known-required` or `owner-admission-decides`), the replica rule (`same-model-allowed` or `pinned`), and the maximum snapshot age. Serving-owner admission is always required. |
| Eligibility | `screen_routes(&ScreenRequest)` | Excludes a route whose snapshot is stale, out of order, or too old. Also excludes routes that are not ready or serve the wrong model or configuration. Under the observed rule, a claimed-only identity is excluded. Privacy, capability, envelope, rollout, and missing admission offers exclude too. Without capacity evidence, the route is excluded under `known-required`; under `owner-admission-decides`, the owner decides at admission. `eligible_catalog()` feeds the existing `select_recipe` and `select_recipe_with_rotation`. `rank_eligible` applies a learned preference only to eligible routes. |
| Admission | `admission_query`, then `gate_dispatch(&GateInput)` | Dispatch needs four things: the existing Mesh decisions; a request-scoped grant for the exact request, deployment generation, model, and configuration; a current recheck of endpoint, generation, model, and readiness; and an unexpired grant from the current owner admission epoch. The verdict is `Admitted`, `Rejected`, or `Blocked`. |
| Receipt | `project_receipt(&ReceiptInput)` | `animus.serving-invocation-receipt.v1` with identity domain `animus-serving-receipt-v1`. The receipt records the caller, the named route and its caller-owned `route_ref`, the deployment, the model and its attestation, capacity evidence, admission, the replica and whether it changed, the outcome class, response provenance, usage provenance, an evidence label, and the non-claims. |

Free memory, queue depth, host probes, and a model listed by `/v1/models` never count as request capacity. A snapshot's capacity is evidence, never a reservation. When two callers both see the last slot, only the serving owner's grant decides which one dispatches.

Outcome classes keep refusals before dispatch (`not-dispatched`) apart from results after provider contact. After contact the class is `completed`, `response-rejected`, or `uncertain`. An uncertain dispatch is never retried or replayed here.

A route may name a `RolloutRequirement`. It is then eligible only when its rollout facts match the requirement. Those facts are the qualification for the served configuration, base model, tokenizer, adapter, an observed loaded policy generation, probability semantics, and operations. The grant, the recheck, and the response must report the same generation. A response that omits the generation or reports another one is recorded unchanged. Ordinary routes never acquire these requirements.

## Host path: `serving-admission` feature

`animus_std::serving::invoke` runs one invocation through these steps:

1. Screen the routes.
2. Select one through the existing recipe selection, with optional rotation.
3. Call `OwnerPort::admit`.
4. Call `OwnerPort::recheck`.
5. Run the gate.
6. Send through the existing Mesh shell (`animus_std::mesh::execute`).
7. Record response provenance.
8. Project the receipt, bound to the existing Mesh receipt identity.

The transport reports usage through `TransportOutcome::facts` (`ResponseFacts`). The standalone `UnavailableOwner` blocks every protected dispatch before any provider effect.

`FixtureOwner` holds fixed slots and replays scripted replies. It is for deterministic tests only, and its results are fixture evidence, not live evidence.

The JSON boundary is `parse_snapshot`, `parse_profile`, and `receipt_json`. It denies unknown fields and caps snapshots at 256 KiB. The reviewed profile is authored in `config/serving-capacity-profile.ncl` and exported to `config/generated/serving-capacity-profile.json`.

Run the focused rail:

```sh
scripts/cairn/check-serving-admission --root .
nix build .#checks.x86_64-linux.serving-admission --no-link -L
```

## Non-claims

Receipts carry `not-gpu-scheduling`, `not-production-availability`, `not-physical-placement-truth`, `not-model-correctness`, `not-capacity-reservation`, `not-provider-honesty`, and `not-cost-or-pricing`.

Usage counters are reported by the provider through the injected transport, and the core does not verify them. Animus does not discover peers, allocate devices, load or evict models, probe health, or schedule a fleet.

Passing fixtures does not establish GPU scheduling or production availability. No live endpoint has been qualified. A live result must carry the `live-endpoint` label and is not live evidence until it has been run against a selected endpoint.
