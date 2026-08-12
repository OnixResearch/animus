# Adapter module ownership map

This map supports r[agent_harness_runtime.adapter_crate_maintainability.inventory], r[agent_harness_runtime.adapter_crate_maintainability.ownership_map], r[agent_harness_runtime.adapter_crate_maintainability.semantic_shards], r[agent_harness_runtime.adapter_crate_maintainability.functional_core], r[agent_harness_runtime.adapter_crate_maintainability.lint_scope], and r[agent_harness_runtime.adapter_crate_maintainability.verification]. It is review evidence for behavior-preserving adapter refactors only.

## Hotspot inventory

Large or historically mechanical adapter areas are inventoried before refactor work:

| Area | Hotspot | Disposition |
|---|---|---|
| Generic memory adapters | `crates/animus-adapters/src/front/memory/impl/generic/adapter_records.rs` | Owns injected memory adapter descriptors, category-aware safe records, admission/projector core, and receipt-safe record identities. |
| Generic memory controls | `crates/animus-adapters/src/front/memory/impl/generic/operation_controls.rs` | Owns safe memory operation receipts, remote/control admission, and command validation. |
| Generic memory conformance | `crates/animus-adapters/src/front/memory/impl/generic/conformance_matrix.rs` | Owns memory backend conformance rows, null-backend planning, and safe evidence rows. |
| Tool gateway delivery | `crates/animus-adapters/src/front/tool/gateway/delivery_types.rs` | Owns stable delivery enums, target/capability classes, adapter availability, and retryability labels. |
| Tool gateway delivery | `crates/animus-adapters/src/front/tool/gateway/delivery_request.rs` | Owns borrowed/validated artifact delivery request metadata. |
| Tool gateway delivery | `crates/animus-adapters/src/front/tool/gateway/delivery_policy.rs` | Owns host-effect-free policy decisions and safe delivery execution input shapes. |
| Tool gateway delivery | `crates/animus-adapters/src/front/tool/gateway/delivery_outbox.rs` | Owns safe outbox record construction after policy and adapter readiness are known. |
| Tool gateway delivery | `crates/animus-adapters/src/front/tool/gateway/delivery_retry_receipts.rs` | Owns retry planning and BLAKE3 delivery receipt construction. |
| Tool gateway delivery | `crates/animus-adapters/src/front/tool/gateway/delivery_errors.rs` | Owns delivery validation errors and normalization helpers. |
| Review metrics | `crates/animus-adapters/src/review/impl/report_types.rs` | Owns review report drafts, verdicts, diagnostics, and parsed finding DTOs. |
| Review metrics | `crates/animus-adapters/src/review/impl/metric_records.rs` | Owns parsed report, metric records, summaries, and promotion candidate DTOs. |
| Review metrics | `crates/animus-adapters/src/review/impl/report_reducers.rs` | Owns pure report parsing entrypoint, append-only record construction, and summary reducers. |
| Review metrics | `crates/animus-adapters/src/review/impl/report_parser.rs` | Owns parser helpers, tag normalization, safe metadata checks, and deterministic record hashing. |
| Corrective review memory | `crates/animus-adapters/src/review/impl/corrective_memory.rs` | Owns corrective-memory DTOs, admission, promotion routing, host-store request validation, and safe receipts. |
| Done-review lifecycle | `crates/animus-adapters/src/review/impl/lifecycle_review.rs` | Owns done-review lifecycle subjects, evidence, rail descriptors, host-effect-free dispatch plans, reducers, and receipts. |

## Semantic ownership map

- Memory adapter family: `front::memory` owns `agent_harness_runtime.memory_*` and generic memory conformance requirements. Public exports continue through `pub use memory::*` from `front/mod.rs` and through `pub use front::*` from `lib.rs`.
- Tool gateway family: `front::tool_gateway` owns `agent_harness_runtime.clankers_tool_gateway.*` delivery request, policy, outbox, retry, and receipt requirements. Public exports continue through `pub use tool_gateway::*` from `front/mod.rs` and through `pub use front::*` from `lib.rs`.
- Review family: `review` owns review report parsing, metrics, corrective memory, and done-review lifecycle DTOs. Public exports continue through the existing `pub use review::{...}` list in `lib.rs`.
- Conformance evidence stays linked from `.cairn/adapter-conformance-index.ncl` and `docs/contracts-index.md`; this map names source owners, not a replacement conformance index.

## Compatibility aliases and migrations

The refactor only renames private implementation shards. No public Rust item names or public import paths were removed. The old mechanical filenames (`impl009.rs`, `impl010.rs`, `impl011.rs`, `gateway/impl001.rs` through `gateway/impl006.rs`, and review implementation shards other than test wiring) are not public API, so compatibility is preserved by keeping the wrapper modules and public re-export paths unchanged.

`crates/animus-adapters/src/review/impl/impl005.rs` remains as private test wiring because it has no semantic runtime owner and only includes positive/negative review fixtures.

## Functional-core / shell split

These adapter shards remain reusable, deterministic, and host-effect-free. Validation, projection, reduction, receipt construction, and dispatch planning are pure functions over typed inputs. Host shells still own filesystem access, network/provider calls, clocks, credentials, process execution, persistence, terminal UI, and product mutation.

- Memory operation and conformance files contain DTO validation, availability planning, and safe receipt construction only.
- Tool delivery files separate request validation, policy evaluation, outbox record construction, retry planning, and receipt hashing.
- Review files separate report DTOs, parser helpers, reducers, corrective-memory planning, and done-review lifecycle reduction.


Existing adapter-wide lint allowances are retained only where the public DTO surface intentionally has repeated adapter-family names or optional Tiger Style lint names. The semantic shard names make remaining allowances reviewable by owner. Future broad `allow` additions in these hotspots should either be removed, moved to the narrow semantic file that needs them, or justified in this map before closeout.

## Verification and non-claims

Focused closeout evidence for this map is:

- `cargo test -p animus-adapters`
- `scripts/cairn/check-adapter-conformance-index --root .`
- `scripts/cairn/check-dogfood-readiness --root .`
- `scripts/cairn/check-adapter-maintainability --root .`
- Cairn gates and validation for the active change.

This maintainability work does not add provider clients, tool executors, persistence backends, dogfood shell behavior, product CLI behavior, live service access, or production-readiness claims.
