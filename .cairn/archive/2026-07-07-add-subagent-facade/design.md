# Design: Add a unified subagent facade

Animus should make subagent orchestration easier by creating a facade over existing safe contracts rather than moving execution authority into reusable runtime code.

## Decisions

### 1. Facade is a projection, not a new authority

**Choice:** `animus-runtime::subagents` introduces common `ChildRunDraft`, `ChildRunKind`, `ChildRunAdmission`, `ChildRunDiagnostic`, and `ChildRunReceipt` data. It delegates validation to the existing recipe-backed, fresh-context, side-question, and worker-team validators.

**Rationale:** Embedders get one entry point while existing fail-closed behavior remains authoritative.

### 2. Source-specific data remains typed

**Choice:** The facade stores source-specific fields in typed variants rather than a stringly metadata map.

**Rationale:** Typed variants prevent field loss and preserve testable, deterministic conversion behavior.

### 3. Conversions are lossless and auditable

**Choice:** Facade conversions must preserve child identity, delegation source, kind, capability ceilings, workspace scope, context reference identity, return mode, diagnostics, and safe receipt hashes.

**Rationale:** Parent orchestration, provenance, and UI projections need common fields without hiding why a child run was accepted or rejected.

### 4. Execution remains host-owned

**Choice:** Facade validation does not call models, tools, schedulers, worker services, external CLIs, or network resources.

**Rationale:** Animus' reusable runtime should keep the functional-core / imperative-shell boundary intact.
