# Design: Agent-harness roadmap sequencing

## Context

The current Cairn tree captures a broad product surface: runtime loop, policies, receipts, ledgers, prompts, providers, tools, background queues, headless protocols, TUI hosting, datasets, and bounded references. The missing artifact is not more scope; it is ordering and dependency hygiene.

## Decisions

### 1. Treat lifecycle infrastructure as tier zero

Accepted baseline specs, local Cairn policy wiring, validation commands, and roadmap evidence come before behavior changes. Every later change relies on a predictable lifecycle surface.

### 2. Land reusable contracts before host adapters

The runtime loop, injected service boundary, event lifecycle, safe receipts, and neutral ledger/provenance contracts should be accepted before concrete model providers, filesystem/process tools, UI shells, background queues, or dataset exporters claim stable behavior.

### 3. Keep optional references behind explicit gates

Reference adaptations such as Syndicate, Toolpath, Trellis, Sandlock, BranchFS, Qualifier, Hermes, Codex, Claude Code, Pi, and Clankers must declare whether they affect core contracts, host adapters, docs, or evaluation-only scope. Optional adapters cannot become default dependencies without a dedicated change.

### 4. Consolidate overlaps by owning concepts

Ledger changes own replay and branch semantics. Receipt changes own safe evidence and redaction. Event lifecycle changes own ordering and hook/middleware boundaries. Provider registry changes own provider metadata and credential shell behavior. Manifests/catalogs own descriptor validation only unless a separate executor change exists.

### 5. Archive only after prerequisites are accepted or waived

A dependent change may proceed when prerequisite requirement IDs are accepted, are implemented in the same archive batch, or have a documented waiver with a replacement requirement ID.

## Risks

- A roadmap can become stale. Keep it small, requirement-ID based, and validated by focused fixtures rather than prose-only promises.
- Over-constraining order can block useful docs and evaluation changes. Evaluation-only changes may land earlier when they do not create implementation contracts.
