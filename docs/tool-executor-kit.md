# Tool executor kit

Animus exposes one coherent host-injected tool execution boundary in `animus_runtime::tool_executor_kit`. The kit combines metadata-only catalogs, explicit executor bindings, pure admission, a thin injected-service shell, bounded output normalization, and safe receipts. It does not ship product tools or discover executors.

## Functional core

`ToolCatalog` stores validated `ToolDefinition` records. A definition combines existing adapter metadata with exact current-schema argument names, an executor binding label, mutation and approval posture, and bounded output policy. Catalog construction is pure and rejects duplicate tool identities, unsafe binding labels, malformed argument sets, and unbounded policies.

`plan_tool_execution` is the final pure gate before effects. It composes the existing neutral `ToolDispatchPlan` with executor-kit checks for:

- active and trusted tool metadata;
- exact tool and schema identity;
- exact current-schema argument names;
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

`ToolExecutorService` is host-implemented. `execute_admitted_tool` is intentionally thin: it rejects denied plans and binding mismatches, invokes exactly the admitted injected service, normalizes transient output through the existing bounded tool-output contract, and returns a validated receipt. Filesystem, process, network, provider, plugin, persistence, credential, and other concrete authority remains entirely with the host implementation.

## Receipt boundary

`ToolExecutionReceipt` stores only contract/status/failure classes, safe tool and executor labels, schema identity, exact capability labels, opaque mutation-target summaries, bounded counts, a safe optional host reference, redaction classes, and BLAKE3 output/receipt identities. Raw output is omitted.

Receipt and output validation reject raw credentials, private paths, raw file bodies, provider payload markers, unbounded stdout/stderr markers, unsafe references, oversized output, malformed hashes, and inconsistent completed/denied states. Rejection does not emit completed-effect evidence.

A resumable session may bind the effective catalog and capability-ceiling identities in its [session runtime authority profile](session-runtime-authority-profile.md). Equality of those identities neither replaces this final admission nor proves current executor, capability, credential, or tool readiness.

## ACP pass-through boundary

The optional [ACP stdio host](acp-stdio-host.md) never executes a tool directly. It asks an injected admission service for the current policy, permission mode, authorization, capability, sandbox, authority-profile, and executor facts; an optional ACP client decision is then applied as an additional denial-only gate before the coding-session host reaches this executor kit. Tool arguments and outputs are omitted from ACP receipts and metadata updates.

## Verification and non-claims

Focused checks:

```sh
cargo test -p animus-runtime tool_executor_kit --locked
cargo test -p animus-runtime composition_conformance --locked
scripts/cairn/check-tool-catalog --root .
scripts/cairn/check-composition-conformance --root .
scripts/cairn/check-adapter-conformance-index --root .
scripts/cairn/check-component-catalog --root .
```

Positive fixtures cover read-only and declared-mutation fake execution. Negative fixtures cover missing or unavailable executors, denied capability, schema and argument mismatch, inactive or untrusted definitions, approval and output policy denial, mutation conflict, cancellation, timeout, fallback attempts, unsafe or oversized output, unsafe references, duplicate catalog identities, and receipt leaks.

This kit does not add default tools, shell execution, filesystem mutation, network tools, MCP transport, plugin loading, provider dispatch, global discovery, or production tool packs.
