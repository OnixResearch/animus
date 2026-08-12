# Proposal: Adapt Clankers process-job profiles for Animus

## Why

Animus already plans host-managed background queues, but long-running work also needs reusable project-defined profiles: named jobs with validated commands, backend choices, working-directory policy, environment rules, writable-path bounds, timeout/resource limits, and safe identity metadata. Clankers' process-job profile kit provides the right pattern: resolve profiles into backend-neutral start requests before any queue, shell, pueue, or systemd dispatch.

## What Changes

- Add Nickel-authored process-job profile contracts for profile schema, backend preference, command/program shape, args, cwd, writable paths, environment allowlist, timeout/resource bounds, notification policy, and safe metadata.
- Define pure profile resolution over global, workspace, and explicit sources with deterministic precedence and duplicate-profile rejection.
- Produce backend-neutral queue start requests plus safe identity envelopes without spawning processes or contacting queue backends.
- Reject secret-like environment keys, path escapes, unsupported backends, conflicting command/program fields, and policy-limit violations before dispatch.
- Add focused positive and negative fixtures for valid profiles, duplicate names, invalid command shapes, denied env/path fields, missing policies, unsupported backends, and generated-config drift.

## Impact

- **Files**: future `policy/process-jobs/` Nickel contracts, generated fixtures if needed, profile DTOs, resolver tests, docs, and Cairn artifacts.
- **Testing**: positive profile-resolution fixtures and negative duplicate/env/path/backend/shape/drift fixtures.
- **Non-claims**: this does not implement process supervision, pueue/systemd integration, or direct shell execution in reusable runtime crates.
