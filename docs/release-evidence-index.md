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
- `missing_mode` records for checks that are intentionally absent;
- one optional `real_check_receipt` record with the receipt schema, rail, live status, clean head, source BLAKE3, selected check ids, and pinned Cairn binding.

The required pure modes for a `current-head-pure-evidence` claim are focused rails, workspace tests, Clippy, flake checks, Nickel drift checks, Cairn validation, and Cairn gates. The separate `packaged-headless-dogfood` claim requires only a clean head and one passing, head-matched `packaged-headless-dogfood-smoke` receipt.

The separate `bounded-real-check-completion` claim requires one exact live receipt for the clean indexed head. The receipt must name `nickel-config,cairn-validate`, `pinned-cairn-core`, and a canonical 64-character BLAKE3 source identity. Neither bounded claim implies full current-head proof or release readiness. VCS collection is shell-owned so reusable validation logic consumes supplied metadata instead of reading Git/Jujutsu state.

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

After the packaged smoke passes on a clean head, generate the bounded dogfood claim:

```sh
scripts/cairn/generate-current-head-release-evidence-index \
  --out target/cairn/release-evidence/packaged-dogfood.lines \
  --check packaged-headless-dogfood-smoke=pass \
  --claim packaged-headless-dogfood
```

This command records supplied status. It does not run the packaged smoke.

After the real-check smoke passes on a clean head, generate its separate bounded claim:

```sh
scripts/cairn/check-real-check-completion-host-smoke --root . --live
scripts/cairn/generate-current-head-release-evidence-index \
  --out target/cairn/release-evidence/bounded-real-check.lines \
  --claim bounded-real-check-completion \
  --real-check-receipt target/cairn/focused-rails/real-check-completion-host-smoke.md
scripts/cairn/check-release-evidence-index --root . \
  --index target/cairn/release-evidence/bounded-real-check.lines
```

The generator reads only the explicit receipt. It does not run the smoke or infer release readiness.

## Current-head evidence rule

Current-head claims are valid only when:

- the worktree was clean when evidence was collected;
- every required pure mode is present and passing;
- every selected receipt matches the recorded commit/head;
- a real-check claim has exact live, pass, source, selected-check, and pinned-Cairn metadata;
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

The first rail validates Nickel sample fixtures for positive current-head evidence, stale receipt rejection, dirty-head rejection, missing-mode rejection, optional-surface overclaim rejection, packaged-dogfood binding, and exact real-check receipt binding. The second rail validates generated full-pure, packaged-dogfood, and bounded real-check claims. It also rejects missing, stale, wrong-check, and unsafe receipt data, then writes an actual-head draft when a Git worktree is available.

`scripts/cairn/check-dogfood-readiness --root .` composes the first rail with claim-hygiene validation. This single command rejects missing, stale, duplicate, or dirty packaged-dogfood receipts.
