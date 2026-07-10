# Evidence: Add an explicit tool executor kit

## Baseline

- PASS: `nix develop -c cargo test -p animus-runtime --locked` before implementation.
- PASS: lifecycle validation, roadmap validation, and proposal/design/tasks gates before implementation.

## Implementation

- Added `animus_runtime::tool_executor_kit` with metadata-only `ToolCatalog` and executor-bound `ToolDefinition` contracts.
- Added pure `plan_tool_execution` admission over current schema and arguments, exact capabilities, approval, mutation posture/conflicts, output bounds, explicit executor readiness, cancellation, deadline posture, and fallback denial.
- Added the host-injected `ToolExecutorService` boundary and thin `execute_admitted_tool` shell. Reusable code performs no executor discovery or concrete host effect.
- Added completed and denied `ToolExecutionReceipt` construction with BLAKE3 output/receipt identity, bounded counts, safe labels/references, explicit redaction, and raw-output omission.
- Added positive read-only and declared-mutation fake executor fixtures plus negative missing/unavailable executor, capability, schema, argument, inactive/untrusted, approval, output-policy, mutation, cancellation, timeout, fallback, unsafe output/reference, oversized output, duplicate catalog, and receipt-leak fixtures.
- Extended the existing tool composition scenario to execute through the coherent fake executor kit without changing live-service or product-tool claims.

## Verification

- PASS: `nix develop -c cargo test -p animus-runtime tool_executor_kit --locked`; 4 focused tests passed.
- PASS: `nix develop -c cargo test -p animus-runtime composition_conformance --locked`; 6 focused tests passed.
- PASS: `nix develop -c cargo test -p animus-runtime --locked`.
- PASS: `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings`.
- PASS: `nix develop -c cargo fmt --all -- --check`.
- PASS: `scripts/cairn/check-tool-catalog --root .`.
- PASS: `scripts/cairn/check-composition-conformance --root .`.
- PASS: `scripts/cairn/check-adapter-conformance-index --root .`.
- PASS: `scripts/cairn/check-component-catalog --root .`.
- PASS: `scripts/cairn/check-runtime-facade-inventory --root .`.
- PASS: `scripts/cairn/check-dogfood-readiness --root .`.
- PASS: `scripts/cairn/check-request-lifecycle --root .`.
- PASS: `scripts/cairn/check-safe-receipts-artifacts --root .`.
- PASS: `nix run .#cairn -- validate --root .`.
- PASS: proposal, design, and tasks gates after implementation.

## Non-claims

This change does not add default product tools, shell or process execution, filesystem mutation, network tools, MCP transport, plugin loading, provider dispatch, global tool discovery, or production tool packs.
