# Design: Adapt Zerostack context file layering for Animus

## Context

Zerostack's context loader walks from the current directory upward, collecting project guidance and optional architecture documents. Animus should adapt the source layering idea as host-owned discovery feeding typed context references into prompt assembly.

## Decisions

### 1. Context classes are explicit

**Choice:** Define context file classes such as agent guidance, architecture guidance, project policy note, and host note.

**Rationale:** Different file classes have different trust, freshness, and prompt precedence expectations.

### 2. Discovery is a host plan

**Choice:** Runtime core receives discovered context descriptors in memory. Host adapters own directory walks, file reads, ignore rules, and path canonicalization.

**Rationale:** Reusable core must not gain ambient filesystem traversal authority.

### 3. Admission precedes prompt influence

**Choice:** Discovered context requires policy admission before prompt assembly, with source identity, trust scope, precedence, size bounds, and redaction classification.

**Rationale:** Project files can contain instructions, secrets, or stale guidance. Prompt influence must be attributable.

### 4. Omission is observable

**Choice:** Prompt assembly returns omission diagnostics and receipts for disabled, untrusted, stale, oversized, malformed, or unsafe context files.

**Rationale:** Operators need to know why expected guidance did or did not influence the model.

## Risks

- Ancestor files can unexpectedly override local guidance. Make precedence explicit and test conflicts.
- Naming compatibility with other agents can imply trust. File names should classify candidates but not automatically authorize them.
