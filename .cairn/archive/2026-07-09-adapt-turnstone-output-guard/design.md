# Design: Turnstone-inspired output guard contracts

## Context

Animus tool responses already require bounded output and safe references before runtime projection. Turnstone adds a useful operational split: assess tool results after execution but before they enter model context, surface warnings, redact credentials, and preserve deterministic findings even when an optional LLM reviewer disagrees.

## Decisions

### 1. Output assessment is a pure core over bounded text summaries

**Choice:** The reusable core accepts already-bounded output snippets, safe output references, output policy, and detector policy. It returns an `OutputAssessment` with risk labels, flags, redaction state, evidence hashes, and projection advice.

**Rationale:** Reusable crates must not read files, fetch URLs, execute tools, call classifiers, or persist output. Hosts own those effects.

### 2. Deterministic findings cannot be lowered

**Choice:** Optional classifier or judge output may add flags or annotate disagreement, but deterministic credential, injection, encoded-payload, adversarial-URL, and unsafe-reference findings remain visible and binding for redaction/projection policy.

**Rationale:** A model that reads adversarial output must not erase deterministic tripwires.

### 3. Redaction happens before model-visible projection

**Choice:** Credential and secret-like findings force redaction or denial of prompt influence before a `ToolResponse` becomes model-visible.

**Rationale:** The model should never see credentials merely because the tool printed them.

### 4. Receipts record metadata only

**Choice:** Runtime events and receipts store flags, risk class, counts, redaction marker, detector tier labels, output hashes, and safe references. They reject raw tool output and unsafe handles.

**Rationale:** Assessment evidence is useful for calibration, but raw outputs remain high-risk data.

## Risks / Trade-offs

- Deterministic scanners can be noisy. The contract should return warning/review outcomes separately from hard denial.
- Redaction can remove debugging context. Receipts should include stable hashes and counts so hosts can correlate with private artifacts.
- Classifier annotations are useful for operators but unsafe as authority. They remain advisory and non-granting.