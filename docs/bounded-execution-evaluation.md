# Bounded-execution feasibility evaluation

<!-- r[impl agent_harness_runtime.bounded_execution_evaluation.inventory] -->
<!-- r[impl agent_harness_runtime.bounded_execution_evaluation.decision] -->
<!-- r[impl agent_harness_runtime.bounded_execution_evaluation.authority] -->
<!-- r[verify agent_harness_runtime.bounded_execution_evaluation.non_claims.present] -->

## Result

The terminal status is `blocked`.

A small outcome vocabulary remains promising. The reviewed consumers do not share lossless mandatory semantics for signals, cancellation, network enforcement, filesystem confinement, timeout, output completeness, and cleanup.

No repository extraction, dependency, DTO move, or consumer cutover is approved. The next smallest action is an outcome-only dual run in Octet and Nickel Export. It must exclude sandbox capability claims.

## Search contract

The goal was a checked extraction feasibility decision, not a preferred architecture. Completion required an exact source cohort, typed matrix, positive and negative fixtures, deterministic decision, metadata-only receipt, and adversarial claim audit.

False completion included model agreement, similar type names, a tiny common success record, unchecked task boxes, or a successful command without source and claim evidence.

The review used four source repositories, three serial lenses, two implementation and validation rounds, and no network retrieval. The serial passes are correlated because no subagent consent was available.

Allowed outcomes were `extract`, `incubate`, `blocked`, or an exact evidence blocker.

## Reviewed cohort

| Consumer | Revision | Reviewed source BLAKE3 | License | Retained authority |
| --- | --- | --- | --- | --- |
| Animus | `0c1f6dce0ed11b4b848df0b4f56cac49bd8c19ab` | `196c9a78574536f5012995a386f6fbef87fd95ce3ea9b5a73dfdaa90384345b3` | MIT OR Apache-2.0 | Process-job policy, sandbox descriptors, and host adapters |
| Octet | `d87153a1bbfe4c3469b2dee6fb5512eafa812d88` | `388f36d5f7aebeef6d46a7f897b83d7a8b9393981a8712df366b8426ee770e59` | AGPL-3.0-or-later | Proof selection, theorem admission, proof execution, and proof receipts |
| Nickel Export | `0de12cd4400cf0df6f4f66b235d5dba9ac0b2870` | `51f81fcb325517b62f644ce81577bf76d4f6cda72c75fe1bedc80612f6f38a17` | MPL-2.0 core; AGPL-3.0-or-later shell | Evaluator capture, import observations, freshness, and publication |
| Mantle | `3854efbd7ed144ecafabd694a3913c5470cb7180` | `46d228d83f855fe9e3f8d3ea8bf434c8a42dabd0bc52fbcaf1f65a7edf9314e8` | AGPL-3.0-or-later | Build sandbox, store, scheduler, cache, build, and release policy |

The source BLAKE3 values bind the selected documents and implementation files listed in the replay section. They are not repository-tree identities.

## Inventory and matrix

The typed source is `config/bounded-execution-evaluation/profile.ncl`. Its checked projection is `config/bounded-execution-evaluation/generated/profile.lines`.

Each consumer has one row for environment, filesystem root, network, timeout, output, cancellation, signal, and child cleanup. States remain distinct: requested, configured, enforced, observed, unavailable, unsupported, and not applicable.

Disposition values are lossless, consumer extension, mechanism fact, retained authority, interoperability metadata, or blocker.

The shared outcome candidates are success, nonzero exit, launch denial, missing executable, policy rejection, timeout, cancellation, signal termination, output overflow, cleanup failure, and unsupported mechanism.

## Mechanism findings

### Animus

Animus resolves backend-neutral process-job profiles and portable sandbox descriptors. It does not execute the selected request in its pure runtime core.

Filesystem roots, environment construction, clocks, output capture, cancellation, signals, and cleanup remain host-owned. A plan is not an execution observation.

### Octet

