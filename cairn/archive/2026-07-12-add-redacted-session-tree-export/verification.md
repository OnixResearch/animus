# Verification: redacted session-tree export

## Baseline

Before implementation, proposal, design, and tasks gates passed for `add-redacted-session-tree-export`. Focused dataset-export, session-persistence, and compaction baselines also passed. Existing contracts established separate owners for active-leaf ledger replay, persistence, compaction and branch handoff, interrupted-tool repair, training-row export, safe evidence, and host writes; none owned a redacted human topology artifact.

## Implemented boundary

The implementation uses `config/session-tree-export-policy.ncl` as the typed source of truth, checks in `config/generated/session-tree-export-policy.json`, and recomputes policy content identity `828fed5de19fe77a8e7a7050778113872d686343cd2a997070cf5e30930284db` in Rust. The initial profile admits all branches as redacted metadata, status-only tool metadata, no private custom entries, metadata-only content, omitted paths, forbidden scripts and remote resources, HTML and safe-lines formats, and an exact receipt field set.

`animus_runtime::session_tree_export` is a functional core over caller-supplied already-loaded metadata. It:

- validates safe session/snapshot/replay/entry/receipt identities, one root, contiguous storage sequence, exact active leaf, parent existence, cycles across every entry, prior-only repair targets, policy bounds, and allowed kinds;
- projects storage order, parent/depth/child relationships, branch points, active path, compaction and branch-summary markers, repair targets, statuses, omissions, and separate source/projection identities;
- emits fixed semantic topology and storage-order HTML views with complete escaping and no scripts, embedded styles, links, or remote resources;
- emits safe-lines metadata with escaped separators;
- enforces exact artifact byte budgets and domain-separated BLAKE3 artifact identity;
- accepts no filesystem path, provider format, raw journal, clock, locale, terminal, process environment, randomness, credential, or persistence service; and
- constructs metadata-only receipts only after exact capability-label, private-mode, byte-budget, artifact-hash, byte-count, committed, synchronized, and committed-status write evidence validates.

`animus-std` owns the imperative shell. It loads only an explicit generated-policy path and calls only an injected `SessionTreeExportWriteService` after destination-capability, overwrite, mode `0o600`, and byte-budget admission. Missing destination/service, denied overwrite, service failure, partial commit, failed synchronization, identity mismatch, and byte-count mismatch return errors. No current-directory, home, session-store, or global destination fallback exists.

Raw prompts, assistant bodies, tool bodies, hidden reasoning, provider payloads, credentials, environment values, private paths, raw errors, and private custom entries are outside artifact and receipt authority. Raw journal backup, training dataset export, release evidence, session correctness, omitted-content completeness, exactly-once effects, filesystem durability, interactive web serving, and production readiness remain explicit non-claims.

## Focused evidence

The final implementation passed:

- `scripts/cairn/check-session-tree-export --root .` — Nickel drift and positive/negative fixtures, pure-core authority scan, 8 runtime tests, 4 std host tests, dataset/session/compaction/repair companions, Clippy, root feature composition, and no-default-feature checks;
- focused receipt `target/cairn/focused-rails/session-tree-export.md` with source BLAKE3 `05d441dcc591e6425d72bd4acc5e4e07026a20b70165f1955870fed2e4f7ef53`;
- `nix build .#checks.x86_64-linux.session-tree-export --no-link -L`;
- `scripts/hooks/octet-lints` with all session-export Tiger Style findings resolved structurally;
- `scripts/config/check-nickel-configs`, component-catalog and runtime-facade inventory checks, safe-receipt/artifact, neutral-ledger, and interrupted-tool-repair companion rails;
- `cargo test --workspace --locked --no-fail-fast`;
- `cargo check --workspace --all-targets --locked`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `pre-commit run --all-files` — Nickel configs, router policy, and Octet hooks passed;
- Cairn validation plus proposal, design, and tasks gates; and
- `nix flake check -L`, with the configured unavailable remote builder warning followed by successful local fallback.

Repeated rendering tests prove byte-for-byte equality for equal inputs. Independence from locale, terminal width, process environment, host clocks, and random IDs is additionally enforced structurally: those values are absent from the pure API and the focused authority scan rejects ambient access in the runtime source.

The pre-sync repository-wide Tracey run remains pre-existingly invalid at 2,170 requirements and 652 referenced markers. All eight `session_tree_export` IDs appeared only in the expected pre-sync dangling set and none appeared in the missing set. After accepted-spec sync, Tracey reports 2,178 requirements and 660 referenced markers; no `session_tree_export` ID appears in either the dangling or missing diagnostics.

## Bounded adversarial audit

The implementation compared full-fidelity journal rendering, best-effort topology repair, active-branch-only rendering, and redaction-first validated-metadata projection. Full-fidelity rendering was rejected because prompts, tool bodies, provider payloads, paths, and credentials would enter shareable artifacts. Best-effort topology repair was rejected because corrupt history could look authoritative. Active-branch-only rendering was rejected because it hides sibling topology. Redaction-first projection was selected because it preserves reviewable branch/storage facts while keeping private bodies outside the type and receipt boundaries.

A secondary adversarial review proposed duplicate/missing-parent, budget inflation, marker evasion, removed-parent, script-injection, concurrent-write, and non-claim checks. Duplicate/missing-parent and script cases are directly covered. Artifact size is measured from rendered bytes and bounded by both typed policy and host request. Parents are not removed because all admitted branches remain metadata-visible. Marker scanning is defense in depth over host-validated safe summaries and is not claimed to detect arbitrary secrets or prove omitted-content completeness. Concurrent filesystem behavior remains host-owned and exactly-once/durability are explicit non-claims. Receipt non-claims are fixed constructor output rather than caller text.

## Non-claims

This evidence does not prove caller metadata honesty, arbitrary secret detection, source hash semantics, session correctness, transcript completeness, provider behavior, destination capability resolution correctness, filesystem atomicity or durability, concurrent writer serialization, exactly-once effects, release eligibility, training-data suitability, or production readiness.
