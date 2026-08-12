# Proposal: Add a unified subagent facade

## Why

Animus currently has three related child-run surfaces: recipe-backed subagents in `animus-adapters`, fresh-context tasks in `animus-runtime`, and worker-team contracts in `animus-runtime`. Each surface is safe on its own, but embedders must learn separate DTOs and failure vocabulary before they can build coherent subagent orchestration.

This change adds a reusable facade that presents those surfaces as one typed child-run boundary without weakening their existing validators or host-owned execution split.

## What Changes

- Add an `animus-runtime::subagents` facade that represents supported child runs with common draft, kind, admission, receipt, and diagnostic types.
- Preserve source-specific validators for recipe-backed, fresh-context, side-question, and worker-team runs.
- Provide conversion helpers that keep source-specific metadata and failure diagnostics lossless.
- Document which fields are common, which fields remain kind-specific, and which APIs are compatibility shims.

## Impact

- **Files**: future runtime facade module, public exports, docs, positive and negative facade tests, and conformance fixtures.
- **Testing**: valid conversions for all existing child-run kinds; negative tests for missing kind-specific fields, invalid source metadata, lossy diagnostics, and accidental execution during facade validation.
- **Non-claims**: this does not spawn subagents, merge transcripts, or replace the existing lower-level child/fresh-task/worker validators.