The reviewed proof rail clears and rebuilds the environment, uses a fresh work root, mounts its provider closure, bounds output, uses a timeout, and denies networking through a packaged mechanism.

Those facts are scoped to proof-runner selection and evidence. They do not define general product execution or sandbox equivalence.

### Nickel Export

Nickel Export captures declared files in a private snapshot, removes ambient evaluator environment variables, bounds streams and lifetime, and kills and reaps a timed-out evaluator.

It explicitly does not confine every possible filesystem read, observe the full import closure, or claim network sandboxing. Signal cause and cleanup failure are not canonical shared outcomes.

### Mantle

Mantle maps derivations into build requests with execution-profile, environment, hermeticity, network, store, closure, and output semantics. Practical, strict, and impure modes do not collapse into one capability state.

Build success means output realization and persistence, not only child exit. Fetchers, build sandboxes, scheduler state, cache admission, store publication, and cleanup remain product-owned.

## Approach registry

| Family | Mechanism | Claim | State | Exact blocker or next check |
| --- | --- | --- | --- | --- |
| Common request | Normalize argv, environment, roots, bounds, and requested capabilities | A neutral request might support multiple products | blocked | Consumers request different authority and artifact lifecycles |
| Common outcome | Preserve terminal class plus native extension | Some timeout and output cases map losslessly | active | Dual-run Octet and Nickel Export without sandbox claims |
| Universal sandbox | Treat all mechanisms as one enforcement interface | One shared sandbox contract | falsified | Nickel Export has no network sandbox or full filesystem confinement; mechanism guarantees differ |
| Product adapter | Keep a small core and explicit product extensions | Shared labels can coexist with retained authority | audit | Mandatory signal, cancellation, cleanup, and release-value criteria remain unmet |
| Tiny exit record | Share only success and exit status | A minimal crate is easy to extract | falsified | It erases cleanup, completeness, policy denial, and independent release value |

## Adversarial audit

The audit tried to promote requested or observed restrictions into enforced facts. The checker rejects that claim laundering.

The audit also tested missing consumers, duplicate identities, unknown states and outcomes, incomplete semantics, stale source identities, dependency cycles, unsafe fixture fields, single-consumer extraction, and weakened non-claims.

A license-compatible subset does not resolve semantic blockers. AGPL and MPL boundaries also require an explicit successor provenance and distribution review before code movement.

## Fixtures and decision

`config/bounded-execution-evaluation/fixtures.lines` contains positive and negative cases for the required cross-product. `config/bounded-execution-evaluation/lattice-consumer-fixture.lines` freezes a read-only Lattice mapping at revision `5c9902f777412296c13e2347e45b3bede335b828`.

The Lattice fixture accepts bounded `blocked` or `incubate` evidence. It requires a new Cairn before `extract`. It does not execute or authorize a workflow.

The focused checker is `scripts/cairn/check-bounded-execution-evaluation`. It compiles the Rust checker, runs pure self-tests, checks Nickel freshness, validates the fixture corpus, emits the terminal decision, and writes a metadata-only receipt under `target/cairn/focused-rails/`.

## Replay

From the Animus repository root:

```sh
scripts/cairn/check-bounded-execution-evaluation --root .
scripts/cairn/check-process-job-profiles --root .
scripts/cairn/check-sandlock-host --root .
scripts/cairn/check-agent-guardrail --root .
```

The reviewed source groups were:

- Animus: process-job and Sandlock documents, runtime background data/core, adapter descriptor/dispatch, and std Sandlock core.
- Octet: Aeneas/Lean and effect-port documents, effect-port source, and Aeneas execution source.
- Nickel Export: README, schema guide, core source, and shell source.
- Mantle: README, build-request source, hermeticity source, and network-policy source.

## Non-claims

This evaluation does not prove sandbox equivalence, hermeticity, network denial, filesystem isolation, executor correctness, proof soundness, build correctness, future determinism, or release eligibility.

It does not run a consumer executor or sandbox. It does not create a repository, move public DTO ownership, add a runtime dependency, or approve consumer migration.
