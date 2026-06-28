# Design: Adapt OpenAI Codex dedicated diff review mode for Animus

## Context

Codex's review mode runs a dedicated reviewer over selected changes and produces findings rather than edits. Animus can make this a first-class runtime profile that uses the same model/tool infrastructure but sharply constrains side effects and output shape.

## Decisions

### 1. Review input is a VCS-neutral change set

**Choice:** Define change-set selectors for working copy changes, commits, branch comparisons, and externally supplied patches through an adapter interface.

**Rationale:** Animus should support Jujutsu, Git, hosted diffs, and patch files without hard-coding one VCS.

### 2. Review mode is read-only by contract

**Choice:** Review runs use a policy profile that forbids mutations and denies approval escalation for write-capable tools unless an explicit follow-up run changes mode.

**Rationale:** Users should be able to request review without risking accidental edits.

### 3. Findings are structured

**Choice:** Review output uses typed findings with severity, category, affected span, evidence, rationale, confidence, and suggested remediation.

**Rationale:** Structured findings support deduplication, triage, follow-up, and machine-readable reports.

### 4. Evidence is bounded and source-attributed

**Choice:** Findings carry file/span or patch-hunk references and bounded evidence snippets, never unbounded logs or full files by default.

**Rationale:** Review reports should be actionable without overwhelming context or leaking unrelated data.

### 5. Reruns track finding state

**Choice:** Review reports can compare against prior review findings to classify findings as open, fixed, dismissed, or new.

**Rationale:** Rerun review should help users confirm fixes instead of re-reading the entire report.

## Risks / Trade-offs

- Read-only constraints may prevent automated quick fixes. That is intentional; fixes should happen in a separate coding run.
- VCS-neutral selectors require adapters to normalize spans. Span validation must reject impossible or stale locations.
- Review findings can be noisy. Severity, confidence, and deduplication reduce noise while preserving evidence.
