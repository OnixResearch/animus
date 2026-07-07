# Contracts index

This index is the map of record for maintained public adapter contract families. It is intentionally bounded: it names owner modules, stability/version posture, authority boundary, safe receipt behavior, serialization bridge status, focused surfaces, and validation rails without requiring concrete host providers, filesystem access, process execution, network access, credentials, or plugin loading.

Run the rail:

```sh
scripts/cairn/check-adapter-conformance-index --root .
nix build .#checks.x86_64-linux.adapter-conformance-index --no-link -L
```

## Maintained families

| Family | Owner module | Stability/version posture | Authority boundary | Safe receipt behavior | Serialization bridge | Focused surface | Focused checks |
| --- | --- | --- | --- | --- | --- | --- | --- |
| family:model-router | `crates/animus-router/src/lib.rs` and `animus_adapters::router` transition shim | stable `Recipe`, `Signals`, `Budget`, `SelectedPlan`, `RouteResult`, `Receipt`, and checked-in policy JSON DTOs | pure routing decisions before host dispatch; no ambient authority or provider calls | router receipts carry safe labels, counts, redaction markers, and BLAKE3 identities only | Preserves projection is absent by default and must revalidate DTOs when enabled | `animus_router::{Recipe, Signals, Budget, SelectedPlan, Receipt, ROUTER_RECIPE_POLICY_JSON, select_recipe, validate_receipt_text}` and `animus_adapters::router` | `cargo test -p animus-router --locked`; `scripts/cairn/check-router-policy --root .`; `cargo test -p animus-adapters router --locked` |
| family:model | `crates/animus-adapters/src/model.rs`, `crates/animus-adapters/src/model/provider_conformance.rs`, and [canonical model contracts](canonical-model-contracts.md) | stable `animus.model-request.v1`, `animus.model-response.v1`, `animus.model-message-request.v1`, and `animus.model-response-envelope.v1` DTOs | injected `ModelAdapter` only after exact runtime capability admission; canonical contracts and provider projection fixtures remain provider-neutral and pure | model responses and canonical envelopes are validated as safe text/labels; durable runtime evidence stores statuses, classes, counts, and hashes rather than raw prompts, provider bodies, credentials, or hidden reasoning | optional Preserves bridge after DTO revalidation | `animus_adapters::{ModelAdapter, ModelRequest, ModelResponse, CanonicalModelRequest, CanonicalModelResponseEnvelope, ProviderProjectionFamily, ProviderProjectionFeatureSupport, ProviderProjectionSupport, ModelOutcomeAdapter}` and `animus_runtime::tool_prelude` | `cargo test -p animus-adapters model --locked`; `cargo test -p animus-adapters provider_conformance --locked`; `cargo test -p animus-runtime shell::tests --locked` |
| family:tool | `crates/animus-adapters/src/tool.rs` | stable `animus.tool-request.v1` and `animus.tool-response.v1` DTOs | injected `ToolAdapter` only after exact runtime capability admission and provenance checks | tool output is bounded or hashed, mutation receipts carry safe labels, and unsafe output references fail closed | optional Preserves bridge after DTO revalidation | `animus_adapters::{ToolAdapter, ToolRequest, ToolResponse, ToolOutcomeAdapter}` and `animus_runtime::tool_prelude` | `cargo test -p animus-adapters tool::tests --locked`; `cargo test -p animus-runtime shell::tests --locked` |
| family:host-status | `crates/animus-adapters/src/status` | stable status observation DTOs and runtime status selection contracts | host status observations are injected; runtime status selection is pure and does not poll ambient hosts | status evidence uses bounded safe hashes, TTLs, and redacted summaries | no required serialization backend | `animus_adapters::{HostStatusAdapter, HostStatusRequest, HostStatusObservationBatch}` and `animus_runtime::status_prelude` | `cargo test -p animus-adapters status::tests --locked`; `cargo test -p animus-runtime status::tests --locked` |
| family:workspace | `crates/animus-adapters/src/workspace` | stable workspace plan, isolation, handoff, and receipt DTOs | workspace contracts describe plans and descriptors; concrete VCS and filesystem mutation stays shell-owned | workspace receipts contain safe labels, BLAKE3-style hashes, restore evidence, and leak classes, not raw file bodies | host shells may serialize after revalidation | documented workspace export set: `WorkspaceAdapterDescriptor`, `WorkspaceIsolationPlan`, `WorkspaceReceipt`, `validate_workspace_receipt_text` | `cargo test -p animus-adapters workspace::tests --locked` |
| family:security-policy | `crates/animus-adapters/src/effect` | stable policy, permission, confirmation, authorization, sandbox, and outcome DTOs | policy resolution is pure and denies by default; concrete enforcement is injected or shell-owned | permission, authorization, confirmation, and sandbox receipts are redacted and leak-scanned | external policy stores are outside adapter core; bridge only after validation | `animus_adapters::{CapabilityPolicyContract, RuntimePolicyService, PermissionPolicy, SandboxPolicy}` and `animus_runtime::policy_prelude` | `cargo test -p animus-adapters effect::tests --locked` |
| family:safe-receipt | `crates/animus-adapters/src/effect/audit.rs` | stable `animus.safe-receipt.v1` identity and bounded receipt primitives | reusable DTO validation only; no provider, filesystem, process, network, credential, UI, or plugin authority | receipts store bounded safe text, safe hashes, redaction labels, and leak classes only | text contract first; bridges encode after revalidation | `animus_adapters::{Receipt, ReceiptDraft, SafeHash, SAFE_RECEIPT_CONTRACT_IDENTITY}` plus receipt validators | `cargo test -p animus-adapters effect::tests --locked`; `cargo test -p animus-adapters contract::tests --locked` |

