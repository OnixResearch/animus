# Tool executor kit

Animus exposes one coherent host-injected tool execution boundary in `animus_runtime::tool_executor_kit`. The kit combines metadata-only catalogs, explicit executor bindings, pure admission, a thin injected-service shell, bounded output normalization, and safe receipts. It does not ship product tools or discover executors.

## Functional core

`ToolCatalog` stores validated `ToolDefinition` records. A definition combines existing adapter metadata with exact current-schema argument names, an executor binding label, mutation and approval posture, and bounded output policy. Catalog construction is pure and rejects duplicate tool identities, unsafe binding labels, malformed argument sets, and unbounded policies.

`plan_tool_execution` is the final pure gate before effects. It composes the existing neutral `ToolDispatchPlan` with executor-kit checks for:

- active and trusted tool metadata;
- exact tool and schema identity;
- exact current-schema argument names;
- a domain-separated BLAKE3 binding over all current request values;
- exact capability registry admission;
- catalog and request approval posture;
- read-only or declared-target mutation posture;
- mutation conflicts;
- output character and byte bounds;
- explicit ready executor bindings;
- cancellation and caller-owned deadline posture; and
- fallback denial.

A denied plan never invokes a service and can produce a safe denial receipt.

## Imperative shell

`ToolExecutorService` is host-implemented. `execute_admitted_tool` rejects denied plans, substituted request values, and binding mismatches before it invokes the selected service. It normalizes transient output once and returns a `ToolExecutionOutcome` with a bounded `ToolResponse` and validated receipt. Filesystem, process, network, provider, plugin, persistence, credential, and other concrete authority remains entirely with the host implementation.

## Receipt boundary

`ToolExecutionReceipt` stores only contract/status/failure classes, safe tool and executor labels, schema identity, exact capability labels, opaque mutation-target summaries, bounded counts, a safe optional host reference, redaction classes, and BLAKE3 output/receipt identities. Raw output is omitted.

Receipt and output validation reject raw credentials, private paths, raw file bodies, provider payload markers, unbounded stdout/stderr markers, unsafe references, oversized output, malformed hashes, and inconsistent completed/denied states. Rejection does not emit completed-effect evidence.

A resumable session may bind the effective catalog and capability-ceiling identities in its [session runtime authority profile](session-runtime-authority-profile.md). Equality of those identities neither replaces this final admission nor proves current executor, capability, credential, or tool readiness.

## Model-facing Steel tool

The non-default `SteelRlmToolAdapter` implements the generic tool port with tool name `steel_repl`. The adapter evaluates one bounded Steel cell and returns either a value or an admitted typed host request.

A host request does not execute inside the adapter. `animus-runtime::rlm_kernel::execute_admitted_host_request` revalidates it and calls one injected `RlmKernelHostExecutor`. Tool-class requests must still route through this executor kit before concrete tool execution.

## Revision-gated workspace mutation

The optional [revision-gated workspace edit host](revision-gated-workspace-edit-host.md) is one possible concrete owner for an already-admitted declared-target mutation. It does not bypass this kit.

The executor must first bind the exact tool, schema, request values, capability, authority, approval, mutation target, cancellation, deadline, and output policy. The edit host then performs its own root, path, visibility, revision, file-metadata, writer-generation, publication, and receipt checks. Neither admission can create the authority required by the other.

## ACP pass-through boundary

The optional [ACP stdio host](acp-stdio-host.md) never executes a tool directly. It asks an injected admission service for the current policy, permission mode, authorization, capability, sandbox, authority-profile, and executor facts; an optional ACP client decision is then applied as an additional denial-only gate before the coding-session host reaches this executor kit. Tool arguments and outputs are omitted from ACP receipts and metadata updates.

## Verification and non-claims

Focused checks:

```sh
cargo test -p animus-runtime executionkit --locked
cargo test -p animus-runtime composition_conformance --locked
scripts/cairn/check-tool-catalog --root .
scripts/cairn/check-composition-conformance --root .
scripts/cairn/check-adapter-conformance-index --root .
scripts/cairn/check-component-catalog --root .
scripts/cairn/check-revision-gated-workspace-edit-host --root .
```

Positive fixtures cover read-only and declared-mutation fake execution. Negative fixtures cover substituted argument values, missing or unavailable executors, denied capability, schema and argument mismatch, inactive or untrusted definitions, approval and output policy denial, mutation conflict, cancellation, timeout, fallback attempts, unsafe or oversized output, unsafe references, duplicate catalog identities, and receipt leaks.

The optional [read-only codebase tool host](read-only-codebase-tool-host.md) adapts the existing [codebase graph](codebase-graph-host.md) through this kit. The executor still owns final tool, schema, capability, output, and request-value admission. Graph access cannot authorize repository reads or edits, and stale indexes fail before completion evidence.

This kit does not add default tools, shell execution, filesystem mutation, network tools, MCP transport, plugin loading, provider dispatch, global discovery, or production tool packs.
