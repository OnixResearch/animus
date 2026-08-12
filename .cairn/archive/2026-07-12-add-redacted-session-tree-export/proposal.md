## Why

Animus can export policy-controlled training rows and safe evidence, but it lacks a human-readable view of a branchable session. Operators need to inspect active paths, sibling branches, compaction points, repair records, and storage order without running a TUI or reading a private journal directly.

Tau's self-contained tree-plus-transcript HTML export is a useful product reference, but its full-fidelity rendering of prompts, tool arguments, tool results, and source paths is not an acceptable Animus default. Animus should add a deterministic, script-free, redaction-first export over validated session projections.

## What Changes

- Export only from validated Animus session snapshots, replay reports, and safe ledger projections. r[agent_harness_runtime.session_tree_export.source]
- Define a typed Nickel export policy whose default excludes raw prompts, tool arguments/results, hidden reasoning, credentials, provider payloads, and private paths. r[agent_harness_runtime.session_tree_export.policy]
- Build a pure ordered projection containing active-path, branch, parent, compaction, repair, status, count, and BLAKE3 identity metadata. r[agent_harness_runtime.session_tree_export.projection]
- Render deterministic self-contained HTML and safe line-oriented interchange with escaping, bounded content, and no scripts or remote resources. r[agent_harness_runtime.session_tree_export.html] r[agent_harness_runtime.session_tree_export.determinism]
- Keep destination selection and writes in an explicit host shell. r[agent_harness_runtime.session_tree_export.host_write]
- Emit safe export receipts and positive/negative malformed-tree, injection, and leakage fixtures. r[agent_harness_runtime.session_tree_export.receipts] r[agent_harness_runtime.session_tree_export.verification]

## Impact

- **Runtime core**: gains export-policy validation and pure session-tree projection over already-loaded validated data.
- **Host shell**: writes an artifact only to an explicitly admitted destination; it does not discover a default global export directory.
- **Privacy**: the initial product surface is redacted-only. Copying or exporting a raw private journal remains a separate administrative operation outside this exporter.
- **Claims**: export identity proves deterministic projection of the supplied validated snapshot under one policy. It does not prove session correctness, behavioral correctness, completeness of omitted private content, release eligibility, or suitability as a training dataset.
