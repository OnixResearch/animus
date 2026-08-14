# Local daemon supervisor host

The `animus-local-daemon` package supplies the first concrete local supervisor and worker host. The package is non-default and uses `AGPL-3.0-or-later` because its worker composes the live Steel agent.

## Prerequisite record

The pure role contracts adapt the reviewed Prime Agent daemon cohort at revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`. That cohort is MIT licensed.

The host also uses these accepted Animus owners:

- `animus-adapter-daemon` for client, supervisor, worker, detach, and containment contracts;
- transactional daemon handoff for generation and complete-tree replacement rules;
- `animus-live-steel-agent` for worker-owned session execution and strict durable restore;
- `host-control` contracts for transport-neutral status meaning.

The package feature `live-worker` enables the live provider worker path. The package stays outside workspace `default-members`.

## Authority boundary

The supervisor owns one caller-supplied runtime directory. It uses one explicit Unix stream endpoint and one capability-relative registry file.

The supervisor owns routing, attachment leases, bounded events, health classification, process lifecycle, stop finalization, and explicit retry. It does not execute providers, tools, Steel cells, child sessions, queues, or prompts.

The worker owns the live Steel application and its descendants. It validates durable session and kernel state before continuation.

The transport does not use TCP, home-directory discovery, endpoint fallback, or inherited credentials. Linux peer credentials, endpoint identity, and worker generation bind each connection.

## Registry and recovery

Registry records contain only root, worker, generation, process, endpoint, session-head, lifecycle, restore, attachment, event-count, and receipt metadata. Publication writes a private staged file, synchronizes it, and renames it within the capability root.

Recovery distinguishes these states:

- healthy;
- starting;
- stopping;
- stopped;
- disconnected;
- stale;
- identity mismatch;
- zombie;
- unresponsive;
- failed start;
- restore blocked.

A live PID is not sufficient identity. An identity mismatch does not signal the unrelated process.

Stop finalization continues without clients. The registry reports `stopped` only after root exit, descendant exit, and root reap are observed.

Explicit retry increments the generation only when recovery admits replacement.

## Attach and routing

Attach returns a current snapshot before later events. Event flow has a finite count. Detach removes only the client lease. It does not stop worker execution.

Commands cross the `WorkerSessionPort`. The supervisor selects the exact worker generation but does not execute the command.

<!-- r[impl agent_harness_runtime.component_scope.registration] -->
## Component-scope mapping

Each worker generation maps to one deterministic root component-scope plan. Each client attachment gets a separate generation-bound delivery lease.

Client detach removes only that delivery lease. It does not own or close the worker scope. Authorized worker stop remains separate.

`ComponentScopeHost` calls only its injected delivery, cancellation, quiescence, release, and receipt ports. It executes only effects from the pure reducer.

## Evidence

Receipts contain BLAKE3 identities, bounded counts, and these non-claims:

- `not-remote-security`;
- `not-security-sandbox`;
- `not-multi-machine-supervision`;
- `not-high-availability`;
- `not-service-manager-integration`;
- `not-production-readiness`;
- `not-release-eligibility`.

The focused rail is:

```text
scripts/cairn/check-local-daemon-supervisor-host --root .
```

The rail proves the checked fixture and local process paths only. It does not prove security isolation, crash freedom, arbitrary terminal support, or live provider availability.
