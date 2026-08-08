# Execution backend selection evidence

## Baseline

Proposal gate receipt: `5b340b2c59b59015265428a0c3259d2c99feddbe980dedc0fdc010d7344fbb53`.

Design gate receipt: `0e645cffb2c0b9f4a1a7bc68bf99c2502fa272aed57a136b61a1f5094d00557f`.

Tasks gate receipt: `472e38a4f5d07e5e386c91041d330c3e5dcf0617b9ecf671d85ef45a95f9fda5`.

Before core changes, these focused rails passed:

- `scripts/cairn/check-application-host-binding --root .`
- `scripts/cairn/check-runtime-authority-seal --root .`
- `scripts/cairn/check-bounded-execution-evaluation --root .`

## Reference and consumer

- agentOS revision `5b76f89ffd70817de303cf4654b1badd79c0442d`, Apache-2.0. Selected concept: use mediated in-process execution when it satisfies requirements and require explicit escalation for full operating-system workloads.
- Lattice `origin/main` revision `090162f758fdb43c86c9d93d58e4348a478bd86e`. The frozen fixture admits only metadata plan fields and rejects command, credential, environment, path, provider-payload, and diagnostic fields.

Animus does not adopt agentOS V8, Wasm runtime, VM, persistence, transport, sandbox mounting, provider, benchmark, or security claims.

## Boundary

The pure planner uses explicit policy order. It never derives authority from a numeric rank, command name, agent label, or availability failure. The std seam calls one injected current-enforcement service and does not execute selected work.

Positive tests cover all four backend classes. Negative tests cover unknown and incomplete requirements, ambiguity, unavailable selected backends, authority drift, resource denial, silent fallback, service absence, receipt leakage, and sandbox overclaims.

## Validation

The following checks passed after implementation:

- `scripts/cairn/check-execution-backend-selection --root .`
- focused runtime and std tests, Clippy, no-default-feature, Wasm, and root-facade checks from the new rail
- `nix run .#check-nickel-configs`
- component catalog, runtime facade inventory, composition conformance, and safe-receipt rails

- application-host binding, runtime-authority seal, and bounded-execution evaluation rails after implementation
- `nix run .#cairn -- validate --root .`
- `nix build .#checks.x86_64-linux.execution-backend-selection --no-link -L`

`check-roadmap` reports only two unrelated pre-existing unclassified active changes: `add-optional-basalt-authority-adapter` and `add-pijul-vcs-adapter`. This change is classified in the `concrete-adapters` tier.

Sync dry-run receipt `e8a2d4f8fffb98f09bf53a7735a246714a90494e6ca0c6db143b4488b970a1c8` and executed receipt `13172042a3cc0f508063c624a4e25248290178cf04435284594b34c9fb825357` added all seven `agent_harness_runtime.execution_backend_planner.*` requirement IDs to the accepted specification.

Archive dry-run receipt: `bc6ae1c1bbb0477bf7e655880c20d57387b1bc808134bc38735db3c233c2567d`.

Executed archive receipt: `64b475f1cddc7b994229fd3c5d55d2c0d8d1bd4562edd719e819dc2f9f1651c0`.

This evidence does not prove backend availability, sandbox enforcement, work completion, output correctness, production readiness, or release eligibility.
