# Proposal: Adapt Goose recipe-backed subagents for Animus

## Why

Goose subagents combine isolated execution, recipe-defined roles, extension/tool restrictions, return-mode controls, and optional external-agent delegation. Animus already has subagent thread work focused on explicit delegation and summary handoff, but Goose adds a useful link between reusable recipes and specialized subagent profiles.

This change adapts recipe-backed subagents as explicit, policy-gated child runs with isolated ledgers and bounded summaries.

## What Changes

- Extend subagent delegation requests with optional recipe/profile references, parameter bindings, tool ceilings, timeout/turn ceilings, return mode, and summary schema.
- Require explicit user, trusted skill, or admitted automation policy delegation; do not adopt Goose's automatic subagent spawning by default.
- Keep subagent transcripts isolated from parent context unless a bounded import is explicitly requested.
- Add return modes for summary-only, status-plus-summary, and full-detail-with-bounds.
- Add external subagent descriptors for ACP/MCP-backed agents while preserving tool, workspace, and credential boundaries.
- Add failure aggregation for timeout, malformed summary, partial parallel success, denied tools, and write conflicts.

## Impact

- **Files**: future recipe-backed subagent DTOs, delegation validators, summary/return-mode contracts, external-subagent descriptors, aggregation helpers, docs, tests, and runtime event records.
- **Testing**: positive recipe-backed review/research subagents and parallel read-only aggregation tests; negative tests for implicit spawning, unauthorized tools, untrusted recipes, raw-log flooding, failed subagents, write conflicts, and malformed summaries.
- **Non-claims**: this change does not enable uncontrolled automatic subagent creation, does not recursively spawn subagents, does not grant inherited tools by default without capability intersection, and does not merge parallel writes silently.
