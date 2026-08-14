# RLM host-effect executor

`animus-std-rlm-host` is a non-default AGPL-3.0-or-later host component. It implements the existing `RlmKernelHostExecutor` seam.

The crate does not add operating-system authority to `animus-runtime` or the no-std RLM contracts. A composition root must supply all authority and all owners.

## Construction

Create one `RlmHostExecutorConfig` with these values:

- one sealed `ExpectedAuthority`;
- one injected `CurrentAuthorityObserver`;
- one `RouteBinding` for each enabled route;
- named `ExecutorLimits`;
- an exact `CapabilityPacks` value.

Construction fails when a binding and its pack do not agree. No route falls back to ambient authority.

Each binding contains a stable BLAKE3 owner identity, a stable BLAKE3 scope identity, and a generation. Before each effect, the observer must return current seal, generation, cancellation, timeout, scope, and binding facts. The executor rejects stale or denied facts before it calls the route owner.

## Route owners

| Route | Existing owner | Authority and limits | Unsupported behavior |
| --- | --- | --- | --- |
| `file_read` | `cap-root-core` admission and caller-opened `cap_std::fs::Dir` | Relative lexical admission, descriptor `O_NOFOLLOW` on Unix, regular-file checks, identity rechecks, and a byte bound | Parent paths, symlinks, directories, ambient roots, and oversized reads |
| `file_edit` | `animus_std_workspace_edit::WorkspaceEditHost` | An injected binder supplies exact snapshot, revision, target, visibility, gate, and writer facts | Direct writes, invented revisions, implicit targets, and conflict masking |
| `shell` | `animus_runtime::resolve_process_job_profile` and `bounded_exec` | Exact named profile, absolute program, explicit working directory, cleared environment, bounded output, timeout, cancellation, process-group teardown | Shell command text, relative programs, inherited environment, unknown profiles, and CPU or memory claims that `bounded-exec` cannot enforce |
| `tool` | `ToolExecutorKit` and staged execution through `ToolKitRoute` | Exact admitted tool, request, authority seal, commit owner, output policy, durable commit, and terminal receipt | Global lookup, unknown tools, unbounded output, opaque fallback, and uncommitted return |
| `context_read` | `ContextResolverSet` through `ResolverContextRoute` | Explicit resolver pack and byte/line budget | Resolver discovery, unrestricted URL or file access, and hidden truncation |
| `subagent` | `ConcreteChildSessionService` through `ConcreteChildRoute` | Injected pre-admitted child facts and bounded status polling | Ambient agent spawning, transcript receipts, and unbounded waits |
| `kernel_clear` | The injected Steel kernel bridge owner | One delegated clear call and one stable result identity | Global kernel lookup and direct state mutation by the executor |

The host crate links AGPL host components. Its manifest therefore declares `AGPL-3.0-or-later`. It is not a default workspace member or a default root feature.

## Tool-stage commit ownership

The route requires a product-owned `ToolRouteCommitService`. The generic RLM host cannot claim durable completion.

The tool result remains private until the service returns exact synchronized commit evidence. The route then completes metadata-only observation and projects a terminal staged receipt.

See [tool execution stages](tool-execution-stages.md).

## Results and failures

Successful calls return bounded model-visible text and a separate `DispatchObservation`. Observations contain only route labels, BLAKE3 identities, generation, byte count, and character count.

Debug output hides model-visible result text. Failures contain a route, a bounded class, and a safe source label. They do not contain file bodies, command output, tool bodies, context bodies, child transcripts, credentials, or raw paths.

Failure classes are `unavailable`, `denied`, `stale`, `malformed`, `cancelled`, `timed_out`, `bound_exceeded`, `conflict`, and `owner_failure`.

## Validation

Run:

```text
scripts/cairn/check-rlm-host-effect-executor --root .
```

The focused Nix check is:

```text
nix build .#checks.x86_64-linux.rlm-host-effect-executor --no-link -L
```

## Non-claims

This component does not prove sandboxing, hermetic execution, command authenticity, whole-filesystem race freedom, tool correctness, reversible mutation, child correctness, task success, production readiness, or release eligibility.
