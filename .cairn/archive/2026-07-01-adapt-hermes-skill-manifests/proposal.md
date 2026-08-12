# Proposal: Adapt Hermes skill manifests for Animus

## Why

Hermes skills provide a useful portable manifest shape: a small frontmatter contract advertises a workflow by name, description, version, tags, related skills, and category, while the larger body remains load-on-demand. Animus already plans typed resource discovery; the Hermes manifest pattern should refine that model so hosts can expose reusable workflows with stable metadata, deterministic validation, and progressive disclosure without copying Hermes' local directory layout.

## What Changes

- Define an Animus-owned skill manifest descriptor compatible with Hermes-style metadata fields while remaining host-provided.
- Add validation for names, descriptions, versions, categories, tags, related-skill links, body presence, and safe content identity.
- Map validated manifests into the existing resource descriptor and skill-disclosure flow.
- Keep Hermes local state, user-specific categories, symlinks, and raw runtime directories outside reusable Animus crates.
- Add positive and negative fixtures using representative Hermes `SKILL.md` shapes.

## Impact

- **Files**: future resource DTOs, skill manifest parser/validator modules, fixture manifests, prompt construction tests, and docs.
- **Testing**: valid manifest acceptance plus missing-description, overlong-description, invalid-name, empty-body, duplicate-tag, broken-related-skill, and unsafe-content-handle rejection.
- **Non-claims**: this change does not adopt `~/.hermes/skills`, Hermes skill installation commands, category naming as policy, or any Hermes-specific runtime cache/state path.
