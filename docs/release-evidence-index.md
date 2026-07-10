# Release evidence index

Animus uses a bounded release evidence index pattern: collect already-run check receipts, tie them to the exact repository head they cover, and state what the evidence does not prove. This does not adopt Clankers' full harness, dogfood suites, live-provider checks, VM gates, production-readiness claims, or release tagging workflow.

## Scope

An Animus evidence index is a summary over supplied evidence, not a hidden test runner. It may compose focused rail receipts, workspace tests, Clippy, flake checks, Nickel/config drift checks, Cairn validation, Cairn gates, fixture-backed provider stream parser checks, packaged headless dogfood smoke checks, and model-router execution service checks. Missing modes stay missing unless a caller explicitly runs them and supplies fresh receipts.

Out of scope:

- live model/provider behavior beyond fixture-backed parser and service-contract checks;
- daemon or terminal UI dogfood;
- VM/package checks;
- remote services and credentials;
- plugin runtime execution;
- production readiness and release tagging.

## Schema model

The Nickel source `cairn/release-evidence-index-schema.ncl` defines the fixture-facing line format:

- a `head` record with VCS kind, commit, branch, and dirty status;
- `check` records with mode, pure/optional class, status, and receipt/head match;
- `claim` records for evidence surfaces the index asserts;
- `non_claim` records for omitted optional surfaces;
- `missing_mode` records for checks that are intentionally absent.

The required pure modes for a `current-head-pure-evidence` claim are focused rails, workspace tests, Clippy, flake checks, Nickel drift checks, Cairn validation, and Cairn gates. The packaged headless dogfood smoke is a recognized pure selected check that can be included alongside the required modes. VCS collection is shell-owned so reusable validation logic consumes supplied metadata instead of reading Git/Jujutsu state.

## Current-head generator

`scripts/cairn/generate-current-head-release-evidence-index` renders an index from explicit check statuses. By default it collects the current Git head, branch, and dirty status from the shell. Tests and Nix checks may supply `--commit`, `--branch`, and `--dirty` for synthetic fixtures.

Example draft that records actual head metadata but does not claim full current-head proof:

```sh
scripts/cairn/generate-current-head-release-evidence-index \
  --out target/cairn/release-evidence/current-head-draft.lines \
  --check focused-rails=pass \
  --check packaged-headless-dogfood-smoke=pass \
  --missing-mode workspace-tests=not-run \
  --missing-mode clippy=not-run \
  --missing-mode flake-check=not-run \
  --missing-mode nickel-drift=not-run \
  --missing-mode cairn-validate=not-run \
  --missing-mode cairn-gates=not-run
```

Only add `--claim-current-head` after every required pure mode has fresh passing evidence for the same recorded head.

## Current-head evidence rule

Current-head claims are valid only when:

- the worktree was clean when evidence was collected;
- every required pure mode is present and passing;
- every selected receipt matches the recorded commit/head;
- missing modes are named as missing instead of being implied;
- optional checks are either present as passing optional evidence or listed as non-claims.

Dirty worktrees and stale receipts can remain historical or draft evidence, but they are not current-head proof.

## Pure versus optional evidence

Pure evidence is credential-free and deterministic enough for default validation. Optional evidence depends on host-specific services such as live providers, daemon/TUI dogfood, VMs, remote services, plugin runtimes, or production-readiness checks. An index must not promote omitted optional checks into claims.

## Focused validation rails

Run the schema/fixture rail:

```sh
scripts/cairn/check-release-evidence-index --root .
nix build .#checks.x86_64-linux.release-evidence-index --no-link -L
```

Run the current-head generator rail:

```sh
scripts/cairn/check-current-head-release-evidence-index --root .
nix build .#checks.x86_64-linux.current-head-release-evidence-index --no-link -L
```

The first rail validates Nickel sample fixtures for positive current-head evidence, stale receipt rejection, dirty-head rejection, missing-mode rejection, and optional-surface overclaim rejection. The second rail validates generated positive, dirty, missing, and stale synthetic indexes, then writes an actual-head draft under `target/cairn/release-evidence/` when a Git worktree is available.

Dogfood-specific claims must also satisfy `scripts/cairn/check-dogfood-readiness --root .` so omitted optional host surfaces remain explicit non-claims.
