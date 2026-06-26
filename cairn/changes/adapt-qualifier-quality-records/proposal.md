# Proposal: Adapt Qualifier quality-record patterns without runtime coupling

## Why

Qualifier proves a useful file-native pattern for durable quality observations: humans and agents can record concerns, suggestions, waivers, resolutions, and drift checks beside the code without a server or database. That aligns with Animus' planned safe receipts, provenance facts, VCS-neutral line ranges, and reviewable agent evidence.

Animus should adapt the reusable ideas while keeping Qualifier as a bounded reference. The reusable runtime must not become a `.qual` store, a Git-only diff implementation, or a raw transcript archive. Qualifier compatibility belongs at optional std/CLI adapter boundaries; Animus-owned provenance and receipts remain safe, redacted, and VCS-neutral.

## What Changes

- Treat Qualifier as a bounded reference for Metabox-style envelopes, content-addressed BLAKE3 record IDs, append-only JSONL sidecars, annotation lifecycle semantics, span freshness, and AGENTS-CLI guidance.
- Define Animus-owned safe quality/provenance record envelopes that can project to Qualifier records without requiring the `qualifier` crate in core or adapter contracts.
- Add an optional std-only Qualifier sidecar adapter boundary for reading/writing `.qual`-compatible quality observations while preserving unknown records and redaction policy.
- Adapt annotation lifecycle semantics such as `kind`, `references`, `supersedes`, `resolve`, `waiver`, `suggested_fix`, and span `content_hash` into pure, testable logic.
- Specify future `animus agents` CLI guidance following AGENTS-CLI orientation/topic/pitfall conventions when Animus grows a public CLI.
- Add focused positive and negative verification for canonical IDs, safe body validation, unknown record preservation, span drift, adapter isolation, redaction, and agent guidance discovery.

## Impact

- **Files**: New Cairn change artifacts under `cairn/changes/adapt-qualifier-quality-records/`; later implementation may add quality/provenance DTO modules, optional std/CLI Qualifier compatibility code, docs, focused tests, and CLI guidance pages.
- **Testing**: Future implementation must include positive and negative tests for canonical BLAKE3 IDs, malformed envelopes, unsafe body rejection, unknown record passthrough, span freshness states, supersession cycles, cross-subject resolution rejection, optional adapter dependency isolation, and AGENTS-CLI topic behavior.
- **Non-claims**: This change does not adopt Qualifier as Animus' session ledger, does not import Qualifier into no-std core crates, does not adopt Git-only `qualifier diff` internals, does not add numeric scoring to core, and does not allow raw prompts, provider payloads, full paths, credentials, or tool/model outputs into quality records.
