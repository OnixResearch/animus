# Design: Cairn policy validation wiring

## Context

Cairn runtime validation consumes generated policy JSON. Animus can already validate when a developer passes an absolute path to the Cairn repo's generated policy, but the default command fails if the generated policy is absent under the Animus root.

## Decisions

### 1. Check in generated runtime policy

Animus should keep `cairn-policy/generated/cairn-policy.json` as the runtime policy artifact expected by Cairn commands. The file is generated data because Cairn currently consumes JSON for runtime policy.

### 2. Document source and refresh path

The policy docs should name the canonical source, the command used to refresh the generated JSON, and the command used to verify it. If Cairn later supports a package-exported policy app, Animus can switch the refresh path without changing validation semantics.

### 3. Keep runtime validation Nickel-free

Repo-owned config should prefer Nickel where Animus owns the schema, but Cairn runtime validation expects generated JSON. Cairn commands should not execute Nickel while validating Animus lifecycle artifacts.

### 4. Avoid hidden absolute paths in ordinary checks

Normal validation should be `cairn validate --root .` through the pinned Cairn app or equivalent. Absolute source paths are acceptable only in refresh/check tooling and diagnostics.

## Risks

- Generated policy drift can make validation misleading. Add a focused check or documented refresh process.
- Vendoring policy can hide upstream policy updates. Record source identity and refresh intentionally.
