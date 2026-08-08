# Native evaluation host

`animus-std-eval` is the non-default standard-library host for native evaluation.
It consumes plans from `animus-eval-core` and ports from `animus-adapter-eval`.
It does not discover services or storage.

Enable the root `evaluation-host` feature to expose `animus::evaluation_host`.
The feature also enables the native core and adapter contracts.
Default Animus builds do not include this host.

## Construction

A caller supplies these values:

1. One checked policy from `config/generated/evaluation-host-policy.json`.
2. One validated native `RecipePlan`.
3. One exact nine-service adapter catalog.
4. Separate tool, timer, cancellation, and cleanup services.
5. Separate model and task-state gateway capabilities.
6. Durable accepted-turn and turn-commit evidence.
7. One allowlisted artifact manifest.
8. One capability-rooted logical storage target.

`preflight_host` validates all values before a service effect.
Missing, denied, unavailable, unsupported, stale, or excessive services fail closed.
The host does not inspect `PATH`, environment variables, home directories, registries, or provider defaults.

## Authority map

| Surface | Owner | Host rule |
| --- | --- | --- |
| Task loading | injected `TaskSourcePort` | Use the exact admitted implementation and bounds. |
| Agent execution | injected `AgentExecutionPort` | Use the solver environment and exact run binding. |
| Model calls | injected capture port and gateway capability | Require durable acceptance before dispatch. |
| Tools | injected `ToolPort` | Use the selected tool identity only. |
| Artifacts | injected collect and restore ports | Require allowlisted, collision-free, no-link entries. |
| Verifier | injected `VerifierExecutionPort` | Use a distinct verifier environment. |
| Results | injected append, replace, and reopen ports | Use one explicit logical destination and strict evidence. |
| Time and cancellation | injected timer and cancellation ports | Stop new dispatch when either becomes current. |
| Cleanup | injected cleanup port | Keep unresolved work as `CleanupRequired`. |

The host shell can own these effects for one embedding.
Reusable crates do not gain this authority.

## Concurrency and lifecycle

The policy has separate positive episode and per-episode agent limits.
The request states its required parallelism.
Preflight rejects requests that exceed either limit.

The shell acquires permits only around selected work.
It releases agent permits after each service call.
Cancellation and deadline checks occur before later dispatch.
Late gateway writes are denied after a run seal.

The public entrypoint uses this order:

```text
preflight
  -> reopen prior safe result
  -> materialize task
  -> gateway admission and model metadata capture
  -> agent and tool services
  -> allowlisted artifact handoff
  -> distinct verifier service
  -> append, replace, and reopen safe result
  -> seal run
  -> cleanup
  -> metadata-only receipt
```

The native core remains the reduction owner.
The canonical session ledger remains the trace and replay owner.

## Gateway

The gateway accepts loopback or a caller-admitted bind.
Public binds and wildcard network policy are denied.
Model and task-state capabilities must differ.

The gateway records bounded protocol metadata only.
It does not retain raw provider bodies.
It does not rewrite responses, tool results, web results, or assistant bodies.

Model dispatch requires exact durable accepted-turn evidence.
Terminal delivery requires matching turn-commit and result evidence.
Only exact non-streaming retries can coalesce.
Content equality alone is not enough.

## Verifier and artifacts

An isolated verifier uses a different implementation and environment identity.
The request also binds its policy, subject, rubric, and attempt lineage.
Shared-runtime judges remain advisory.

Artifact paths are logical and capability-relative.
Preflight rejects absolute paths, parent segments, duplicates, links, hidden tests, and over-limit material.
The host does not claim artifact authenticity.
The selected filesystem implementation must enforce no-follow behavior.

## Persistence and resume

The request supplies a root capability identity and a logical output path.
Safe output uses metadata and canonical ledger references.
It excludes prompts, reasoning, tool bodies, provider bodies, hidden tests, credentials, environment values, and physical paths.

The result ports model append, complete replacement, and strict reopen.
The policy selects a durability class.
A receipt reports that class as selected evidence only.
It does not prove stronger host durability.

Torn, malformed, failed, missing, stale, conflicting, or mismatched records cannot suppress rerun.
Native content-addressed resume remains the decision owner.

## Failure recovery

A blocked service stops later dispatch.
Cancellation and timeout keep their own outcome classes.
A terminal result and a cleanup failure remain separate facts.
The host returns `CleanupRequired` until the selected cleanup service succeeds.

## Reviewed Verifiers cohort

The reviewed `PrimeIntellect-ai/verifiers` revision is `1a997814e85a2e4f409add960aa852737388dbd8` under MIT.
The ordered host cohort identity is `b3:c6336b188596db9b18cb2f0a721c9f5bc551e619342e89807e20ec45f3bb676f`.
The cohort includes `verifiers/v1/cli/eval/runner.py`, resume, episode, interception, rollout, limiter, state, and artifact sources.

Animus adopts bounded episode fan-out, borrowed-resource lifecycle checks, exact retry coalescing, sealed late-write handling, content-keyed resume, atomic replacement, and bounded artifacts.
Animus rejects dynamic installation, broad networking, user-global limiter files, raw trace storage, absolute artifact restoration, and built-in Docker, Modal, Prime, cloud, or registry behavior.

## Verification

Run:

```sh
scripts/cairn/check-std-evaluation-host --root .
nix build .#checks.x86_64-linux.std-evaluation-host --no-link -L
```

The focused rail covers policy freshness, public-entrypoint fake services, all built-in recipes, preflight denial, concurrency, cancellation, gateway ordering, verifier isolation, artifacts, persistence, cleanup, receipts, and dependency isolation.

## Non-claims

This host does not prove confinement, network enforcement, provider correctness, tool safety, persistence durability, or exactly-once remote effects.
It does not prove verifier soundness, artifact authenticity, benchmark validity, training fitness, authoritative completion, production readiness, or release eligibility.
