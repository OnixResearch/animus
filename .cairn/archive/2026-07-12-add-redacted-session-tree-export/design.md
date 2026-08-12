## Context

Animus sessions are append-only trees selected by an active leaf. Existing dataset export intentionally reshapes selected replay context for training-oriented rows, while evidence exports omit private transcript material. A human debugging artifact has different goals: preserve branch topology and storage order, explain compaction and repair, and remain safe to share by default.

Tau renders a self-contained HTML tree and entry details from JSONL. Animus will adapt the two coordinated views but project from validated Animus data under an explicit redaction policy before any markup or file I/O.

## Decisions

### 1. Consume validated projections, not storage files

**Choice:** Pure export accepts a validated session snapshot, replay report, active-leaf selection, safe ledger metadata, and optional safe receipt references. It does not open journals, detach missing parents heuristically, infer formats, or import provider-native transcripts.

**Rationale:** Storage validation and private-content admission must finish before presentation begins.

### 2. Make redaction policy a typed Nickel source

**Choice:** Nickel defines the export schema, allowed entry classes, active/sibling visibility, summary limits, tool metadata posture, path posture, custom-entry posture, malformed-input behavior, output formats, and receipt fields. Generated policy is revalidated in Rust.

**Rationale:** Shareability and omission choices are reviewable policy, not scattered rendering conditionals.

### 3. Ship a redacted-only initial profile

**Choice:** The default and initially supported product profile excludes raw system prompts, user prompts, assistant bodies, hidden reasoning, tool names when not explicitly safe, tool arguments, tool outputs, provider payloads, credentials, private paths, environment data, raw errors, and private custom entries. It emits safe summaries, counts, kinds, status classes, and BLAKE3 identities.

**Rationale:** Tau's full-fidelity export is useful locally but unsafe as an Animus default. Raw journal copy remains a distinct administrative action.

### 4. Preserve both topology and storage order

**Choice:** The pure projection produces a tree view with roots, parent/child links, branch points, active path, active leaf, compaction summaries, branch summaries, and repair markers, plus a storage-order detail view over admitted entries. Leaf-pointer plumbing may be summarized rather than rendered as transcript content.

**Rationale:** A flat transcript hides why replay selected or omitted entries.

### 5. Fail closed on malformed topology

**Choice:** Duplicate IDs, cycles, missing required parents, invalid active leaves, stale repair targets, unsupported entry kinds, or inconsistent hashes produce a bounded diagnostics-only projection or an error according to policy. They are never rendered as a valid session tree.

**Rationale:** Best-effort tree drawing can make corrupt history look authoritative.

### 6. Keep projection deterministic

**Choice:** Ordering derives from validated storage order plus explicit parent relationships. Artifact content excludes ambient generation time, random IDs, host paths, locale, and terminal width. Titles and source labels are explicit safe inputs. Projection and artifact identities use domain-separated BLAKE3.

**Rationale:** Identical snapshot and policy inputs should produce byte-identical artifacts.

### 7. Render script-free self-contained HTML

**Choice:** HTML uses fixed semantic markup, escaped text and attributes, no JavaScript, no embedded styles, no remote fonts/assets, no external links generated from session data, and bounded sections. A safe line-oriented metadata projection may be offered for automation.

**Rationale:** Session fields are untrusted content. Static escaped markup minimizes injection and offline-viewing risks.

### 8. Keep file writes in the host shell

**Choice:** The renderer returns bytes plus identity and bounds metadata. A host writer requires an explicit destination capability, overwrite posture, file mode, and output budget. It creates no default directory and does not expose the private store location as a source label.

**Rationale:** Destination selection and mutation are host authority.

### 9. Reuse safe export primitives without conflating products

**Choice:** Shared redaction, bounded-summary, hash, and leakage primitives may be reused from dataset/evidence export. Session-tree export has its own DTOs and claims and does not emit training rows or release evidence automatically.

**Rationale:** Policy consistency is valuable; product semantics must remain distinct.

### 10. Receipts describe projection, not content

**Choice:** Receipts include schema/policy/snapshot/artifact hashes, admitted and omitted counts, branch and active-path counts, diagnostic classes, output format, byte count, redaction status, and explicit non-claims. They omit source paths, destination paths, raw text, tool data, and private entry bodies.

**Rationale:** Export evidence should be reviewable without reproducing the export.

## Risks / Trade-offs

- Heavy redaction can reduce debugging value. Safe local handles may be added later without embedding private bodies.
- HTML escaping and bounded rendering need adversarial fixtures across every field and custom entry class.
- Deterministic output avoids generation timestamps; operators needing time metadata must keep it outside the hashed artifact or supply an explicit logical observation.

## Non-Goals

- Full-fidelity transcript sharing or raw journal backup.
- Interactive JavaScript navigation, remote assets, or web serving.
- Training-dataset, release-evidence, or behavioral-correctness claims.
- Reading provider, Pi, Tau, Claude Code, or external agent session files directly.
- Repairing malformed sessions during export.
