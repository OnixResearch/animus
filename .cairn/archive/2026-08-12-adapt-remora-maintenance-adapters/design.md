## Context

The reviewed source is `git@github.com:OnixResearch/remora.git` at revision `e3c857c857f4aa170d8a5b6286aad5855db541be`. The imported scope is limited to authority-free domain values, deterministic update grouping, neutral change-request and review plans, separate service boundaries, bounded failure classes, and metadata-only evidence patterns.

The user's request is the owner grant for OnixResearch-controlled Remora material. Both repositories declare `MIT OR Apache-2.0`. This change does not copy third-party source or claim that either public license changed.

Animus already owns generic schedule, queue, orchestration, semantic review, quality, model-evidence, authority, validation-planning, VCS snapshot, workspace-edit, and safe-receipt contracts. The new families must connect to these owners rather than replace them.

Completion requires two focused no-std crates, pure deterministic planners, exact request binding, separate injected ports, safe receipts, facade isolation, and positive and negative evidence. A single Remora compatibility monolith is false completion.

## Decisions

### Decision: split dependency maintenance from change requests

**Choice:** Add `animus-adapter-maintenance` and `animus-adapter-change-request` as separate `#![no_std]` plus `alloc` crates. Maintenance can depend on change-request contracts for prepared request drafts. Change-request contracts cannot depend on maintenance.

**Rationale:** Change requests and code review are useful outside dependency updates. The dependency direction keeps each capability independently reusable.

### Decision: adapt domain meaning, not application shells

**Choice:** Adapt Remora's authority-free values and reducers. Do not move its CLI, Cargo or Nix scanners, command generation, Git worktree implementation, provider HTTP adapters, scheduler state directories, or dashboard file layout.

**Rationale:** Those effects belong in product hosts. Importing them would violate Animus reusable-crate authority boundaries.

### Decision: use separate least-authority ports

**Choice:** Define separate ports for dependency inventory, update-candidate discovery, security-advisory observation, workspace update execution, validation execution, change-request fetch, review analysis, change-request publication, and review publication.

**Rationale:** A composite maintenance backend can hide unrelated filesystem, registry, model, VCS, forge, and credential authority.

### Decision: use trusted validation profiles instead of raw commands

**Choice:** Update plans request validation by safe profile identity and expected subject identity. They do not carry shell command text.

**Rationale:** Existing Animus scoped-validation contracts keep executable command selection in a trusted host profile. Raw commands in the maintenance core would duplicate policy and enable shell authority.

### Decision: preserve deterministic Remora grouping with stricter compatibility keys

**Choice:** Security and major updates are isolated. Routine updates can share a group only when ecosystem and manifest identities match. Every group remains bounded.

**Rationale:** Remora's bounded grouping is useful. Requiring matching ecosystem and manifest prevents accidental cross-tool or cross-workspace batches.

### Decision: bind every service result before reduction

**Choice:** Every request and observation carries a `MaintenanceBinding` or `ChangeRequestBinding`. The binding includes repository, policy, plan, generation, implementation, and correlation identities where applicable.

**Rationale:** Matching labels without matching immutable subject and policy identities permit stale or cross-repository replay.

### Decision: reuse existing Animus families by composition

**Choice:** Document and catalog composition with schedule, queue, orchestration, semantic review, model, quality, model-evidence, authority, validation-planning, workspace, VCS snapshot, and safe-receipt families. Do not duplicate these types inside the new crates.

**Rationale:** Remora-specific workflow meaning belongs in the new adapters. Generic execution, review authority, scheduling, and evidence meaning stay with their current owners.

### Decision: retain bounded source provenance

**Choice:** Export constants for the Remora repository, revision, declared license, imported scope, and owner-grant basis. Focused checks bind these values to the reviewed cohort.

**Rationale:** Owner-authorized reuse still requires exact source identity and imported-scope evidence.

## Adversarial Cases

- A candidate observation uses the correct dependency name but a different repository or policy identity.
- A security update is grouped with a routine version update.
- A routine group mixes ecosystems or manifests.
- A workspace service reports success for an older plan generation.
- A validation result uses an untrusted command instead of an admitted profile identity.
- A publication observation targets a different repository, branch, commit, or change request.
- A review comment requests an unobserved inline anchor and no summary fallback exists.
- A receipt includes dependency names, raw diffs, source text, credentials, provider payloads, commands, paths, or unsupported completion claims.
- A host omits one required service and the planner silently selects an ambient implementation.

Each case must fail before reusable state accepts the result or claims an effect.

## Risks / Trade-offs

- Two crates add more integration work than one broad compatibility crate.
- Strict identity binding rejects loose host integrations until they provide current observations.
- This change exposes reusable maintenance contracts but does not provide a complete dependency bot.
- Remora can continue to evolve independently, so future parity requires explicit cohort review.

## Non-Claims

This design does not implement or prove registry freshness, advisory completeness, update correctness, manifest or lockfile validity, VCS isolation, check sufficiency, forge delivery, review quality, signed commits, merge safety, exactly-once effects, production readiness, or release eligibility.
