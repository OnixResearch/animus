# Proposal: Adapt Pi skills and resource discovery for Animus

## Why

Pi's skills and resource loading show a useful progressive-disclosure model: at startup, only names and descriptions need to enter the system prompt; full skill instructions, prompt templates, context files, and project-local resources load only when needed and only after project trust is resolved. Animus needs the same separation so hosts can offer reusable workflows without bloating context or trusting project-local executable guidance by default.

Animus should adapt the resource model and trust sequencing, while keeping Nickel as the source of truth for repo-owned config and leaving package installation/execution to host shells.

## What Changes

- Define resource descriptors for skills, prompt templates, context files, and host-provided guidance with source/provenance metadata and trust scope.
- Add project-trust gating before project-local dynamic resources are loaded or allowed to influence prompts/tools.
- Add progressive skill disclosure: model-visible skill summaries by default, full skill content loaded only by explicit command or matching host policy.
- Add command/resource lookup behavior with deterministic collision handling and diagnostics.
- Add validation for malformed resource descriptors, missing descriptions, unsafe paths, duplicate names, and untrusted project resources.

## Impact

- **Files**: resource loader contracts, trust policy DTOs, skill/template/context descriptor types, prompt-construction integration, tests, and docs.
- **Testing**: positive tests for trusted resources and on-demand skill loading; negative tests for untrusted resources, malformed skills, duplicate names, unsafe paths, and context bloat.
- **Non-claims**: this change does not adopt Pi packages, npm/git package installation, TypeScript extensions, themes, TUI components, or automatic third-party skill execution.
