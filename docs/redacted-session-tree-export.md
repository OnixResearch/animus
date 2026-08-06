# Redacted session-tree export

The [session branch explorer](session-branch-explorer.md) uses the same redaction-first topology boundary for interactive map facts. Export artifacts and TUI state remain separate consumers.

Animus exposes a deterministic human-debug export for already-loaded, validated session metadata. It preserves branch topology and storage order without accepting transcript bodies. It is not a raw journal backup, training dataset, or release-evidence export.

## Boundary

The pure `animus_runtime::session_tree_export` module accepts typed metadata supplied after session snapshot and replay validation:

- safe session, snapshot, replay, entry, and receipt identities;
- one current active leaf;
- parent links and contiguous storage sequence numbers;
- admitted entry kinds, safe status classes, bounded summaries, and omission counts;
- explicit repair-target identities for repair markers.

It does not read journals, paths, provider formats, Pi/Tau/Claude Code state, environment variables, clocks, terminals, or network services. Duplicate identities or sequences, cycles, missing parents, stale active leaves, stale repair targets, unsupported kinds, invalid hashes, and exceeded bounds fail before rendering.

## Policy

`config/session-tree-export-policy.ncl` is the typed source of truth. `config/generated/session-tree-export-policy.json` is the checked Rust input. The runtime recomputes the domain-separated BLAKE3 policy identity before accepting it.

The initial profile is intentionally narrow:

- all branches remain visible only as redacted metadata;
- tool metadata is status-only;
- private custom entries are rejected;
- raw prompts, assistant bodies, reasoning, tool bodies, provider payloads, credentials, environment values, paths, and raw errors are outside the input contract;
- receipt field classes are exact and cannot be widened by a generated file.

Policy content identity: `828fed5de19fe77a8e7a7050778113872d686343cd2a997070cf5e30930284db`.

## Projection and artifacts

Projection preserves the validated storage order while computing parent depth, direct child counts, branch points, active-path membership, compaction markers, branch summaries, repair markers, statuses, and omission counts. Source, projection, artifact, and receipt identities use separate BLAKE3 domains.

The HTML renderer emits fixed semantic markup with escaped text, no scripts, no styles, no links, and no remote resources. The safe-lines renderer escapes backslash and control separators. Both renderers enforce policy byte budgets and omit ambient timestamps, locale, terminal dimensions, random values, host paths, and environment state. Equal inputs therefore produce equal bytes and artifact identities.

## Host write authority

Reusable runtime code only returns bytes and metadata. `animus_std` owns the thin host shell:

1. the caller supplies a safe destination-capability label, overwrite posture, private mode `0o600`, and byte budget;
2. admission completes before bytes reach the injected `SessionTreeExportWriteService`;
3. the service resolves the capability privately and performs create/replace, rename, permission, and synchronization work;
4. success requires exact artifact hash, byte count, commit, and synchronization evidence;
5. missing service, missing destination capability, denied overwrite, invalid mode, over-budget input, partial write, failed synchronization, or mismatched identity fails without a success result.

No current-directory, home-directory, session-directory, or global fallback exists. Destination paths are never receipt fields.

## Receipts and non-claims

Receipt projection requires exact validated write evidence and rejects every sensitive draft field. Receipts bind policy, source, projection, artifact, format, byte count, entry count, active-path count, branch-point count, omission count, redaction state, and safe write status. They do not contain source or destination paths or private bodies.

An export receipt proves only deterministic projection of the supplied validated metadata. It does not prove session correctness, omitted-content completeness, behavioral correctness, release eligibility, filesystem durability, exactly-once effects, or training-data suitability.

## Verification

Run:

```sh
scripts/cairn/check-session-tree-export --root .
```

The focused rail checks Nickel drift and negative fixtures, pure-core authority, topology and redaction tests, deterministic renderers, host-write failure handling, metadata-only receipts, companion session/compaction/repair tests, Clippy, feature composition, and no-default-feature boundaries. It writes a bounded receipt to `target/cairn/focused-rails/session-tree-export.md`.
