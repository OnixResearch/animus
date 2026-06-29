# Release evidence index evaluation

Animus evaluates Clankers' release evidence index as a bounded future pattern: collect already-run check receipts, tie them to the exact repository head they cover, and state what the evidence does not prove. This page does not adopt Clankers' full harness, dogfood suites, live-provider checks, VM gates, or release tagging workflow.

## Scope

A future Animus evidence index should be a summary over existing evidence, not a hidden test runner. It may compose focused rail receipts, workspace tests, Clippy, flake checks, Nickel/config drift checks, Cairn validation, and Cairn gates. Missing modes stay missing unless a caller explicitly runs them and supplies fresh receipts.

Out of scope for this evaluation:

- live model/provider behavior;
- daemon or terminal UI dogfood;
- VM/package checks;
- remote services and credentials;
- plugin runtime execution;
- production readiness and release tagging.

## Schema model

The Nickel source `cairn/release-evidence-index-schema.ncl` defines the fixture-facing shape for a future index:

- a `head` record with VCS kind, commit, branch, and dirty status;
- `check` records with mode, pure/optional class, status, and receipt/head match;
- `claim` records for evidence surfaces the index asserts;
- `non_claim` records for omitted optional surfaces;
- `missing_mode` records for checks that are intentionally absent.

The required pure modes are focused rails, workspace tests, Clippy, flake checks, Nickel drift checks, Cairn validation, and Cairn gates. VCS collection is shell-owned so reusable core logic can validate supplied metadata without reading Git or Jujutsu state.

## Current-head evidence

Current-head claims are valid only when:

- the worktree was clean when evidence was collected;
- every required pure mode is present and passing;
- every selected receipt matches the recorded commit/head;
- missing modes are named as missing instead of being implied;
- optional checks are either present as passing optional evidence or listed as non-claims.

Dirty worktrees and stale receipts can remain historical evidence, but they are not current-head proof.

## Pure versus optional evidence

Pure evidence is credential-free and deterministic enough for default validation. Optional evidence depends on host-specific services such as live providers, daemon/TUI dogfood, VMs, remote services, plugin runtimes, or production-readiness checks. An index must not promote omitted optional checks into claims.

## Focused validation rail

Run the evaluation rail with:

```sh
scripts/cairn/check-release-evidence-index --root .
```

The rail validates the Nickel sample and fixtures for:

- a positive current-head pure-evidence index;
- stale receipt rejection;
- dirty-head rejection;
- missing-mode rejection;
- optional-surface overclaim rejection.

This is a fixture validation rail only. It is not a release generator and does not prove live providers, UI/daemon behavior, VM packaging, plugin runtime behavior, remote auth, performance, or production readiness.
