## Phase 1: Export source and policy

- [x] [serial] Inventory session-tree, persistence, compaction, repair, dataset export, evidence export, safe summary, host-write, and receipt owners and define the human-debug export's exact non-overlap. r[agent_harness_runtime.session_tree_export.source] r[agent_harness_runtime.session_tree_export.verification]
- [x] [serial] Define validated export source, entry, topology, active-path, storage-order, diagnostic, omission, artifact, host-write, and receipt DTOs with named bounds. r[agent_harness_runtime.session_tree_export.source] r[agent_harness_runtime.session_tree_export.projection]
- [x] [serial] Author a typed Nickel redacted export policy for allowed entry classes, branch visibility, content/metadata posture, bounds, malformed-input behavior, formats, and receipt fields; generate and revalidate the Rust projection. r[agent_harness_runtime.session_tree_export.policy]

## Phase 2: Pure projection and rendering

- [x] [depends:agent_harness_runtime.session_tree_export.source] Implement source validation that requires a validated Animus snapshot/replay, active leaf, safe identities, and consistent hashes without reading session files or importing external formats. r[agent_harness_runtime.session_tree_export.source]
- [x] [depends:agent_harness_runtime.session_tree_export.policy] Implement redaction-first projection for roots, parent/child links, branch points, active path/leaf, compaction, branch summaries, repair markers, statuses, counts, omissions, safe summaries, and BLAKE3 identities. r[agent_harness_runtime.session_tree_export.projection]
- [x] [depends:agent_harness_runtime.session_tree_export.projection] Implement fail-closed duplicate/cycle/missing-parent/stale-leaf/unsupported-entry/hash diagnostics and prohibit malformed topology from rendering as a valid tree. r[agent_harness_runtime.session_tree_export.projection]
- [x] [depends:agent_harness_runtime.session_tree_export.projection] Implement deterministic, escaped, script-free, self-contained HTML plus a safe line-oriented metadata projection with fixed semantic structure, bounded sections, and no embedded styles or remote resources. r[agent_harness_runtime.session_tree_export.html] r[agent_harness_runtime.session_tree_export.determinism]
- [x] [depends:agent_harness_runtime.session_tree_export.determinism] Implement domain-separated artifact identity and metadata-only export receipt construction without ambient time, random IDs, source/destination paths, or private bodies. r[agent_harness_runtime.session_tree_export.receipts]

## Phase 3: Host write shell

- [x] [depends:agent_harness_runtime.session_tree_export.html] Add an explicit destination-capability writer with overwrite posture, file mode, byte budget, bounded errors, and no current-directory/home/global fallback. r[agent_harness_runtime.session_tree_export.host_write]
- [x] [depends:agent_harness_runtime.session_tree_export.host_write] Keep render bytes transient until destination admission succeeds and ensure write/rename/sync failures do not emit a successful export receipt. r[agent_harness_runtime.session_tree_export.host_write] r[agent_harness_runtime.session_tree_export.receipts]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive fixtures for a linear session, diverged branches, selected active path, empty selected branch, compaction, branch handoff, interrupted-tool repair markers, safe tool status, omitted private entries, deterministic repeated render, HTML and safe line formats, and explicit destination writes. r[agent_harness_runtime.session_tree_export.verification]
- [x] [parallel] Add negative fixtures for duplicate IDs, cycles, missing parents, stale active leaf, stale repair target, unsupported/custom private entries, invalid hashes, over-budget tree/depth/body, malformed policy, destination denial, partial write, overwrite denial, and unsafe receipt fields. r[agent_harness_runtime.session_tree_export.verification]
- [x] [parallel] Add injection/leak fixtures across titles, labels, summaries, tool metadata, errors, custom entries, and source labels for HTML/script/style/URL injection, raw prompts, tool arguments/results, hidden reasoning, credentials, provider payloads, environment data, absolute paths, and unbounded output. r[agent_harness_runtime.session_tree_export.policy] r[agent_harness_runtime.session_tree_export.html] r[agent_harness_runtime.session_tree_export.verification]
- [x] [depends:agent_harness_runtime.session_tree_export.determinism] Add byte-for-byte determinism tests across locale, terminal width, process environment, repeated execution, and host clock changes. r[agent_harness_runtime.session_tree_export.determinism] r[agent_harness_runtime.session_tree_export.verification]

## Phase 5: Documentation and closeout

- [x] [serial] Update session, compaction, repair, export, safety, component catalog, runtime facade, focused rails, and README reference docs; distinguish redacted human export from raw journal administration, dataset export, and release evidence. r[agent_harness_runtime.session_tree_export.policy] r[agent_harness_runtime.session_tree_export.receipts]
- [x] [serial] Add a focused session-tree-export rail for Nickel drift, topology validation, redaction, escaping, bounds, determinism, host writes, receipts, and positive/negative fixtures. r[agent_harness_runtime.session_tree_export.verification]
- [x] [serial] Run focused runtime/export/std tests, formatting, Clippy, Nickel checks, safe-receipt/evidence rails, session/ledger/compaction tests, Cairn validation/gates, and relevant Nix checks. r[agent_harness_runtime.session_tree_export.verification]
