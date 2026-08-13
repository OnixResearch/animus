# Remora-derived adapter families

Animus adapts Remora maintenance meaning through two optional `#![no_std]` crates. The crates contain typed values, pure plans, injected ports, and safe receipts.

They do not form a dependency bot. A host must supply every registry, advisory, workspace, validation, VCS, model, forge, scheduler, and storage effect.

## Reviewed source

- Repository: `git@github.com:OnixResearch/remora.git`
- Revision: `e3c857c857f4aa170d8a5b6286aad5855db541be`
- Declared license: `MIT OR Apache-2.0`
- Owner grant: user-requested OnixResearch owner adaptation
- Source-cohort identity: `b3:a084a8a06727b1fae59377d34754f224065351e550caadf4e12c0bbcdcc26e42`

The reviewed source cohort contains these files:

- `crates/remora-core/src/domain.rs`
- `crates/remora-core/src/effect.rs`
- `crates/remora-core/src/planning.rs`
- `crates/remora-core/src/review.rs`

The adapted scope is authority-free domain data, deterministic grouping, review planning, service boundaries, failure classes, and metadata-only evidence.

This owner grant does not change the public license. It does not grant rights for third-party or contributor-owned material outside OnixResearch control.

## Family split

| Family | Direct crate | Optional facade | Owned meaning |
| --- | --- | --- | --- |
| Change requests | `animus-adapter-change-request` | `animus_adapters::change_request`; `animus::change_request_adapters` | repository and forge DTOs, diff observations, review plans, CI triage, publication requests, exact bindings, and receipts |
| Maintenance | `animus-adapter-maintenance` | `animus_adapters::maintenance`; `animus::maintenance_adapters` | inventory DTOs, candidates, advisories, grouping, update plans, validation-profile requests, exact bindings, and receipts |

The maintenance crate depends on change-request contracts only for prepared request drafts. The change-request crate does not depend on maintenance.

Both facade features are off by default. Direct crate dependencies remain the smallest integration surface.

## Functional core

`plan_code_review` validates repository, branch, commit, diff-side, line, policy, confidence, repair, and bound facts. It emits effect intents only.

`plan_update_run` isolates security and major updates. It groups routine updates only when ecosystem and manifest identities match.

Each update group binds complete candidate material with BLAKE3. A changed version, risk, advisory, order, or group number changes the group identity.

Neither planner reads manifests, diffs, clocks, environment values, credentials, files, networks, registries, advisory databases, or forge state.

## Least-authority ports

The change-request family has four separate ports:

1. `ChangeRequestSourcePort`
2. `ReviewAnalysisPort`
3. `ChangeRequestPublicationPort`
4. `ReviewPublicationPort`

The maintenance family has five separate ports:

1. `DependencyInventoryPort`
2. `UpdateCandidatePort`
3. `SecurityAdvisoryPort`
4. `WorkspaceUpdatePort`
5. `ValidationPort`

Together, the families expose nine separate ports. No port grants authority to another port.

A service descriptor states its family, implementation identity, and availability. The port family is its capability ceiling.

Missing services return `MissingService`. Denied, unsupported, unavailable, or stale descriptors return `ServiceBlocked` with the exact availability class.

Service calls can also return typed denied, unavailable, unsupported, cancelled, timed-out, malformed, stale, mismatch, overflow, or infrastructure failures.

The crates do not discover replacements. A missing or blocked service stops before host dispatch.

## Exact binding

Every request carries a `ChangeRequestBinding` or `MaintenanceBinding`. The binding includes these identities:

- repository;
- policy;
- plan;
- generation;
- implementation;
- correlation.

Operation DTOs add the applicable branch, commit, change-request, manifest, candidate, group, validation-profile, workspace-subject, and evidence identities.

Validation rejects stale or mismatched observations. Review publication also checks the exact planned inline count and summary outcome.

Inline comments require an observed commit, file, side, terminal line, and optional range start. The planner does not infer provider positions.

## Composition owners

These families add maintenance-specific meaning only. Existing Animus components retain their authority and policy:

| Concern | Existing owner |
| --- | --- |
| schedules and missed-run policy | `animus-runtime::schedule_reliability` and scheduled-workflow contracts |
| queues and orchestration | existing queue contracts and `animus-adapters::orchestration` |
| semantic review and model calls | existing review contracts and `ModelAdapter` |
| quality and radar summaries | quality, codegraph, document, and model-evidence families |
| authority admission | existing host-authority and runtime seal gates |
| validation command selection | existing scoped validation profiles |
| workspace mutation | `animus-std-workspace-edit` or another explicit host |
| VCS observations | existing VCS snapshot or injected VCS adapters |
| safe workflow receipts | existing safe-receipt and Cairn evidence owners |

A composition root may schedule a maintenance plan. The maintenance crate cannot start a timer, enqueue work, execute commands, or publish changes.

## Receipt boundary

Receipts contain operation, status, count, binding-derived BLAKE3 identities, and metadata-only disclosure posture.

Receipt validation rejects raw content and unsupported claims. Raw dependencies, versions, manifests, diffs, comments, commands, logs, credentials, payloads, and paths stay outside receipts.

A receipt does not prove host behavior. It does not prove registry freshness, advisory completeness, update correctness, validation sufficiency, forge delivery, or review quality.

It also does not prove merge safety, production readiness, or release eligibility.

## Verification

Run the focused rail from the repository root:

```sh
scripts/cairn/check-remora-adapter-families --root .
```

The rail checks source provenance, static authority boundaries, positive and negative tests, Clippy, formatting, no-default facades, Wasm, dependency isolation, catalogs, and Cairn metadata.

The Nix check is:

```sh
nix build .#checks.x86_64-linux.remora-adapter-families --no-link -L
```
