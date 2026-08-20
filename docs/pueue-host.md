# Pueue host

`animus-std-pueue` is a non-default host component for one explicit local pueue namespace.

The component implements the existing Animus queue service contract. It also supplies one executor-kit tool definition.

## Authority boundary

The caller supplies these values:

- Absolute `pueue`, `pueued`, and task-timeout executables
- An absolute namespace root
- One configuration file leaf inside that root
- Reviewed process-job profiles
- Environment, timeout, output, and interaction bounds
- One cancellation flag for bounded pueue client processes

The host clears the process environment. It adds only the namespace values and reviewed profile variables.

The host does not inspect the current directory, home directory, ambient pueue configuration, or ambient daemon.

A caller-owned supervisor starts and stops `pueued`. The host checks the configured daemon before each operation and fails closed when it is unavailable.

## Enqueue admission

An enqueue request contains a reviewed profile name. It does not contain raw shell text.

Animus resolves the profile before any process call. The host rejects unsupported backends, shell-command profiles, missing bounds, memory limits, CPU limits, or writable paths.

The host passes a reviewed timeout wrapper, program, and arguments to `pueue add --escape`. The wrapper enforces the profile deadline and kill-after bound.

Pueue still uses its backend shell to start tasks. This component does not claim task sandboxing or command correctness.

## Supported operations

The first cohort supports:

- Enqueue by reviewed profile name
- Poll one task
- Wait through bounded status polling
- Read one bounded log viewport
- List groups and bounded status counts
- Pause, resume, restart, cancel, and clean with explicit targets

The first cohort denies stream, send-input, and group-parallelism changes.

## Output and evidence

Status projection omits commands, working paths, and environment values.

Log projection returns a bounded tail, counts, a BLAKE3 full-output identity, and a safe reference. Tool output removes known private-output markers before model visibility.

Service receipts contain only outcome classes, counts, and BLAKE3 identities. They do not contain raw process output.

## Verification

Run the focused rail:

```text
scripts/cairn/check-pueue-host --root .
```

The rail runs typed configuration checks, positive and negative crate tests, the isolated real-daemon test, Clippy, facade checks, and component-catalog checks.

## Non-claims

The component does not prove:

- Command correctness
- Scheduler reliability
- Sandboxing
- Production readiness
- Release eligibility
