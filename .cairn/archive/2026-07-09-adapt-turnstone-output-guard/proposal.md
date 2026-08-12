# Proposal: Adapt Turnstone output guard contracts

## Why

Turnstone's output guard is a strong reference for keeping untrusted tool output from becoming model-visible control text. Animus already bounds tool responses and safe receipts, but it needs a focused output-assessment contract that detects prompt-injection markers, credential leakage, encoded payloads, adversarial URLs, and unsafe references before output is replayed into the loop.

This change adapts the pattern as a pure reusable contract. Deterministic findings can warn, redact, deny prompt influence, or require confirmation according to host policy, but optional classifier output cannot lower deterministic findings or grant authority.

## What Changes

- Add typed output-assessment DTOs for risk level, flags, annotations, redaction state, bounded counts, detector tiers, and safe evidence hashes.
- Add a deterministic scanner core for prompt injection, credentials, encoded payloads, adversarial URLs, and unsafe host references.
- Define merge semantics where deterministic findings are preserved and optional classifier findings may only add or annotate risk.
- Gate model-visible tool-output projection on assessment policy, redaction policy, and safe output bounds.
- Produce safe receipts and runtime events that omit raw tool output, credentials, private paths, provider payloads, judge rationales, and unbounded traces.

## Impact

- **Files**: adapter DTOs, runtime output-projection checks, scanner fixtures, receipt validation, docs, and focused tests.
- **Testing**: positive clean-output and redacted-credential cases plus negative prompt-injection, encoded-payload, adversarial-URL, unsafe-reference, classifier-downgrade, over-budget, and receipt-leak cases.
- **Non-claims**: this does not add a Turnstone dependency, product UI warning chips, live classifier calls, web fetching, shell execution, or storage of raw tool output.