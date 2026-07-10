# Design: Add roadmap DAG explanations

## Context

The roadmap rail already parses Nickel-exported records and validates active change membership, prerequisite references, cycles, and product-surface safety. Its output is intentionally bounded but terse. As the active change set grows, maintainers need DAG explanations that show safe ordering and blockers without leaking local paths or command output.

## Decisions

### 1. Nickel remains source of truth

**Choice:** Continue authoring roadmap data in `cairn/roadmap.ncl`. Any DAG explanation consumes the existing exported `roadmap_lines` records and active change directory names.

**Rationale:** The explanation layer should not introduce a second roadmap format.

### 2. Explanation logic is pure over loaded records

**Choice:** Split or preserve a pure core that takes roadmap records, active change IDs, and accepted requirement IDs, then returns topological batches, blocked-change explanations, safety paths, and issue summaries. The script shell owns Nickel invocation, filesystem reads, stdout, and optional artifact writes.

**Rationale:** The same logic should be testable with fixture records and no repository checkout side effects.

### 3. Rendered graph evidence is safe by construction

**Choice:** Markdown or DOT-style review output may contain only change IDs, roles, tiers, prerequisite IDs, issue codes, counts, and safe labels. It must not include raw proposal/design/task bodies, private file paths, command logs, environment values, provider payloads, credentials, or raw prompts.

**Rationale:** Roadmap evidence belongs in closeout summaries and must stay durable and redacted.

### 4. Product-safety paths are first-class explanations

**Choice:** For product-surface changes, the explanation should show whether transitive prerequisites include accepted foundational and runtime safety markers, and name the safe marker path or the missing marker class.

**Rationale:** Maintainers need to know why a product surface is safe or blocked, not only that a rail failed.

## Risks / Trade-offs

- More output can obscure the failure. Keep default output concise and put expanded graph reviews behind a flag or ignored artifact path.
- DOT renderings can imply UI behavior. Treat them as review artifacts only, not as a product graph visualizer.
- The roadmap checker is currently a script-owned Rust utility. If it grows, preserve a functional core and a thin shell rather than mixing filesystem reads with graph logic.
