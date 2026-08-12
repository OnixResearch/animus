# Design: Adapt Goose session branching controls

Goose is useful here because its edit/fork/duplicate controls make session exploration explicit. Animus should preserve that product shape while anchoring it in neutral ledger and provenance contracts.

### 1. Branch controls are ledger plans

**Choice:** Introduce request and plan DTOs for edit-in-place, fork-from-message, duplicate-session, rename/label, and continuation target selection. Each request names source session identity, source ledger head, cut point, requested edit body class, branch label, copy-settings policy, and provenance.

**Rationale:** Reusable logic can validate identities, freshness, and branch relationships without reading session files or mutating host storage.

### 2. Edit-in-place is explicit destructive intent

**Choice:** Treat edit-in-place as a suffix-discard plan that requires policy admission, an explicit discarded-span summary, and checkpoint or rollback evidence before host storage can be changed. Forking from an edit preserves the original branch and creates a new active leaf instead.

**Rationale:** Goose's edit-in-place control is useful, but Animus must not silently erase ledger history or receipts.

### 3. Duplicate copies safe descriptors only

**Choice:** Duplicate-session planning can copy safe session settings, recipe references, extension labels, model profile labels, and policy labels, but must not copy credentials, live process handles, provider sessions, raw tool outputs, or host database rows.

**Rationale:** Duplicating ergonomics should not imply process or credential inheritance.

### 4. Receipts are metadata-only

**Choice:** Branch receipts record operation kind, source and target session hashes, cut-point hash, suffix span counts, branch label, policy hash, checkpoint status, continuation target, diagnostics, and a BLAKE3 receipt hash.

**Rationale:** Durable evidence should prove what branch operation happened without storing raw prompts, private transcripts, provider payloads, or tool bodies.

### Risks

- Destructive edit wording can mislead users if host UIs hide suffix loss. Mitigate with required suffix-discard metadata and policy prompts.
- Duplicate-session behavior may be mistaken for live process continuation. Mitigate with explicit non-claim fields and degraded continuity diagnostics.
- Branch labels can leak private context. Mitigate with safe label validation and hash-only fallback.
