# Proposal: Adapt Goose recipe workflows for Animus

## Why

Goose recipes package repeatable agent setups: instructions, prompts, activities, parameters, extensions, model settings, retry validation, subrecipes, and structured automation output. Animus already plans workflow commands, policy profiles, tool contracts, and headless surfaces, but it needs a richer workflow manifest contract for reusable, shareable, and automatable agent runs.

This change adapts the useful recipe pattern as an Animus-owned workflow specification. Nickel remains the source-of-truth authoring format, and exported YAML or JSON is treated as interchange only.

## What Changes

- Define typed Animus recipe/workflow manifests for instructions, initial prompt, activities, parameter schema, model profile, allowed capabilities, extension/toolset references, validation gates, retry policy, subrecipes, and final response contract.
- Use Nickel contracts as the repo-owned authoring and validation layer, with optional export to tool-native YAML or JSON.
- Add pure parameter binding, template-variable validation, and file-parameter safety checks before prompt assembly.
- Add structured final-output validation so automation can depend on typed success, warning, or failure payloads.
- Add retry planning that records checks and cleanup requests without letting shell commands bypass capability, sandbox, timeout, confirmation, or receipt policy.
- Add trust and provenance records for first-run, changed, imported, or project-provided recipe manifests.

## Impact

- **Files**: future recipe/workflow DTOs, Nickel contracts, pure binding and validation helpers, final-output schema validation, retry plan DTOs, trust receipts, docs, tests, and optional exporters.
- **Testing**: positive recipe loading, parameter binding, structured output, and retry-planning tests; negative tests for malformed templates, unsafe file parameters, missing defaults, invalid response schemas, untrusted imports, and shell-check bypass attempts.
- **Non-claims**: this change does not adopt Goose YAML as Animus' source of truth, does not execute recipe shell checks directly in reusable core, does not include global memory or secrets in recipes, and does not make recipe instructions a substitute for runtime policy enforcement.