## Conformance matrix

Every maintained family declares all required matrix cases in `cairn/adapter-conformance-index.ncl`. Provider projection subcases are recorded separately in `cairn/provider-projection-conformance-index.ncl` so provider/API-style coverage can distinguish covered, unsupported, and not-applicable cases with explicit reasons. A case is either `covered` with a positive or negative evidence label, `unsupported` with a safe reason in the provider projection index, or `not-applicable` with a safe reason. Missing negative coverage without a reason fails the rail.

| Case | Required expectation |
| --- | --- |
| `happy_path` | Valid DTOs or adapters construct and round-trip the public boundary. |
| `missing_service` | Absence of an injected service fails closed without fallback. |
| `denied_capability` | Denied capability or policy stops before adapter execution. |
| `unavailable_service` | Known but unavailable service maps to a typed fail-closed outcome. |
| `timeout` | Deadline expiry maps to a typed fail-closed outcome or is marked not-applicable when the family owns no clock. |
| `cancellation` | Cancellation maps to a typed fail-closed outcome or is marked not-applicable when the family owns no handle. |
| `malformed_output` | Malformed DTOs or adapter outputs are rejected. |
| `oversized_output` | Oversized outputs are rejected, bounded, or hashed with safe evidence. |
| `unsafe_reference` | Raw host references, private paths, and unsafe receipt material are rejected. |
| `schema_mismatch` | Missing or unsupported contract versions fail closed. |
| `duplicate_field` | Duplicate keyed data is rejected or safely not-applicable for scalar DTOs. |
| `unknown_field` | Unknown open fields are rejected or safely not-applicable for constructors that do not parse open maps. |
| `leak_rejection` | Raw prompts, provider bodies, credentials, raw file bodies, unbounded logs, and similar sensitive material are rejected from durable evidence. |

## Focused surfaces

Embedders should choose the smallest surface that matches their host responsibility:

- Router-only embedders use `animus-router` for pure recipe, signal, selection, result, receipt decisions, and the checked-in `ROUTER_RECIPE_POLICY_JSON`; `animus_adapters::router` remains a transition shim.
- Model-only embedders use `ModelRequest`, `ModelResponse`, canonical `CanonicalModelRequest` / `CanonicalModelResponseEnvelope` when structured conversations are needed, provider projection conformance helpers (`ProviderProjectionFamily`, `ProviderProjectionFeatureSupport`, `ProviderProjectionSupport`, `project_model_provider_request_fixture`, `normalize_model_provider_response_fixture`, `assert_model_provider_stream_fixture`, and `validate_model_provider_projection_index`) when claiming provider/API-style parity, `ModelAdapter`, `ModelOutcomeAdapter`, and runtime capability descriptors from `tool_prelude`.
- Tool-only embedders use `ToolRequest`, `ToolResponse`, `ToolAdapter`, `ToolOutcomeAdapter`, tool provenance DTOs, and runtime exact capability descriptors from `tool_prelude`.
- Host/status embedders use status DTOs plus `status_prelude`; runtime selection remains pure and does not poll host state.
- Workspace embedders use workspace descriptors, plans, and receipt validators; concrete checkout mutation remains in a shell adapter.
- Security/policy embedders use effect-policy DTOs plus `policy_prelude`; concrete enforcement and confirmation brokers remain injected.
- Receipt embedders use safe receipt primitives and validators directly; serialization bridges must revalidate after decode.

## Evidence contract

The rail emits counts, family identifiers, case dispositions, fixture names, and BLAKE3 hashes of exported manifest lines. It omits raw prompts, provider payload bodies, credentials, private paths, unbounded tool output, and host-local configuration values.
