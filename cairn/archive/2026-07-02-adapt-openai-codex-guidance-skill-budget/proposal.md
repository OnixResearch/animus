# Proposal: Adapt OpenAI Codex guidance and skill context budgeting for Animus

## Why

OpenAI Codex combines layered project guidance with progressive-disclosure skills. Project guidance provides durable repository instructions, while skills expose only metadata up front and load full instructions only when selected. This is a strong pattern for keeping the main prompt focused while still making reusable workflows discoverable.

Animus should adapt the guidance-loading and skill-budget semantics as typed, source-attributed context assembly rather than copying Codex's exact file names or home-directory layout.

## What Changes

- Define layered guidance sources with explicit scope, precedence, override behavior, source identity, byte/token budgets, and truncation evidence.
- Add a prompt assembly contract that preserves source attribution and reports which guidance was included, skipped, or truncated.
- Define skill metadata discovery with bounded initial context, explicit and implicit invocation policy, and deferred full-instruction loading.
- Add skill package descriptors for instructions, optional scripts, references, assets, dependencies, and trust metadata.
- Keep concrete Codex `AGENTS.md`, `.agents/skills`, and plugin formats as import/export adapter concerns rather than Animus core formats.

## Impact

- **Files**: guidance DTOs, prompt assembly logic, skill descriptors, budget accounting, trust/source metadata, docs, and positive/negative tests.
- **Testing**: positive tests for layered overrides, metadata-only skill discovery, explicit invocation, and deferred loading; negative tests for over-budget guidance, ambiguous skill names, disabled skills, untrusted scripts, and source-attribution loss.
- **Non-claims**: this change does not make Codex guidance files the Animus default config language, execute skill scripts in core, or install external plugins.
